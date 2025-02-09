use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::fmt::Write;

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
pub enum OutAction
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

pub fn write_output(actions: &[OutAction], plant_count: usize, distance_traveled: i32)
{
	println!("Solution found in {} moves", actions.len());

	dbg!(plant_count);
	dbg!(distance_traveled);

	let mut exe_name = std::env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().to_owned();
	let file_name = arg_file_name();
	for arg in std::env::args().skip(2)
	{
		write!(&mut exe_name, "_{arg}").unwrap();
	}
	let output_base_name = format!("output/{file_name}_{exe_name}");

	std::fs::write(format!("{output_base_name}.meta"), format!("{plant_count} {distance_traveled}")).unwrap();

	let buffer = BufWriter::new(File::create(format!("{output_base_name}.json")).unwrap());
	let mut moves_str = Vec::new();
	for action in actions
	{
		moves_str.push(match action
		{
			OutAction::Move(pos) => format!("MOVE {} {}", pos[0], pos[1]),
			OutAction::Plant(pos) => format!("PLANT {} {}", pos[0], pos[1]),
			OutAction::Collect => "COLLECT".to_string(),
		});
	}
	
	serde_json::to_writer_pretty(buffer, &moves_str).unwrap();
}
