use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use codingup_qualifs::io::*;
use codingup_qualifs::dijkstra::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	seeds: Rc<[[i32;2]]>, // Use rc instead of vec to avoid cloning the list as much as possible
	plants: Rc<[[i32;2]]>,
}

#[derive(Debug, Clone)]
struct BackAction
{
	old_state: State,
	action: OutAction,
}

fn main() -> serde_json::Result<()>
{
	let input = read_input()?;

	// We assume that there is a solution that plants all seeds
	// Use Dijkstra's algorithm

	let mut priority_queue = BinaryHeap::new();

	let mut seed_set: HashSet<Rc<[[i32;2]]>> = HashSet::new();
	let mut plant_set: HashSet<Rc<[[i32;2]]>> = HashSet::new();

	let all_seeds: Rc<[[i32;2]]> = input.seeds.into();
	let all_plants: Rc<[[i32;2]]> = input.plants.into();

	seed_set.insert(all_seeds.clone());
	plant_set.insert(all_plants.clone());

	let initial_state = State
	{
		robot_pos: [0, 0],
		seed_storage: input.seed_capacity,
		seeds: all_seeds,
		plants: all_plants,
	};

	priority_queue.push(WeightedNode(0, (None, initial_state)));

	let mut prev_move: HashMap<State, Option<BackAction>> = HashMap::new();

	while let Some(WeightedNode(distance_traveled, (back, state))) = priority_queue.pop()
	{
		if prev_move.contains_key(&state)
		{
			continue;
		}

		prev_move.insert(state.clone(), back);

		if prev_move.len() % 20000 == 0
		{
			println!("{} {} {}", priority_queue.len(), prev_move.len(), distance_traveled);
		}

		if state.plants.is_empty()
		{
			let mut moves = VecDeque::new();

			let mut state = &state;

			let mut back = &prev_move[state];
			while let Some(b) = back
			{
				moves.push_front(b.action);
				if state.robot_pos != b.old_state.robot_pos
				{
					moves.push_front(OutAction::Move(state.robot_pos));
				}

				state = &b.old_state;
				back = &prev_move[state];
			}

			write_output(moves.make_contiguous());
			return Ok(())
		}

		let pos = state.robot_pos;

		if state.seed_storage > 0
		{
			for (plant_index, &plant) in state.plants.iter().enumerate()
			{
				let delta = [plant[0] - pos[0], plant[1] - pos[1]];
				let dist = delta[0].abs() + delta[1].abs();

				let mut plants = state.plants.to_vec();
				plants.remove(plant_index);

				let plants = if let Some(plants) = plant_set.get(&plants[..])
				{
					plants.clone()
				}
				else
				{
					let plants: Rc<[[i32; 2]]> = plants.into();
					plant_set.insert(plants.clone());
					plants
				};

				priority_queue.push(WeightedNode(distance_traveled + dist, (
					Some(BackAction
					{
						old_state: state.clone(),
						action: OutAction::Plant(plant),
					}),
					State
					{
						robot_pos: plant,
						seed_storage: state.seed_storage-1,
						seeds: state.seeds.clone(),
						plants,
					},
				)));
			}
		}

		if state.seed_storage < input.seed_capacity
		{
			for (seed_index, &seed) in state.seeds.iter().enumerate()
			{
				let delta = [seed[0] - pos[0], seed[1] - pos[1]];
				let dist = delta[0].abs() + delta[1].abs();
				let distance_traveled = distance_traveled + dist;

				let mut seeds = state.seeds.to_vec();
				seeds.remove(seed_index);

				let seeds = if let Some(seeds) = seed_set.get(&seeds[..])
				{
					seeds.clone()
				}
				else
				{
					let seeds: Rc<[[i32; 2]]> = seeds.into();
					seed_set.insert(seeds.clone());
					seeds
				};

				priority_queue.push(WeightedNode(distance_traveled, (
					Some(BackAction
					{
						old_state: state.clone(),
						action: OutAction::Collect,
					}),
					State
					{
						robot_pos: seed,
						seed_storage: input.seed_capacity,
						seeds,
						plants: state.plants.clone(),
					},
				)));
			}
		}
	}

	println!("No solution found!");

	Ok(())
}