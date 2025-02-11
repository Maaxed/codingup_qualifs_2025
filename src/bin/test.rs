use codingup_qualifs::io::read_input;
use codingup_qualifs::{resolve_fast, resolve_fast2, resolve_q_fast, Action, ActionKind};
use rand::prelude::SliceRandom;
use rand::rng;


fn main()
{
	let input = read_input().unwrap();

	let mut actions = Vec::new();

	for &plant in input.plants.iter()
	{
		actions.push(Action
		{
			pos: plant,
			kind: ActionKind::Plant,
		});
	}

	for &seed in input.seeds.iter()
	{
		actions.push(Action
		{
			pos: seed,
			kind: ActionKind::Collect,
		});
	}

	let mut rng = rng();
	loop
	{
		actions.shuffle(&mut rng);

		let (p1, d1) = resolve_fast2(&input, &actions, true);
		let (p2, d2) = resolve_q_fast(&input, &actions, true);

		if p1 != p2 || d1 != d2
		{
			for action in actions
			{
				println!("{} {:3} {:3}", if action.kind == ActionKind::Plant { "Plant" } else { "Seed " }, action.pos[0], action.pos[1])
			}
			dbg!(p1, p2);
			dbg!(d1, d2);
			break;
		}

		assert_eq!(p1, p2);
		assert_eq!(d1, d2);
	}
}