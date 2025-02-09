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
	let (plant_count, distance_traveled) = resolve_fast(input, actions, false);
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

					let (plant_count, distance_traveled) = resolve_fast(input, actions, false);
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


fn main()
{
	let max_size = std::env::args().nth(2).unwrap().parse().unwrap();
	#[derive(Debug, Clone, Hash, PartialEq, Eq)]
	struct State
	{
		robot_pos: [i32; 2],
		seed_storage: u32,
		seeds: Vec<[i32;2]>,
		plants: Vec<[i32;2]>,
	}

	let input = read_input().unwrap();

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

	for seed in state.seeds
	{
		moves.push(Action
		{
			pos: seed,
			kind: ActionKind::Collect
		});
	}

	
	splice_optim(&input, &mut moves, max_size);

	let (mut out_actions, plant_count, distance_traveled) = resolve(&input, &moves);

	write_output(out_actions.make_contiguous(), plant_count, distance_traveled);
}
