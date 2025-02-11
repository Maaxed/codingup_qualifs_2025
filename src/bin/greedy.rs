use codingup_qualifs::io::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	remaining_distance: u32,
	seeds: Vec<[i32;2]>,
	plants: Vec<[i32;2]>,
}

fn main() -> serde_json::Result<()>
{
	let input = read_input()?;

	let plant_count = input.plants.len();

	let mut state = State
	{
		robot_pos: [0, 0],
		seed_storage: input.seed_capacity,
		remaining_distance: input.max_distance,
		seeds: input.seeds,
		plants: input.plants,
	};

	let mut moves = Vec::new();

	while !state.plants.is_empty()
	{
		let pos = state.robot_pos;

		if state.seed_storage > 0
		{
			let (min_plant_index, min_plant_pos) = state.plants.iter().enumerate().min_by_key(|(_, plant)|
			{
				let delta = [plant[0] - pos[0], plant[1] - pos[1]];
				delta[0].abs() + delta[1].abs()
			}).unwrap();

			let min_plant_pos = *min_plant_pos;

			let delta = [min_plant_pos[0] - pos[0], min_plant_pos[1] - pos[1]];
			let dist = (delta[0].abs() + delta[1].abs()) as u32;

			if dist > state.remaining_distance
			{
				println!("Out of energy!");
				break;
			}

			if min_plant_pos != state.robot_pos
			{
				moves.push(OutAction::Move(min_plant_pos));
			}
			moves.push(OutAction::Plant(min_plant_pos));

			state.robot_pos = min_plant_pos;
			state.seed_storage -= 1;
			state.plants.remove(min_plant_index);
			state.remaining_distance -= dist;
		}
		else
		{
			let (min_seed_index, min_seed_pos) = state.seeds.iter().enumerate().min_by_key(|(_, seed)|
			{
				let delta = [seed[0] - pos[0], seed[1] - pos[1]];
				delta[0].abs() + delta[1].abs()
			}).unwrap();

			let min_seed_pos = *min_seed_pos;

			let delta = [min_seed_pos[0] - pos[0], min_seed_pos[1] - pos[1]];
			let dist = (delta[0].abs() + delta[1].abs()) as u32;

			if dist > state.remaining_distance
			{
				println!("Out of energy!");
				break;
			}

			if min_seed_pos != state.robot_pos
			{
				moves.push(OutAction::Move(min_seed_pos));
			}
			moves.push(OutAction::Collect);

			state.robot_pos = min_seed_pos;
			state.seed_storage = input.seed_capacity;
			state.seeds.remove(min_seed_index);
			state.remaining_distance -= dist;
		}
	}

	write_output(&moves, None, plant_count, (input.max_distance - state.remaining_distance) as i32);

	Ok(())
}
