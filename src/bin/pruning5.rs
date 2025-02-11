use std::cmp::Ordering;

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

fn find_best_action(input: &Input, memo: &mut HashMap<(State, u32), (i32, Res)>, state: &mut State, max_cost: i32, depth: u32) -> Res
{
	if state.plants.is_empty()
	{
		dbg!(max_cost);
		return Res::Solved;
	}

	if depth == 0
	{
		return Res::Solved;
	}

	if let Some((ref_max_cost, res)) = memo.get(&StateAndDepth { state, depth })
	{
		match ref_max_cost.cmp(&max_cost)
		{
			Ordering::Equal => return *res,
			Ordering::Greater =>
			{
				return if let Res::SolutionFound { cost, .. } = res
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
				};
			},
			Ordering::Less =>
			{
				if let Res::SolutionFound { .. } = res
				{
					return *res;
				}
			}
		}
	}

	/*if memo.len() % 10000 == 0
	{
		println!("{} {} {}", memo.len(), max_cost, depth);
	}*/

	let pos = state.robot_pos;

	let mut min_cost = max_cost;
	let mut min_action = None;

	if state.seed_storage > 0
	{
		state.seed_storage -= 1;
		for index in 0..state.plants.len()
		{
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

				match find_best_action(input, memo, state, min_cost, depth-1)
				{
					Res::SolutionFound { cost: child_cost, .. } =>
					{
						cost += child_cost;
					},
					Res::Solved =>
					{ },
					Res::NoSolution =>
					{
						state.plants.insert(index, plant);
						continue;
					},
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
				let min = i32::max(0, input.range - delta[1].abs());
				let max = i32::min(delta[0].abs(), input.range);
				let dx = (min + max) / 2;

				let dy = input.range - dx;

				let new_pos = [plant[0] - sign[0] * dx, plant[1] - sign[1] * dy];
				state.robot_pos = new_pos;
				
				let mut cost = cost;

				match find_best_action(input, memo, state, min_cost - cost, depth-1)
				{
					Res::SolutionFound { cost: child_cost, .. } =>
					{
						cost += child_cost;
					},
					Res::Solved =>
					{ },
					Res::NoSolution =>
					{
						state.plants.insert(index, plant);
						continue;
					},
				}

				if cost < min_cost
				{
					min_cost = cost;
					min_action = Some(MyAction { pos: new_pos, index, kind: OutAction::Plant(plant) });
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

			let res = find_best_action(input, memo, state, min_cost - cost, depth); // collecting a seed doesn't increase the depth
			
			state.seed_storage = old_seed_storage;
			state.robot_pos = pos;
			state.seeds.insert(index, seed);

			match res
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

	res
}

fn main() -> serde_json::Result<()>
{
	let depth: u32 = std::env::args().nth(2).unwrap().parse().unwrap();
	let input = read_input()?;

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
		let Res::SolutionFound { cost, action } = find_best_action(&input, &mut memo, &mut state, max_dist, depth)
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
			moves.push(OutAction::Move(state.robot_pos));
		}

		moves.push(action.kind);

		match action.kind
		{
			OutAction::Plant(_) =>
			{
				state.seed_storage -= 1;
				state.plants.remove(action.index);
			},
			OutAction::Collect =>
			{
				state.seed_storage = input.seed_capacity;
				state.seeds.remove(action.index);
			},
			OutAction::Move(_) =>
			{ },
		}
	}

	write_output(&moves, None, input.plants.len() - state.plants.len(), distance_traveled);

	println!("END");

	Ok(())
}
