use codingup_qualifs::io::{read_input, read_output, Input};
use codingup_qualifs::{resolve_q_fast, solve_and_write_output, Action, ActionKind};


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
	let (plant_count, distance_traveled) = resolve_q_fast(input, actions, true);
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

					let (plant_count, distance_traveled) = resolve_q_fast(input, actions, true);

					//assert_eq!(plant_count, plant_count1);
					//assert_eq!(distance_traveled, distance_traveled1);

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
	let input = read_input().unwrap();
	let mut actions = read_output();

	'plant: for plant in input.plants.iter()
	{
		for action in actions.iter()
		{
			if &action.pos == plant && action.kind == ActionKind::Plant
			{
				continue 'plant;
			}
		}

		panic!("Plant missing in meta");
	}

	'seed: for seed in input.seeds.iter()
	{
		for action in actions.iter()
		{
			if &action.pos == seed && action.kind == ActionKind::Collect
			{
				continue 'seed;
			}
		}

		actions.push(Action
		{
			pos: *seed,
			kind: ActionKind::Collect
		});
	}
	
	let max_size = std::env::args().nth(3).unwrap().parse().unwrap();
	splice_optim(&input, &mut actions, max_size);

	solve_and_write_output(&input, &actions);
}
