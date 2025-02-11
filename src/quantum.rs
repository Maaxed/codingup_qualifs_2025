use crate::{distance, Action, ActionKind};
use crate::io::Input;


#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct QPos
{
	pub robot_pos: [i32; 2],
	pub range: i32, // positive is up right, negative is up left diagonal
}


impl QPos
{
	pub fn apply_plant(&self, input: &Input, plant_pos: [i32; 2]) -> (Self, i32)
	{
		let pos = self.robot_pos;
		let range = self.range;


		let sign = range.signum();
		let sign = if sign == 0 { 1 } else { sign };

		let range = range.abs();

		let delta = [(plant_pos[0] - pos[0]) * sign, plant_pos[1] - pos[1]];

		let dist1 = distance(delta, [0;2]);
		let dist2 = distance(delta, [range; 2]);

		let dist = i32::min(dist1, dist2) - input.range;

		let mut new_state = *self;

		if dist <= 0
		{
			// No move required
			// Decrease range
			let d1 = dist1 - input.range;
			let d2 = dist2 - input.range;
			if d1 > 0
			{
				let d = (d1 + 1) / 2;
				new_state.robot_pos[0] += d * sign;
				new_state.robot_pos[1] += d;
				new_state.range -= d;
			}
			else if d2 > 0
			{
				let d = (d2 + 1) / 2;
				new_state.range -= d;
			}
		}
		else
		{
			// Move is required
			
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

				new_state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
				new_state.range = -sign * new_range;
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

				new_state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
				new_state.range = -sign * new_range;
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

				new_state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
				new_state.range = sign * new_range;
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

				new_state.robot_pos = [pos[0] + new_delta[0] * sign, pos[1] + new_delta[1]];
				new_state.range = sign * new_range;
			}
		}

		(new_state, dist.max(0))
	}

	pub fn apply_seed(&self, plant_pos: [i32; 2]) -> (Self, i32)
	{
		let pos = self.robot_pos;
		let range = self.range;

		let dist = if range == 0
		{
			distance(plant_pos, pos)
		}
		else
		{
			let sign = range.signum();
			let range = range.abs();

			let delta = [(plant_pos[0] - pos[0]) * sign, plant_pos[1] - pos[1]];

			let ref_pos = [delta[0].clamp(0, range); 2];

			distance(delta, ref_pos)
		};

		(
			QPos
			{
				robot_pos: plant_pos,
				range: 0,
			}
			,
			dist,
		)
	}

	pub fn apply_action(&self, input: &Input, action: &Action) -> (Self, i32)
	{
		match action.kind
		{
			ActionKind::Plant => self.apply_plant(input, action.pos),
			ActionKind::Collect => self.apply_seed(action.pos),
		}
	}
}
