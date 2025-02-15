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
	let (plant_count, distance_traveled) = resolve_q_fast(input, actions, false);
	let mut value = (plant_count, -distance_traveled);
	println!("Action count {}", actions.len());
	println!("Base value {value:?}");
	
	loop
	{
		// splice optim
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

					let (plant_count, distance_traveled) = resolve_q_fast(input, actions, false);

					//assert_eq!(plant_count, plant_count1);
					//assert_eq!(distance_traveled, distance_traveled1);

					let new_value = (plant_count, -distance_traveled);

					if new_value > value
					{
						value = new_value;
						best_d = d;
						println!("Splice optim found {new_value:?} {delta} {d}");
						res = true;
					}
				}

				actions[slice_start..slice_end].rotate_right(1 + best_d);
			}
		}
		
		// swap optim

		for a in [0, 1]
		{
			for center in 1..actions.len()-a
			{
				let max_d = usize::min(max_size, usize::min(center, actions.len() - center - a));
				let mut best_d = 0;
				for d in 1..=max_d
				{
					actions.swap(center - d, center + d - 1 + a);

					if !check_actions_at(input, actions, center - d) || !check_actions_at(input, actions, center + d - 1 + a)
					{
						continue;
					}

					let (plant_count, distance_traveled) = resolve_q_fast(input, actions, false);

					let new_value = (plant_count, -distance_traveled);

					if new_value > value
					{
						value = new_value;
						best_d = d;
						println!("Swap optim found {new_value:?} {d}");
						res = true;
					}
				}
				
				let mut best_d2 = max_d+1;

				for d2 in (best_d+1)..=max_d
				{
					actions.swap(center - d2, center + d2 - 1 + a);

					if !check_actions_at(input, actions, center - d2) || !check_actions_at(input, actions, center + d2 - 1 + a)
					 || !check_actions_at(input, actions, center - max_d) || !check_actions_at(input, actions, center + max_d - 1 + a)
					{
						continue;
					}

					let (plant_count, distance_traveled) = resolve_q_fast(input, actions, false);

					let new_value = (plant_count, -distance_traveled);

					if new_value > value
					{
						value = new_value;
						best_d2 = d2;
						println!("Swap optim found {new_value:?} {best_d} {d2}");
						res = true;
					}
				}

				for d3 in (best_d2+1)..=max_d
				{
					actions.swap(center - d3, center + d3 - 1 + a);
				}
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
