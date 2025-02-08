use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use codingup_qualifs::io::*;
use codingup_qualifs::dijkstra::WeightedNode;

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

	let mut processed = HashSet::new();

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
		if !processed.insert(state.clone())
		{
			continue;
		}

		prev_move.insert(state.clone(), back);

		if processed.len() % 20000 == 0
		{
			println!("{} {} {}", priority_queue.len(), processed.len(), distance_traveled);
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
			let remaining_distance = input.max_distance as i32 - distance_traveled;
			
			for (plant_index, plant) in state.plants.iter().enumerate()
			{
				let delta = [plant[0] - pos[0], plant[1] - pos[1]];
				let abs = [delta[0].abs(), delta[1].abs()];
				let dist = abs[0] + abs[1];
				if dist <= input.range + remaining_distance
				{
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

					if dist <= input.range
					{
						// No move required
						priority_queue.push(WeightedNode(distance_traveled, (
							Some(BackAction
							{
								old_state: state.clone(),
								action: OutAction::Plant(*plant),
							}),
							State
							{
								robot_pos: pos,
								seed_storage: state.seed_storage-1,
								seeds: state.seeds.clone(),
								plants,
							},
						)));
					}
					else
					{
						// Move is required
						let sign = [delta[0].signum(), delta[1].signum()];
						for dx in i32::max(0, input.range - abs[1])..=i32::min(abs[0], input.range)
						{
							let dy = input.range - dx;

							let new_pos = [plant[0] - sign[0] * dx, plant[1] - sign[1] * dy];
		
							priority_queue.push(WeightedNode(distance_traveled + dist - input.range, (
								Some(BackAction
								{
									old_state: state.clone(),
									action: OutAction::Plant(*plant),
								}),
								State
								{
									robot_pos: new_pos,
									seed_storage: state.seed_storage-1,
									seeds: state.seeds.clone(),
									plants: plants.clone(),
								},
							)));
						}
					}
				}
			}
		}

		if state.seed_storage < input.seed_capacity
		{
			for (seed_index, &seed) in state.seeds.iter().enumerate()
			{
				let mut distance_traveled = distance_traveled;
				distance_traveled += (seed[0] - pos[0]).abs() + (seed[1] - pos[1]).abs();

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