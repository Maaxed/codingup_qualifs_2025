use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufWriter};

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

fn arg_file_name() -> String
{
	let mut args = std::env::args();
	args.next();

	args.next().unwrap_or("1".to_owned())
}

pub fn read_input() -> serde_json::Result<Input>
{
	let file_name = arg_file_name();
	let reader = BufReader::new(File::open(format!("input/{file_name}.json")).unwrap());
	serde_json::from_reader(reader)
}

pub fn write_output(actions: &[Action])
{
	println!("Solution found in {} moves!", actions.len());
	let exe_name = std::env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().to_owned();
	let file_name = arg_file_name();
	let buffer = BufWriter::new(File::create(format!("output/{file_name}_{exe_name}.json")).unwrap());
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
	
	serde_json::to_writer_pretty(buffer, &moves_str).unwrap();
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
