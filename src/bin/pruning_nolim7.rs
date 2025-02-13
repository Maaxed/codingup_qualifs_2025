use std::cmp::Ordering;
use std::collections::HashMap;

use codingup_qualifs::prim::prim2;
use codingup_qualifs::quantum::QPos;
use codingup_qualifs::{io::*, solve_and_write_output, Action, ActionKind};

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

fn find_best_action(input: &Input, memo: &mut HashMap<State, (i32, Res)>, state: &mut State, max_cost: i32, depth: u32) -> Res
{
	if state.plants.is_empty()
	{
		return Res::Solved;
	}

	if let Some((ref_max_cost, res)) = memo.get(state)
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

	let pos = state.robot_pos;

	let mut min_cost = max_cost;
	let mut min_action = None;

	if state.seed_storage > 0
	{
		state.seed_storage -= 1;

		let mut plants: Vec<(usize, i32)> = (0..state.plants.len())
			.filter_map(|index|
			{
				let plant = state.plants[index];
				let (new_pos, dist) = pos.apply_plant(input, plant);

				if dist >= min_cost
				{
					return None;
				}

				state.plants.remove(index);
				let prim = prim2(input, new_pos, &state.plants) + dist;
				state.plants.insert(index, plant);

				if prim >= min_cost
				{
					return None;
				}

				Some((index, prim))
			})
			.collect();

		plants.sort_unstable_by_key(|(_index, prim)|
		{
			-*prim
		});

		for (index, prim) in plants
		{
			let plant = state.plants[index];
			let (new_pos, dist) = pos.apply_plant(input, plant);
			
			let mut cost = dist;
			
			if prim >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				continue;
			}

			state.robot_pos = new_pos;
			state.plants.remove(index);

			let res = find_best_action(input, memo, state, min_cost - cost, depth+1);
			
			state.plants.insert(index, plant);

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
				min_action = Some(MyAction { index, action: Action { pos: plant, kind: ActionKind::Plant } });
			}
		}
		state.robot_pos = pos;
		state.seed_storage += 1;
	}

	if state.seed_storage < input.seed_capacity
	{
		let mut seeds: Vec<(usize, i32)> = (0..state.seeds.len())
			.filter_map(|index|
			{
				let seed = state.seeds[index];
				let (new_pos, dist) = pos.apply_seed(seed);

				if dist >= min_cost
				{
					return None;
				}

				let prim = prim2(input, new_pos, &state.plants) + dist;

				if prim >= min_cost
				{
					return None;
				}

				Some((index, prim))
			})
			.collect();
		
		seeds.sort_unstable_by_key(|(_index, prim)|
		{
			-*prim
		});

		for (index, prim) in seeds
		{
			let seed = state.seeds[index];
			let (new_pos, dist) = pos.apply_seed(seed);

			let mut cost = dist;

			if prim >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				continue;
			}

			let  old_seed_storage = state.seed_storage;
			state.seed_storage = input.seed_capacity;
			state.robot_pos = new_pos;
			state.seeds.remove(index);

			let res = find_best_action(input, memo, state, min_cost - cost, depth+1);
			
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

	if depth <= 3
	{
		//dbg!(depth, res, max_cost);
		dbg!(depth);
	}

	/*if memo.len() % 100000 == 0
	{
		dbg!(memo.len(), max_cost);
	}*/

	memo.insert(state.clone(), (max_cost, res));

	res
}

fn main() -> serde_json::Result<()>
{
	let input = read_input()?;

	let mut state = State
	{
		robot_pos: QPos::default(),
		seed_storage: input.seed_capacity,
		seeds: input.seeds.clone(),
		plants: input.plants.clone(),
	};

	let mut distance_traveled = 0;

	let mut moves = Vec::new();
	let mut memo = HashMap::new();

	let max_dist = 3270; // input.max_distance as i32

	while !state.plants.is_empty()
	{
		let Res::SolutionFound { action, .. } = find_best_action(&input, &mut memo, &mut state, max_dist - distance_traveled+1, 0)
		else
		{
			break;
		};

		let (new_pos, dist) = state.robot_pos.apply_action(&input, &action.action);

		distance_traveled += dist;
		state.robot_pos = new_pos;

		moves.push(action.action);

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

	dbg!(distance_traveled);

	solve_and_write_output(&input, &moves);

	Ok(())
}
