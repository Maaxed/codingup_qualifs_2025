pub mod dijkstra;
pub mod io;
pub mod quantum;
pub mod prim;

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use dijkstra::WeightedNode;
use io::{Input, OutAction};
use quantum::QPos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionKind
{
	Collect,
	Plant,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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


pub fn distance(a: [i32; 2], b: [i32; 2]) -> i32
{
	let delta = [a[0] - b[0], a[1] - b[1]];
	delta[0].abs() + delta[1].abs()
}


pub fn solve_and_write_output(input: &Input, actions: &[Action])
{
	let (mut res, plant_count, distance_traveled) = resolve(input, actions);

	io::write_output(res.make_contiguous(), Some(actions), plant_count, distance_traveled);
}


pub fn resolve(input: &Input, actions: &[Action]) -> (VecDeque<OutAction>, usize, i32)
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

	let mut end_point: (i32, State) = (0, initial_state);

	while let Some(WeightedNode(distance_traveled, (back, state))) = priority_queue.pop()
	{
		if prev_move.contains_key(&state)
		{
			continue;
		}

		prev_move.insert(state, back);

		if state.action_index >= actions.len()
		{
			continue;
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

				if distance_traveled + (dist - input.range).max(0) > input.max_distance as i32
				{
					continue;
				}

				if dist <= input.range
				{
					// No move required
					let new_state = State
					{
						robot_pos: pos,
						action_index,
					};

					if (action_index, -distance_traveled) > (end_point.1.action_index, -end_point.0)
					{
						end_point = (distance_traveled, new_state);
					}

					priority_queue.push(WeightedNode(distance_traveled, (back, new_state)));
				}
				else
				{
					// Move is required
					let new_distance_traveled = distance_traveled + dist - input.range;
					
					let sign = [delta[0].signum(), delta[1].signum()];
					for dx in i32::max(0, input.range - abs[1])..=i32::min(abs[0], input.range)
					{
						let dy = input.range - dx;

						let new_pos = [action.pos[0] - sign[0] * dx, action.pos[1] - sign[1] * dy];

						let new_state = State
						{
							robot_pos: new_pos,
							action_index,
						};

						if (action_index, -new_distance_traveled) > (end_point.1.action_index, -end_point.0)
						{
							end_point = (new_distance_traveled, new_state);
						}
	
						priority_queue.push(WeightedNode(new_distance_traveled, (back, new_state)));
					}
				}
			},
			ActionKind::Collect =>
			{
				let dist = distance(pos, action.pos);
				let new_distance_traveled = distance_traveled + dist;
				
				if new_distance_traveled > input.max_distance as i32
				{
					continue;
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

	let (distance_traveled, state) = end_point;
	
	let mut moves = VecDeque::new();

	let mut state = &state;

	let mut plant_count = 0;

	let mut back = &prev_move[state];
	while let Some(b) = back
	{
		if b.action.kind == ActionKind::Collect && moves.is_empty()
		{
			// Skip action
		}
		else
		{
			if b.action.kind == ActionKind::Plant
			{
				plant_count += 1;
			}

			moves.push_front(b.action.as_output());
			if state.robot_pos != b.old_state.robot_pos
			{
				moves.push_front(OutAction::Move(state.robot_pos));
			}
		}

		state = &b.old_state;
		back = &prev_move[state];
	}

	(moves, plant_count, distance_traveled)
}


pub fn resolve_fast(input: &Input, actions: &[Action], limit_distance: bool) -> (usize, i32)
{
	#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
	struct State
	{
		robot_pos: [i32; 2],
		action_index: usize,
	}

	let mut priority_queue = BinaryHeap::new();

	let initial_state = State
	{
		robot_pos: [0, 0],
		action_index: 0,
	};

	priority_queue.push(WeightedNode(0, initial_state));

	let mut explored: HashSet<State> = HashSet::new();

	let mut end_point: (i32, State) = (0, initial_state);

	while let Some(WeightedNode(distance_traveled, state)) = priority_queue.pop()
	{
		if !explored.insert(state)
		{
			continue;
		}

		if state.action_index >= actions.len()
		{
			continue;
		}

		let pos = state.robot_pos;

		let action = actions[state.action_index];
		let action_index = state.action_index + 1;

		match action.kind
		{
			ActionKind::Plant =>
			{
				let delta = [action.pos[0] - pos[0], action.pos[1] - pos[1]];
				let abs = [delta[0].abs(), delta[1].abs()];
				let dist = abs[0] + abs[1];

				if limit_distance && distance_traveled + (dist - input.range).max(0) > input.max_distance as i32
				{
					continue;
				}

				if dist <= input.range
				{
					// No move required
					let new_state = State
					{
						robot_pos: pos,
						action_index,
					};

					if (action_index, -distance_traveled) > (end_point.1.action_index, -end_point.0)
					{
						end_point = (distance_traveled, new_state);
					}

					priority_queue.push(WeightedNode(distance_traveled, new_state));
				}
				else
				{
					// Move is required
					let new_distance_traveled = distance_traveled + dist - input.range;
					
					let sign = [delta[0].signum(), delta[1].signum()];
					for dx in i32::max(0, input.range - abs[1])..=i32::min(abs[0], input.range)
					{
						let dy = input.range - dx;

						let new_pos = [action.pos[0] - sign[0] * dx, action.pos[1] - sign[1] * dy];

						let new_state = State
						{
							robot_pos: new_pos,
							action_index,
						};

						if (action_index, -new_distance_traveled) > (end_point.1.action_index, -end_point.0)
						{
							end_point = (new_distance_traveled, new_state);
						}
	
						priority_queue.push(WeightedNode(new_distance_traveled, new_state));
					}
				}
			},
			ActionKind::Collect =>
			{
				let dist = distance(pos, action.pos);
				let new_distance_traveled = distance_traveled + dist;
				
				if limit_distance && new_distance_traveled > input.max_distance as i32
				{
					continue;
				}

				priority_queue.push(WeightedNode(new_distance_traveled,
					State
					{
						robot_pos: action.pos,
						action_index,
					},
				));
			},
		}
	}

	let (distance_traveled, state) = end_point;

	let mut plant_count = 0;

	for action in &actions[0..state.action_index]
	{
		if action.kind == ActionKind::Plant
		{
			plant_count += 1;
		}
	}

	(plant_count, distance_traveled)
}


pub fn resolve_q_fast(input: &Input, actions: &[Action], limit_distance: bool) -> (usize, i32)
{
	let mut pos = QPos::default();
	let mut distance_traveled = 0;
	let mut distance_back = 0;
	let mut plant_count = 0;

	for action in actions
	{
		let (new_pos, dist) = pos.apply_action(input, action);
		
		if limit_distance && (distance_traveled + dist) as u32 > input.max_distance
		{
			break;
		}

		pos = new_pos;
		distance_traveled += dist;

		match action.kind
		{
			ActionKind::Plant =>
			{
				plant_count += 1;
				distance_back = 0;
			},
			ActionKind::Collect =>
			{
				distance_back += dist;
			},
		}
	}

	(plant_count, distance_traveled - distance_back)
}



pub fn unresolve(actions: &[OutAction]) -> Vec<Action>
{
	let mut moves = Vec::new();

	let mut robot_pos = [0, 0];

	for action in actions
	{
		match action
		{
			OutAction::Move(pos) => robot_pos = *pos,
			OutAction::Plant(pos) => moves.push(Action { pos: *pos, kind: ActionKind::Plant }),
			OutAction::Collect => moves.push(Action { pos: robot_pos, kind: ActionKind::Collect }),
		}
	}

	moves
}
