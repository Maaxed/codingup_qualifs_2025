pub mod dijkstra;
pub mod io;

use std::collections::{BinaryHeap, HashMap, VecDeque};

use dijkstra::WeightedNode;
use io::{Input, OutAction};

#[derive(Debug, Copy, Clone)]
pub enum ActionKind
{
	Collect,
	Plant,
}

#[derive(Debug, Copy, Clone)]
pub struct Action
{
	pub pos: [i32; 2],
	pub kind: ActionKind,
}

impl Action
{
	fn as_output(&self) -> OutAction
	{
		match self.kind
		{
			ActionKind::Plant => OutAction::Plant(self.pos),
			ActionKind::Collect => OutAction::Collect,
		}
	}
}



pub fn resolve(input: &Input, actions: &[Action]) -> (u32, VecDeque<OutAction>)
{
	#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
	struct State
	{
		robot_pos: [i32; 2],
		action_index: usize,
	}

	#[derive(Debug, Copy, Clone)]
	struct BackAction
	{
		old_state: State,
		action: Action,
	}

	let mut priority_queue = BinaryHeap::new();

	let initial_state = State
	{
		robot_pos: [0, 0],
		action_index: 0,
	};

	priority_queue.push(WeightedNode(0, (None, initial_state)));

	let mut prev_move: HashMap<State, Option<BackAction>> = HashMap::new();

	let mut end_point = None;

	while let Some(WeightedNode(distance_traveled, (back, state))) = priority_queue.pop()
	{
		if prev_move.contains_key(&state)
		{
			continue;
		}

		prev_move.insert(state, back);

		if state.action_index >= actions.len()
		{
			end_point = Some((distance_traveled, state));
			break;
		}

		let pos = state.robot_pos;

		let action = actions[state.action_index];
		let action_index = state.action_index + 1;

		let back = Some(BackAction
		{
			old_state: state,
			action,
		});

		match action.kind
		{
			ActionKind::Plant =>
			{
				let delta = [action.pos[0] - pos[0], action.pos[1] - pos[1]];
				let abs = [delta[0].abs(), delta[1].abs()];
				let dist = abs[0] + abs[1];

				if dist <= input.range
				{
					// No move required
					priority_queue.push(WeightedNode(distance_traveled, (
						back,
						State
						{
							robot_pos: pos,
							action_index,
						},
					)));
				}
				else
				{
					// Move is required
					let new_distance_traveled = distance_traveled + dist - input.range;

					if new_distance_traveled as u32 > input.max_distance
					{
						end_point = Some((distance_traveled, state));
						break;
					}
					
					let sign = [delta[0].signum(), delta[1].signum()];
					for dx in i32::max(0, input.range - abs[1])..=i32::min(abs[0], input.range)
					{
						let dy = input.range - dx;

						let new_pos = [action.pos[0] - sign[0] * dx, action.pos[1] - sign[1] * dy];
	
						priority_queue.push(WeightedNode(new_distance_traveled, (
							back,
							State
							{
								robot_pos: new_pos,
								action_index,
							},
						)));
					}
				}
			},
			ActionKind::Collect =>
			{
				let delta = [action.pos[0] - pos[0], action.pos[1] - pos[1]];
				let dist = delta[0].abs() + delta[1].abs();
				let new_distance_traveled = distance_traveled + dist;

				if new_distance_traveled as u32 > input.max_distance
				{
					end_point = Some((distance_traveled, state));
					break;
				}

				priority_queue.push(WeightedNode(new_distance_traveled, (
					back,
					State
					{
						robot_pos: action.pos,
						action_index,
					},
				)));
			},
		}
	}

	let Some((distance_traveled, state)) = end_point
	else
	{
		return (0, VecDeque::new());
	};
	
	let mut moves = VecDeque::new();

	let mut state = &state;

	let mut back = &prev_move[state];
	while let Some(b) = back
	{
		moves.push_front(b.action.as_output());
		if state.robot_pos != b.old_state.robot_pos
		{
			moves.push_front(OutAction::Move(state.robot_pos));
		}

		state = &b.old_state;
		back = &prev_move[state];
	}

	(distance_traveled as u32, moves)
}