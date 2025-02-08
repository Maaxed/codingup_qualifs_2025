use codingup_qualifs::{io::*, resolve, Action, ActionKind};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	seeds: Vec<[i32;2]>,
	plants: Vec<[i32;2]>,
}

fn main() -> serde_json::Result<()>
{
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
		let pos = state.robot_pos;

		if state.seed_storage > 0
		{
			let (min_plant_index, min_plant_pos) = state.plants.iter().enumerate().min_by_key(|(_, plant)|
			{
				let delta = [plant[0] - pos[0], plant[1] - pos[1]];
				delta[0].abs() + delta[1].abs()
			}).unwrap();

			let min_plant_pos = *min_plant_pos;

			moves.push(Action
			{
				pos: min_plant_pos,
				kind: ActionKind::Plant
			});

			state.robot_pos = min_plant_pos;
			state.seed_storage -= 1;
			state.plants.remove(min_plant_index);
		}
		else
		{
			let (min_seed_index, min_seed_pos) = state.seeds.iter().enumerate().min_by_key(|(_, seed)|
			{
				let delta = [seed[0] - pos[0], seed[1] - pos[1]];
				delta[0].abs() + delta[1].abs()
			}).unwrap();

			let min_seed_pos = *min_seed_pos;

			moves.push(Action
			{
				pos: min_seed_pos,
				kind: ActionKind::Collect
			});

			state.robot_pos = min_seed_pos;
			state.seed_storage = input.seed_capacity;
			state.seeds.remove(min_seed_index);
		}
	}

	let (_dist, mut res) = resolve(&input, &moves);

	write_output(res.make_contiguous());

	Ok(())
}
