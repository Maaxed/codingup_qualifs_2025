use std::cmp::Ordering;
use std::fs::File;
use std::io::BufWriter;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input
{
	#[serde(rename(deserialize = "maxDistance"))]
	pub max_distance: u32,
	#[serde(rename(deserialize = "seedCapacity"))]
	pub seed_capacity: u32,
	pub range: i32,
	pub seeds: Vec<[i32;2]>,
	pub plants: Vec<[i32;2]>,
}

#[derive(Debug, Clone, Copy)]
pub enum Action
{
	Move([i32;2]),
	Plant([i32;2]),
	Collect,
}

pub fn read_input() -> serde_json::Result<Input>
{
	let json = include_str!("2_champ.json");

	serde_json::from_str(json)
}

pub fn write_output(actions: &[Action])
{
	println!("Solution found in {} moves!", actions.len());
	let buffer = BufWriter::new(File::create("out.json").unwrap());
	let mut moves_str = Vec::new();
	for action in actions
	{
		moves_str.push(match action
		{
			Action::Move(pos) => format!("MOVE {} {}", pos[0], pos[1]),
			Action::Plant(pos) => format!("PLANT {} {}", pos[0], pos[1]),
			Action::Collect => "COLLECT".to_string(),
		});
	}
	serde_json::to_writer(buffer, &moves_str).unwrap();
}



pub struct WeightedNode<Node>(pub i32, pub Node);
	
impl<Node> PartialEq for WeightedNode<Node>
{
	fn eq(&self, other: &Self) -> bool
	{
		self.0.eq(&other.0)
	}
}

impl<Node> Eq for WeightedNode<Node>
{ }

impl<Node> PartialOrd for WeightedNode<Node>
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl<Node> Ord for WeightedNode<Node>
{
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.0.cmp(&other.0).reverse()
	}
}
