use codingup_qualifs::{distance, io::*, resolve, Action, ActionKind};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	seeds: Vec<[i32;2]>,
	plants: Vec<[i32;2]>,
}

fn find_best_action(input: &Input, state: &State, depth: u32) -> Option<(i32, usize, Action)>
{
	if state.plants.is_empty()
	{
		return None;
	}

	let pos = state.robot_pos;
	
	if state.seed_storage > 0
	{
		let (min_dist, min_plant_index, min_plant_pos) = if depth <= 1
		{
			state.plants.iter()
				.enumerate()
				.map(|(index, plant)| (distance(pos, *plant), index, plant))
				.min_by_key(|(dist, _, _)| *dist)
				.unwrap()
		}
		else
		{
			state.plants.iter()
				.enumerate()
				.map(|(index, plant)|
				{
					let mut new_state = state.clone();
					new_state.seed_storage -= 1;
					new_state.robot_pos = *plant;
					new_state.plants.remove(index);

					let dist = distance(pos, *plant);

					if let Some((min_dist, _, _)) = find_best_action(input, &new_state, depth-1)
					{
						(dist + min_dist, index, plant)
					}
					else
					{
						(dist, index, plant)
					}
				})
				.min_by_key(|(dist, _, _)| *dist)
				.unwrap()
		};

		Some((min_dist, min_plant_index, Action
		{
			pos: *min_plant_pos,
			kind: ActionKind::Plant
		}))
	}
	else
	{
		let (min_dist, min_seed_index, min_seed_pos) = if depth <= 1
		{
			state.seeds.iter()
				.enumerate()
				.map(|(index, seed)|
				{
					let delta = [seed[0] - pos[0], seed[1] - pos[1]];
					(delta[0].abs() + delta[1].abs(), index, seed)
				})
				.min_by_key(|(dist, _, _)| *dist)?
		}
		else
		{
			state.seeds.iter()
				.enumerate()
				.map(|(index, seed)|
				{
					let mut new_state = state.clone();
					new_state.seed_storage = input.seed_capacity;
					new_state.robot_pos = *seed;
					new_state.seeds.remove(index);

					let dist = distance(pos, *seed);

					if let Some((min_dist, _, _)) = find_best_action(input, &new_state, depth-1)
					{
						(dist + min_dist, index, seed)
					}
					else
					{
						(dist, index, seed)
					}
				})
				.min_by_key(|(dist, _, _)| *dist)?
		};

		Some((min_dist, min_seed_index, Action
		{
			pos: *min_seed_pos,
			kind: ActionKind::Collect
		}))
	}
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

	let mut moves = Vec::new();

	while !state.plants.is_empty()
	{
		if state.plants.len() % 100 == 0
		{
			dbg!(state.plants.len());
		}

		let Some((_dist, index, action)) = find_best_action(&input, &state, depth)
		else
		{
			break;
		};

		moves.push(action);
		state.robot_pos = action.pos;

		match action.kind
		{
			ActionKind::Plant =>
			{
				state.seed_storage -= 1;
				state.plants.remove(index);
			},
			ActionKind::Collect =>
			{

				state.seed_storage = input.seed_capacity;
				state.seeds.remove(index);
			}
		}
	}

	let (mut res, plant_count, distance_traveled) = resolve(&input, &moves);

	write_output(res.make_contiguous(), plant_count, distance_traveled);

	Ok(())
}
