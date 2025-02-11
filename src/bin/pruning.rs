use codingup_qualifs::{distance, io::*, solve_and_write_output, Action, ActionKind};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State
{
	robot_pos: [i32; 2],
	seed_storage: u32,
	seeds: Vec<[i32;2]>,
	plants: Vec<[i32;2]>,
}

fn find_best_action(input: &Input, state: &State, max_cost: (i32, i32), depth: u32) -> Option<((i32, i32), usize, Action)>
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

			((0, min_dist), min_index, Action { pos: *min_plant, kind: ActionKind::Plant })
		}
		else
		{
			let (min_dist, min_index, min_seed) = 
				state.seeds.iter()
					.enumerate()
					.map(|(index, seed)| (distance(pos, *seed), index, seed))
					.min_by_key(|(dist, _, _)| *dist)?;

			((1, min_dist), min_index, Action { pos: *min_seed, kind: ActionKind::Collect })
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

				let mut cost = (0, dist);

				if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
				{
					continue;
				}

				let mut new_state = state.clone();
				new_state.seed_storage -= 1;
				new_state.robot_pos = *plant;
				new_state.plants.remove(index);

				if let Some((child_cost, _, _)) = find_best_action(input, &new_state, (min_cost.0, min_cost.1 - dist), depth-1)
				{
					cost.0 += child_cost.0;
					cost.1 += child_cost.1;
				}

				if cost < min_cost
				{
					min_cost = cost;
					min_action = Some((index, Action { pos: *plant, kind: ActionKind::Plant }));
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
					min_action = Some((index, Action { pos: *seed, kind: ActionKind::Collect }));
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

	let mut moves = Vec::new();

	while !state.plants.is_empty()
	{
		if state.plants.len() % 100 == 0
		{
			dbg!(state.plants.len());
		}

		let Some((_dist, index, action)) = find_best_action(&input, &state, (i32::MAX, i32::MAX), depth)
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

	solve_and_write_output(&input, &moves);

	Ok(())
}
