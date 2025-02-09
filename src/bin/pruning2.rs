use codingup_qualifs::{distance, io::*, resolve, Action, ActionKind};

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
	pub kind: OutAction,
}

fn find_best_action(input: &Input, state: &State, max_cost: (i32, i32), depth: u32) -> Option<((i32, i32), usize, MyAction)>
{
	if state.plants.is_empty()
	{
		return None;
	}

	let pos = state.robot_pos;

	Some(if depth <= 1
	{
		if state.seed_storage > 0
		{
			let (min_dist, min_index, min_plant) = 
				state.plants.iter()
					.enumerate()
					.map(|(index, plant)| (distance(pos, *plant), index, plant))
					.min_by_key(|(dist, _, _)| *dist)
					.unwrap();
			


			((0, (min_dist - input.range).max(0)), min_index, MyAction { pos: *min_plant, kind: OutAction::Plant(*min_plant) })
		}
		else
		{
			let (min_dist, min_index, min_seed) = 
				state.seeds.iter()
					.enumerate()
					.map(|(index, seed)| (distance(pos, *seed), index, seed))
					.min_by_key(|(dist, _, _)| *dist)?;

			((1, min_dist), min_index, MyAction { pos: *min_seed, kind: OutAction::Collect })
		}
	}
	else
	{
		let mut min_cost = max_cost;
		let mut min_action = None;

		if state.seed_storage > 0
		{
			for (index, plant) in state.plants.iter().enumerate()
			{
				let dist = distance(pos, *plant);
				
				let cost = (0, (dist - input.range).max(0));
				
				if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
				{
					continue;
				}

				if dist <= input.range
				{
					// No move required
					let mut new_state = state.clone();
					new_state.seed_storage -= 1;
					new_state.plants.remove(index);

					let mut cost = cost;

					if let Some((child_cost, _, _)) = find_best_action(input, &new_state, min_cost, depth-1)
					{
						cost.0 += child_cost.0;
						cost.1 += child_cost.1;
					}

					if cost < min_cost
					{
						min_cost = cost;
						min_action = Some((index, MyAction { pos, kind: OutAction::Plant(*plant) }));
					}
				}
				else
				{
					let mut new_state = state.clone();
					new_state.seed_storage -= 1;
					new_state.plants.remove(index);
					
					let delta = [plant[0] - pos[0], plant[1] - pos[1]];

					// Move is required
					let sign = [delta[0].signum(), delta[1].signum()];
					for dx in i32::max(0, input.range - delta[1].abs())..=i32::min(delta[0].abs(), input.range)
					{
						let dy = input.range - dx;

						let new_pos = [plant[0] - sign[0] * dx, plant[1] - sign[1] * dy];
						new_state.robot_pos = new_pos;
						
						let mut cost = cost;
	
						if let Some((child_cost, _, _)) = find_best_action(input, &new_state, (min_cost.0, min_cost.1 - cost.1), depth-1)
						{
							cost.0 += child_cost.0;
							cost.1 += child_cost.1;
						}

						if cost < min_cost
						{
							min_cost = cost;
							min_action = Some((index, MyAction { pos: new_pos, kind: OutAction::Plant(*plant) }));
						}
					}
				}
			}
		}

		if state.seed_storage < input.seed_capacity
		{
			for (index, seed) in state.seeds.iter().enumerate()
			{
				let dist = distance(pos, *seed);

				let mut cost = (1, dist);

				if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
				{
					continue;
				}

				let mut new_state = state.clone();
				new_state.seed_storage = input.seed_capacity;
				new_state.robot_pos = *seed;
				new_state.seeds.remove(index);

				if let Some((child_cost, _, _)) = find_best_action(input, &new_state, (min_cost.0 - 1, min_cost.1 - dist), depth-1)
				{
					cost.0 += child_cost.0;
					cost.1 += child_cost.1;
				}

				if cost < min_cost
				{
					min_cost = cost;
					min_action = Some((index, MyAction { pos: *seed, kind: OutAction::Collect }));
				}
			}
		}

		let (min_index, min_action) = min_action?;
		(min_cost, min_index, min_action)
	})
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

	while !state.plants.is_empty()
	{
		if state.plants.len() % 10 == 0
		{
			dbg!(state.plants.len());
		}

		let Some((_dist, index, action)) = find_best_action(&input, &state, (i32::MAX, input.max_distance as i32 - distance_traveled), depth)
		else
		{
			break;
		};

		distance_traveled += distance(state.robot_pos, action.pos);
		state.robot_pos = action.pos;

		match action.kind
		{
			OutAction::Plant(plant_pos) =>
			{
				moves.push(Action
				{
					pos: plant_pos,
					kind: ActionKind::Plant,
				});
				state.seed_storage -= 1;
				state.plants.remove(index);
			},
			OutAction::Collect =>
			{
				moves.push(Action
				{
					pos: action.pos,
					kind: ActionKind::Collect,
				});
				state.seed_storage = input.seed_capacity;
				state.seeds.remove(index);
			},
			OutAction::Move(_) =>
			{ },
		}
	}

	let (mut res, plant_count, distance_traveled) = resolve(&input, &moves);

	write_output(res.make_contiguous(), plant_count, distance_traveled);

	Ok(())
}
