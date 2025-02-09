use codingup_qualifs::{distance, io::*};

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

pub enum Res
{
	SolutionFound(Option<(i32, usize, MyAction)>),
	NoSolution,
}

fn find_best_action(input: &Input, state: &State, max_cost: i32) -> Res
{
	if state.plants.is_empty()
	{
		dbg!(max_cost);
		return Res::SolutionFound(None);
	}

	let pos = state.robot_pos;

	let mut min_cost = max_cost;
	let mut min_action = None;

	if state.seed_storage > 0
	{
		for (index, plant) in state.plants.iter().enumerate()
		{
			let dist = distance(pos, *plant);
			
			let cost = (dist - input.range).max(0);
			
			if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				continue;
			}

			let mut new_state = state.clone();
			new_state.seed_storage -= 1;
			new_state.plants.remove(index);

			if dist <= input.range
			{
				// No move required

				let mut cost = cost;

				match find_best_action(input, &new_state, min_cost)
				{
					Res::SolutionFound(Some((child_cost, _, _))) =>
					{
						cost += child_cost;
					},
					Res::SolutionFound(None) =>
					{ },
					Res::NoSolution => continue,
				}

				if cost < min_cost
				{
					min_cost = cost;
					min_action = Some((index, MyAction { pos, kind: OutAction::Plant(*plant) }));
				}
			}
			else
			{
				let delta = [plant[0] - pos[0], plant[1] - pos[1]];

				// Move is required
				let sign = [delta[0].signum(), delta[1].signum()];
				for dx in i32::max(0, input.range - delta[1].abs())..=i32::min(delta[0].abs(), input.range)
				{
					let dy = input.range - dx;

					let new_pos = [plant[0] - sign[0] * dx, plant[1] - sign[1] * dy];
					new_state.robot_pos = new_pos;
					
					let mut cost = cost;

					match find_best_action(input, &new_state, min_cost - cost)
					{
						Res::SolutionFound(Some((child_cost, _, _))) =>
						{
							cost += child_cost;
						},
						Res::SolutionFound(None) =>
						{ },
						Res::NoSolution => continue,
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

			let mut cost = dist;

			if cost >= min_cost // we are worst even without checking the children nodes, prune this branch
			{
				continue;
			}

			let mut new_state = state.clone();
			new_state.seed_storage = input.seed_capacity;
			new_state.robot_pos = *seed;
			new_state.seeds.remove(index);

			match find_best_action(input, &new_state, min_cost - cost)
			{
				Res::SolutionFound(Some((child_cost, _, _))) =>
				{
					cost += child_cost;
				},
				Res::SolutionFound(None) =>
				{ },
				Res::NoSolution => continue,
			}

			if cost < min_cost
			{
				min_cost = cost;
				min_action = Some((index, MyAction { pos: *seed, kind: OutAction::Collect }));
			}
		}
	}

	let Some((min_index, min_action)) = min_action
	else
	{
		return Res::NoSolution;
	};

	Res::SolutionFound(Some((min_cost, min_index, min_action)))
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

	let mut distance_traveled = 0;

	let mut moves = Vec::new();

	while !state.plants.is_empty()
	{
		let Res::SolutionFound(Some((_dist, index, action))) = find_best_action(&input, &state, input.max_distance as i32 - distance_traveled+1)
		else
		{
			break;
		};

		if state.robot_pos != action.pos
		{
			distance_traveled += distance(state.robot_pos, action.pos);
			state.robot_pos = action.pos;
			moves.push(OutAction::Move(state.robot_pos));
		}

		moves.push(action.kind);

		match action.kind
		{
			OutAction::Plant(_) =>
			{
				state.seed_storage -= 1;
				state.plants.remove(index);
			},
			OutAction::Collect =>
			{
				state.seed_storage = input.seed_capacity;
				state.seeds.remove(index);
			},
			OutAction::Move(_) =>
			{ },
		}
	}

	write_output(&moves, input.plants.len() - state.plants.len(), distance_traveled);

	Ok(())
}
