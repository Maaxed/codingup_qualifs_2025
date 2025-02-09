use codingup_qualifs::io::{read_input, write_output, Input};
use codingup_qualifs::{resolve, resolve_fast, Action, ActionKind};

fn check_actions_at(input: &Input, actions: &[Action], index: usize) -> bool
{
	let mut seed = input.seed_capacity as i32;
	for action in &actions[index..]
	{
		if action.kind == ActionKind::Collect
		{
			break;
		}
		seed -= 1;
	}

	for action in actions[0..index].iter().rev()
	{
		if action.kind == ActionKind::Collect
		{
			break;
		}
		seed -= 1;
	}

	seed >= 0
}

fn check_actions(input: &Input, actions: &[Action], slice_start: usize, slice_end: usize, d: usize) -> bool
{
	   check_actions_at(input, actions, slice_start)
	&& check_actions_at(input, actions, slice_end)
	&& check_actions_at(input, actions, slice_start + d)
}


fn splice_optim(input: &Input, actions: &mut [Action], max_size: usize)
{
	let (plant_count, distance_traveled) = resolve_fast(input, actions, true);
	let mut value = (plant_count, -distance_traveled);
	println!("Action count {}", actions.len());
	println!("Base value {value:?}");
	
	loop
	{
		let mut res = false;
		for slice_start in 0..actions.len()-2
		{
			for slice_end in slice_start+2..actions.len().min(slice_start+2+max_size)
			{
				let delta = slice_end - slice_start;
				let mut best_d = 0;
				for d in 1..delta
				{
					actions[slice_start..slice_end].rotate_right(1);

					if !check_actions(input, actions, slice_start, slice_end, d)
					{
						continue;
					}

					let (plant_count, distance_traveled) = resolve_fast(input, actions, true);
					let new_value = (plant_count, -distance_traveled);

					if new_value > value
					{
						value = new_value;
						best_d = d;
						println!("Optim found {new_value:?}");
						res = true;
					}
				}

				actions[slice_start..slice_end].rotate_right(1 + best_d);
			}
		}

		if !res
		{
			break;
		}
	}
}









use std::cmp::Ordering;
use std::time::{Duration, Instant};

use codingup_qualifs::{distance, io::*};
use hashbrown::{Equivalent, HashMap};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	seeds: Vec<[i32;2]>,
	plants: Vec<[i32;2]>,
}

#[derive(Debug, Copy, Clone)]
pub struct MyAction
{
	pub pos: [i32; 2],
	pub index: usize,
	pub kind: OutAction,
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
		dbg!(max_cost);
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
		for index in 0..state.plants.len()
		{
			if !force_compute && start.elapsed() >= time_limit
			{
				state.robot_pos = pos;
				state.seed_storage += 1;
				return None;
			}

			let plant = state.plants[index];
			let dist = distance(pos, plant);
			
			let cost = (dist - input.range).max(0);
			
			if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				continue;
			}

			state.plants.remove(index);

			if dist <= input.range
			{
				// No move required

				let mut cost = cost;
				state.robot_pos = pos;

				match find_best_action(input, memo, state, min_cost, start, time_limit, depth-1, force_compute)
				{
					None =>
					{
						state.plants.insert(index, plant);
						state.seed_storage += 1;
						return None;
					},
					Some(Res::SolutionFound { cost: child_cost, .. }) =>
					{
						cost += child_cost;
					},
					Some(Res::Solved) =>
					{ },
					Some(Res::NoSolution) =>
					{
						state.plants.insert(index, plant);
						continue;
					}
				}

				if cost < min_cost
				{
					min_cost = cost;
					min_action = Some(MyAction { pos, index, kind: OutAction::Plant(plant) });
				}
			}
			else
			{
				let delta = [plant[0] - pos[0], plant[1] - pos[1]];

				// Move is required
				let sign = [delta[0].signum(), delta[1].signum()];
				for dx in i32::max(0, input.range - delta[1].abs())..=i32::min(delta[0].abs(), input.range)
				{
					if !force_compute && start.elapsed() >= time_limit
					{
						state.plants.insert(index, plant);
						state.robot_pos = pos;
						state.seed_storage += 1;
						return None;
					}
					let dy = input.range - dx;

					let new_pos = [plant[0] - sign[0] * dx, plant[1] - sign[1] * dy];
					state.robot_pos = new_pos;
					
					let mut cost = cost;

					match find_best_action(input, memo, state, min_cost - cost, start, time_limit, depth-1, force_compute)
					{
						None =>
						{
							state.plants.insert(index, plant);
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
						min_action = Some(MyAction { pos: new_pos, index, kind: OutAction::Plant(plant) });
					}
				}
			}

			state.plants.insert(index, plant);
		}
		state.robot_pos = pos;
		state.seed_storage += 1;
	}

	if state.seed_storage < input.seed_capacity
	{
		for index in 0..state.seeds.len()
		{
			if !force_compute && start.elapsed() >= time_limit
			{
				return None;
			}

			let seed = state.seeds[index];
			let dist = distance(pos, seed);

			let mut cost = dist;

			if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				continue;
			}

			let  old_seed_storage = state.seed_storage;
			state.seed_storage = input.seed_capacity;
			state.robot_pos = seed;
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
				min_action = Some(MyAction { pos: seed, index, kind: OutAction::Collect });
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









fn main()
{
	let time_limit: f32 = std::env::args().nth(2).unwrap().parse().unwrap();
	let input = read_input().unwrap();

	let time_per_action = Duration::from_secs_f32(time_limit / input.plants.len() as f32);
	dbg!(time_per_action);

	let mut state = State
	{
		robot_pos: [0, 0],
		seed_storage: input.seed_capacity,
		seeds: input.seeds.clone(),
		plants: input.plants.clone(),
	};

	let mut distance_traveled = 0;

	let mut moves = Vec::new();
	let mut memo = HashMap::new();

	while !state.plants.is_empty()
	{
		let max_dist = input.max_distance as i32 - distance_traveled+1;
		let Res::SolutionFound { cost, action } = find_best_action_time_limit(&input, &mut memo, &mut state, max_dist, time_per_action)
		else
		{
			break;
		};

		if state.plants.len() % 10 == 0
		{
			println!("End step {} {} {} {}", state.plants.len(), max_dist, cost, memo.len());
		}

		if state.robot_pos != action.pos
		{
			distance_traveled += distance(state.robot_pos, action.pos);
			state.robot_pos = action.pos;
		}

		match action.kind
		{
			OutAction::Plant(pos) =>
			{
				moves.push(Action
				{
					pos,
					kind: ActionKind::Plant,
				});
				state.seed_storage -= 1;
				state.plants.remove(action.index);
			},
			OutAction::Collect =>
			{
				moves.push(Action
				{
					pos: action.pos,
					kind: ActionKind::Collect,
				});

				state.seed_storage = input.seed_capacity;
				state.seeds.remove(action.index);
			},
			OutAction::Move(_) =>
			{ },
		}
	}




	for seed in state.seeds
	{
		moves.push(Action
		{
			pos: seed,
			kind: ActionKind::Collect
		});
	}

	let max_size = std::env::args().nth(3).unwrap().parse().unwrap();
	splice_optim(&input, &mut moves, max_size);

	let (mut out_actions, plant_count, distance_traveled) = resolve(&input, &moves);

	write_output(out_actions.make_contiguous(), plant_count, distance_traveled);
}
