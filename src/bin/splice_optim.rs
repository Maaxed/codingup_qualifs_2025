use codingup_qualifs::io::{read_input, read_output, write_output, Input};
use codingup_qualifs::{resolve, resolve_fast, unresolve, Action, ActionKind};


fn check_actions(input: &Input, actions: &[Action]) -> bool
{
	let mut seed = input.seed_capacity;
	for action in actions
	{
		match action.kind
		{
			ActionKind::Collect => seed = input.seed_capacity,
			ActionKind::Plant =>
			{
				if seed == 0
				{
					return false;
				}

				seed -= 1;
			},
		}
	}

	true
}


fn splice_optim(input: &Input, actions: &mut [Action]) -> bool
{
	let (plant_count, distance_traveled) = resolve_fast(input, actions, true);
	
	let mut value = (plant_count, -distance_traveled);

	let mut res = false;
	for slice_start in 0..actions.len()-2
	{
		for slice_end in slice_start+2..actions.len()
		{
			let delta = slice_end - slice_start;
			for d in 1..delta
			{
				actions[slice_start..slice_end].rotate_right(d);

				if !check_actions(input, actions)
				{
					actions[slice_start..slice_end].rotate_left(d);
					continue;
				}

				let (plant_count, distance_traveled) = resolve_fast(input, actions, true);
				let new_value = (plant_count, -distance_traveled);

				if new_value > value
				{
					value = new_value;
					println!("Optim found {new_value:?}");
					res = true;
				}
				else
				{
					actions[slice_start..slice_end].rotate_left(d);
				}
			}
		}
	}

	res
}

fn max_optim(input: &Input, actions: &mut[Action])
{
	let (plant_count, distance_traveled) = resolve_fast(input, actions, true);
	let value = (plant_count, -distance_traveled);
	println!("Base value {value:?}");

	loop
	{
		if !splice_optim(input, actions)
		{
			break;
		}
	}
}


fn main()
{
	let input = read_input().unwrap();
	let output = read_output();
	let mut actions = unresolve(&output);
	
	max_optim(&input, &mut actions);

	let (mut out_actions, plant_count, distance_traveled) = resolve(&input, &actions);

	write_output(out_actions.make_contiguous(), plant_count, distance_traveled);
}
