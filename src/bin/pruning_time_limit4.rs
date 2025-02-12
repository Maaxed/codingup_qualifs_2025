use std::cmp::Ordering;
use std::time::{Duration, Instant};

use codingup_qualifs::quantum::QPos;
use codingup_qualifs::{io::*, solve_and_write_output, Action, ActionKind};
use hashbrown::{Equivalent, HashMap};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: QPos,
	seed_storage: u32,
	seeds: Vec<[i32;2]>,
	plants: Vec<[i32;2]>,
}

#[derive(Debug, Copy, Clone)]
pub struct MyAction
{
	pub index: usize,
	pub action: Action,
}

#[derive(Debug, Copy, Clone)]
pub enum Res
{
	Solved,
	SolutionFound
	{
		cost: i32,
		action: MyAction,
	},
	NoSolution,
}

#[derive(Debug, Hash)]
struct StateAndDepth<'l>
{
	state: &'l State,
	depth: u32,
}

impl Equivalent<(State,u32)> for StateAndDepth<'_>
{
	fn equivalent(&self, key: &(State,u32)) -> bool
	{
		self.state == &key.0 && self.depth == key.1
	}
}

fn find_best_action_time_limit(input: &Input, memo: &mut HashMap<(State, u32), (i32, Res)>, state: &mut State, max_cost: i32, time_limit: Duration) -> Res
{
	let start = Instant::now();

	let mut last_res = None;
	for i in 1..
	{
		let res = find_best_action(input, memo, state, max_cost, start, time_limit, i, i == 1);
		if let Some(res) = res
		{
			last_res = Some(res);
		}
		else
		{
			break;
		}
	}

	last_res.unwrap()
}

fn find_best_action(input: &Input, memo: &mut HashMap<(State, u32), (i32, Res)>, state: &mut State, max_cost: i32, start: Instant, time_limit: Duration, depth: u32, force_compute: bool) -> Option<Res>
{
	if state.plants.is_empty()
	{
		return Some(Res::Solved);
	}

	if depth == 0
	{
		return Some(Res::Solved);
	}

	if let Some((ref_max_cost, res)) = memo.get(&StateAndDepth { state, depth })
	{
		match ref_max_cost.cmp(&max_cost)
		{
			Ordering::Equal => return Some(*res),
			Ordering::Greater =>
			{
				return Some(if let Res::SolutionFound { cost, .. } = res
				{
					if *cost < max_cost
					{
						*res
					}
					else
					{
						Res::NoSolution
					}
				}
				else
				{
					*res
				});
			},
			Ordering::Less =>
			{
				if let Res::SolutionFound { .. } = res
				{
					return Some(*res);
				}
			}
		}
	}


	let pos = state.robot_pos;

	let mut min_cost = max_cost;
	let mut min_action = None;

	if state.seed_storage > 0
	{
		state.seed_storage -= 1;
		
		let mut plants: Vec<usize> = (0..state.plants.len()).collect();
		plants.sort_unstable_by_key(|&index|
		{
			let plant = state.plants[index];
			let (_new_pos, dist) = pos.apply_plant(input, plant);
			dist
		});

		for index in plants
		{
			if !force_compute && start.elapsed() >= time_limit
			{
				state.robot_pos = pos;
				state.seed_storage += 1;
				return None;
			}

			let plant = state.plants[index];
			let (new_pos, dist) = pos.apply_plant(input, plant);
			
			let mut cost = dist;
			
			if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				break; // Since I sorted the list, I can break here
			}

			state.robot_pos = new_pos;
			state.plants.remove(index);

			let res = find_best_action(input, memo, state, min_cost - cost, start, time_limit, depth-1, force_compute);

			state.plants.insert(index, plant);

			match res
			{
				None =>
				{
					state.robot_pos = pos;
					state.seed_storage += 1;
					return None;
				},
				Some(Res::SolutionFound { cost: child_cost, .. }) =>
				{
					cost += child_cost;
				},
				Some(Res::Solved) =>
				{ },
				Some(Res::NoSolution) => continue,
			}

			if cost < min_cost
			{
				min_cost = cost;
				min_action = Some(MyAction { index, action: Action { pos: plant, kind: ActionKind::Plant } });
			}
		}
		state.robot_pos = pos;
		state.seed_storage += 1;
	}

	if state.seed_storage < input.seed_capacity
	{
		let mut seeds: Vec<usize> = (0..state.seeds.len()).collect();
		seeds.sort_unstable_by_key(|&index|
		{
			let seed = state.seeds[index];
			let (_new_pos, dist) = pos.apply_seed(seed);
			dist
		});

		for index in seeds
		{
			if !force_compute && start.elapsed() >= time_limit
			{
				return None;
			}

			let seed = state.seeds[index];
			let (new_pos, dist) = pos.apply_seed(seed);

			let mut cost = dist;

			if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				break;
			}

			let  old_seed_storage = state.seed_storage;
			state.seed_storage = input.seed_capacity;
			state.robot_pos = new_pos;
			state.seeds.remove(index);

			let res = find_best_action(input, memo, state, min_cost - cost, start, time_limit, depth, force_compute); // collecting a seed doesn't increase the depth
			
			state.seed_storage = old_seed_storage;
			state.robot_pos = pos;
			state.seeds.insert(index, seed);

			match res?
			{
				Res::SolutionFound { cost: child_cost, .. } =>
				{
					cost += child_cost;
				},
				Res::Solved =>
				{ },
				Res::NoSolution => continue,
			}

			if cost < min_cost
			{
				min_cost = cost;
				min_action = Some(MyAction { index, action: Action { pos: seed, kind: ActionKind::Collect } });
			}
		}
	}

	let res = if let Some(min_action) = min_action
	{
		Res::SolutionFound
		{
			cost: min_cost,
			action: min_action,
		}
	}
	else
	{
		Res::NoSolution
	};

	memo.insert((state.clone(), depth), (max_cost, res));

	Some(res)
}

fn main() -> serde_json::Result<()>
{
	let time_limit: f32 = std::env::args().nth(2).unwrap().parse().unwrap();
	let input = read_input()?;

	let time_per_action = Duration::from_secs_f32(time_limit / input.plants.len() as f32);
	dbg!(time_per_action);

	let mut state = State
	{
		robot_pos: QPos::default(),
		seed_storage: input.seed_capacity,
		seeds: input.seeds.clone(),
		plants: input.plants.clone(),
	};

	let mut distance_traveled = 0;

	let mut actions = Vec::new();
	let mut memo = HashMap::new();

	let mut lim = true;

	while !state.plants.is_empty()
	{
		let max_dist = if lim { input.max_distance as i32 - distance_traveled+1 } else { i32::MAX };
		let Res::SolutionFound { cost, action } = find_best_action_time_limit(&input, &mut memo, &mut state, max_dist, time_per_action)
		else
		{
			if lim
			{
				lim = false;
				continue;
			}
			else
			{
				break;
			}
		};

		if state.plants.len() % 10 == 0
		{
			println!("End step {} {} {} {}", state.plants.len(), max_dist, cost, memo.len());
		}
		
		let (new_pos, dist) = state.robot_pos.apply_action(&input, &action.action);
		
		distance_traveled += dist;
		state.robot_pos = new_pos;

		actions.push(action.action);

		match action.action.kind
		{
			ActionKind::Plant =>
			{
				state.seed_storage -= 1;
				state.plants.remove(action.index);
			},
			ActionKind::Collect =>
			{
				state.seed_storage = input.seed_capacity;
				state.seeds.remove(action.index);
			},
		}
	}

	//write_output(&moves, Some(&actions), input.plants.len() - state.plants.len(), distance_traveled);
	solve_and_write_output(&input, &actions);

	Ok(())
}
