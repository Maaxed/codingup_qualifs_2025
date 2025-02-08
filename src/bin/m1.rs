use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use codingup_qualifs::io::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	seeds: Rc<[[i32;2]]>, // Use rc instead of vec to avoid cloning the list as much as possible
	plants: Rc<[[i32;2]]>,
}

#[derive(Debug, Clone)]
struct StateAndMoves
{
	state: State,
	remaining_distance: u32,
	moves: Vec<Action>,
}

fn main() -> serde_json::Result<()>
{
	let input = read_input()?;

	let mut min_pos = [i32::MAX; 2];

	let mut max_pos = [i32::MIN; 2];

	for pos in input.seeds.iter().chain(input.plants.iter())
	{
		min_pos[0] = i32::min(min_pos[0], pos[0]);
		min_pos[1] = i32::min(min_pos[1], pos[1]);
		
		max_pos[0] = i32::max(max_pos[0], pos[0]);
		max_pos[1] = i32::max(max_pos[1], pos[1]);
	}

	// We assume that there is a solution that plants all seeds
	// Use a simple tree search

	let mut queue = VecDeque::new();

	let mut processed = HashSet::new();

	let mut initial_state = StateAndMoves
	{
		state: State
		{
			robot_pos: [0, 0],
			seed_storage: input.seed_capacity,
			seeds: input.seeds.into(),
			plants: input.plants.into(),
		},
		remaining_distance: input.max_distance,
		moves: Vec::new(),
	};
		

	if min_pos[0] > input.range
	{
		initial_state.state.robot_pos[0] = min_pos[0];
		initial_state.remaining_distance -= min_pos[0] as u32;
	}

	if min_pos[1] > input.range
	{
		initial_state.state.robot_pos[1] = min_pos[1];
		initial_state.remaining_distance -= min_pos[0] as u32;
	}

	if min_pos[0] > input.range || min_pos[1] > input.range
	{
		initial_state.moves.push(Action::Move(initial_state.state.robot_pos));
	}

	dbg![min_pos, max_pos];

	queue.push_back(initial_state);

	while let Some(StateAndMoves { state, remaining_distance, moves }) = queue.pop_front()
	{
		if !processed.insert(state.clone())
		{
			continue;
		}

		if processed.len() % 20000 == 0
		{
			println!("{} {} {}", queue.len(), processed.len(), remaining_distance);
		}

		if state.seed_storage > 0
		{
			for (plant_index, plant) in state.plants.iter().enumerate()
			{
				let delta = [plant[0] - state.robot_pos[0], plant[1] - state.robot_pos[1]];
				let dist = delta[0].abs() + delta[1].abs();
				if dist <= input.range
				{
					let mut plants = state.plants.to_vec();
					plants.remove(plant_index);

					let mut moves = moves.clone();
					moves.push(Action::Plant(*plant));

					queue.push_front(StateAndMoves
					{
						state: State // cost 0 -> push front
						{
							robot_pos: state.robot_pos,
							seed_storage: state.seed_storage-1,
							seeds: state.seeds.clone(),
							plants: plants.into(),
						},
						remaining_distance,
						moves,
					});
				}
			}
		}

		if state.plants.is_empty()
		{
			write_output(&moves);
			return Ok(())
		}

		if state.seed_storage < input.seed_capacity
		{
			for (seed_index, seed) in state.seeds.iter().enumerate()
			{
				if *seed == state.robot_pos
				{
					let mut seeds = state.seeds.to_vec();
					seeds.remove(seed_index);

					let mut moves = moves.clone();
					moves.push(Action::Collect);

					queue.push_front(StateAndMoves
					{
						state: State // cost 0 -> push front
						{
							robot_pos: state.robot_pos,
							seed_storage: input.seed_capacity,
							seeds: seeds.into(),
							plants: state.plants.clone(),
						},
						remaining_distance,
						moves,
					});
					break;
				}
			}
		}

		if remaining_distance == 0
		{
			continue;
		}

		let pos = state.robot_pos;

		for (delta, cond) in [
			([-1, 0], pos[0] > min_pos[0]),
			([ 1, 0], pos[0] < max_pos[0]),
			([0, -1], pos[1] > min_pos[1]),
			([0,  1], pos[1] < max_pos[1]),
		]
		{
			if !cond
			{
				continue;
			}

			let new_pos = [pos[0] + delta[0], pos[1] + delta[1]];

			let mut moves = moves.clone();
			moves.push(Action::Move(new_pos));

			queue.push_back(StateAndMoves
			{
				state: State // cost 1 -> push back
				{
					robot_pos: new_pos,
					seed_storage: state.seed_storage,
					seeds: state.seeds.clone(),
					plants: state.plants.clone(),
				},
				remaining_distance: remaining_distance-1,
				moves,
			});
		}
	}

	println!("No solution found!");

	Ok(())
}