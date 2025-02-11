pub mod dijkstra;
pub mod io;

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use dijkstra::WeightedNode;
use io::{Input, OutAction};
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

	let (p, d) = resolve_faster(input, actions, true);

	assert_eq!(plant_count, p);
	assert_eq!(distance_traveled, d);

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
			//println!("Found solution with all plants!");
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
						//println!("Out of energy!");
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
				let dist = distance(pos, action.pos);
				let new_distance_traveled = distance_traveled + dist;

				if new_distance_traveled as u32 > input.max_distance
				{
					//println!("Out of energy!");
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

	let Some((mut distance_traveled, state)) = end_point
	else
	{
		return (VecDeque::new(), 0, 0);
	};
	
	let mut moves = VecDeque::new();

	let mut state = &state;

	let mut plant_count = 0;

	let mut back = &prev_move[state];
	while let Some(b) = back
	{
		if b.action.kind == ActionKind::Collect && moves.is_empty()
		{
			distance_traveled -= distance(state.robot_pos, b.old_state.robot_pos);
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

	priority_queue.push(WeightedNode(0, (0, initial_state)));

	let mut explored: HashSet<State> = HashSet::new();

	let mut end_point = None;

	while let Some(WeightedNode(distance_traveled, (dist_back, state))) = priority_queue.pop()
	{
		if !explored.insert(state)
		{
			continue;
		}

		if state.action_index >= actions.len()
		{
			end_point = Some((distance_traveled, dist_back, state));
			break;
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

				if dist <= input.range
				{
					// No move required
					priority_queue.push(WeightedNode(distance_traveled, (
						0,
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

					if limit_distance && new_distance_traveled as u32 > input.max_distance
					{
						end_point = Some((distance_traveled, dist_back, state));
						break;
					}
					
					let sign = [delta[0].signum(), delta[1].signum()];
					for dx in i32::max(0, input.range - abs[1])..=i32::min(abs[0], input.range)
					{
						let dy = input.range - dx;

						let new_pos = [action.pos[0] - sign[0] * dx, action.pos[1] - sign[1] * dy];
	
						priority_queue.push(WeightedNode(new_distance_traveled, (
							0,
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
				let dist = distance(pos, action.pos);
				let new_distance_traveled = distance_traveled + dist;

				if limit_distance && new_distance_traveled as u32 > input.max_distance
				{
					end_point = Some((distance_traveled, dist_back, state));
					break;
				}

				priority_queue.push(WeightedNode(new_distance_traveled, (
					dist_back + dist,
					State
					{
						robot_pos: action.pos,
						action_index,
					},
				)));
			},
		}
	}

	let Some((mut distance_traveled, dist_back, state)) = end_point
	else
	{
		return (0, 0);
	};

	let mut plant_count = 0;

	for action in &actions[0..state.action_index]
	{
		if action.kind == ActionKind::Plant
		{
			plant_count += 1;
		}
	}

	distance_traveled -= dist_back;

	(plant_count, distance_traveled)
}



pub fn resolve_faster(input: &Input, actions: &[Action], limit_distance: bool) -> (usize, i32)
{
	#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
	struct State
	{
		robot_pos: [i32; 2],
		range: i32, // positive is up right, negative is up left diagonal
		distance_traveled: i32,
		distance_back: i32,
		plant_count: usize,
	}

	let mut state = State
	{
		robot_pos: [0, 0],
		range: 0,
		distance_traveled: 0,
		distance_back: 0,
		plant_count: 0,
	};

	for action in actions
	{
		let pos = state.robot_pos;
		let range = state.range;
		match action.kind
		{
			ActionKind::Plant =>
			{
				let sign = range.signum();
				let sign = if sign == 0 { 1 } else { sign };

				let range = range.abs();

				let delta = [(action.pos[0] - pos[0]) * sign, action.pos[1] - pos[1]];

				let dist1 = distance(delta, [0;2]);
				let dist2 = distance(delta, [range; 2]);

				let dist = i32::min(dist1, dist2) - input.range;

				if limit_distance && (state.distance_traveled + dist.max(0)) as u32 > input.max_distance
				{
					break;
				}

				state.plant_count += 1;
				state.distance_back = 0;

				if dist <= 0
				{
					// No move required
					// Decrease range
					let d1 = dist1 - input.range;
					let d2 = dist2 - input.range;
					if d1 > 0
					{
						let d = (d1 + 1) / 2;
						state.robot_pos[0] += d * sign;
						state.robot_pos[1] += d;
						state.range -= d;
					}
					else if d2 > 0
					{
						let d = (d2 + 1) / 2;
						state.range -= d;
					}
				}
				else
				{
					// Move is required

					state.distance_traveled += dist;
					
					if delta[0] <= 0 && delta[1] <= 0
					{
						// Partial top right

						let mut new_delta = [delta[0] + input.range, delta[1]];
						let mut new_range = input.range;

						if new_delta[0] > 0
						{
							// Clamp
							let cd = new_delta[0];
							new_delta[0] -= cd;
							new_delta[1] += cd;
							new_range -= cd;
						}
						if new_delta[1] + new_range > 0
						{
							// Clamp
							let cd = new_delta[1] + new_range;
							new_range -= cd;
						}

						state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
						state.range = -sign * new_range;
					}
					else if delta[0] >= range && delta[1] >= range
					{
						// Partial bottom left

						let mut new_delta = [delta[0], delta[1] - input.range];
						let mut new_range = input.range;

						if new_delta[1] < range
						{
							// Clamp
							let cd = range - new_delta[1];
							new_delta[0] -= cd;
							new_delta[1] += cd;
							new_range -= cd;
						}
						if new_delta[0] - new_range < range
						{
							// Clamp
							let cd = range - (new_delta[0] - new_range);
							new_range -= cd;
						}

						state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
						state.range = -sign * new_range;
					}
					else if delta[1] >= delta[0]
					{
						// Partial bottom right
						
						let mut new_delta = [delta[0], delta[1] - input.range];
						let mut new_range = input.range;

						if new_delta[1] < 0
						{
							// Clamp
							let cd = -new_delta[1];
							new_delta[0] += cd;
							new_delta[1] += cd;
							new_range -= cd;
						}
						if new_delta[0] + new_range > range
						{
							// Clamp
							let cd = new_delta[0] + new_range - range;
							new_range -= cd;
						}

						state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
						state.range = sign * new_range;
					}
					else
					{
						// Partial top left
						
						let mut new_delta = [delta[0] - input.range, delta[1]];
						let mut new_range = input.range;

						if new_delta[0] < 0
						{
							// Clamp
							let cd = -new_delta[0];
							new_delta[0] += cd;
							new_delta[1] += cd;
							new_range -= cd;
						}
						if new_delta[1] + new_range > range
						{
							// Clamp
							let cd = new_delta[1] + new_range - range;
							new_range -= cd;
						}

						state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
						state.range = sign * new_range;
					}
				}
			},
			ActionKind::Collect =>
			{
				let dist = if range == 0
				{
					distance(action.pos, pos)
				}
				else
				{
					let sign = range.signum();
					let range = range.abs();

					let delta = [(action.pos[0] - pos[0]) * sign, action.pos[1] - pos[1]];

					let ref_pos = [delta[0].clamp(0, range); 2];

					distance(delta, ref_pos)
				};

				if limit_distance && (state.distance_traveled + dist) as u32 > input.max_distance
				{
					break;
				}

				state.robot_pos = action.pos;
				state.range = 0;
				state.distance_traveled += dist;
				state.distance_back += dist;
			},
		}
	}

	(state.plant_count, state.distance_traveled - state.distance_back)
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
