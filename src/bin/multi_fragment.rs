use codingup_qualifs::{distance, solve_and_write_output, Action, ActionKind};
use codingup_qualifs::io::{arg_file_name, read_input};
use image::{Rgb, RgbImage};
use line_drawing::Bresenham;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NodeKind
{
	Start,
	Plant,
	Seed
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NodeState
{
	NotLinked,
	Linked(usize),
	Full(usize, usize),
}

#[derive(Debug, Copy, Clone)]
struct Node
{
	pos: [i32; 2],
	kind: NodeKind,
	state: NodeState,

	linked_node: usize,
	has_seed: bool,
	seed_used: u32,
}


fn main()
{
	let input = read_input().unwrap();
	
	let mut plant_count = input.plants.len();

	let mut nodes: Vec<Node> = vec![
		Node
		{
			pos: [0; 2],
			kind: NodeKind::Start,
			state: NodeState::Linked(0),
			linked_node: 0,
			has_seed: true,
			seed_used: 0,
		},
	];

	for plant in input.plants.iter().copied()
	{
		let index = nodes.len();
		nodes.push(Node
		{
			pos: plant,
			kind: NodeKind::Plant,
			state: NodeState::NotLinked,
			linked_node: index,
			has_seed: false,
			seed_used: 1,
		});
	}

	for seed in input.seeds.iter().copied()
	{
		let index = nodes.len();
		nodes.push(Node
		{
			pos: seed,
			kind: NodeKind::Seed,
			state: NodeState::NotLinked,
			linked_node: index,
			has_seed: true,
			seed_used: 0,
		});
	}









	let mut max_pos = [0_u32; 2];

	for l in [&input.plants, &input.seeds]
	{
		for pos in l
		{
			if pos[0] as u32 > max_pos[0]
			{
				max_pos[0] = pos[0] as u32;
			}

			if pos[1] as u32 > max_pos[1]
			{
				max_pos[1] = pos[1] as u32;
			}
		}
	}

	let mut img = RgbImage::new(max_pos[0] + 1, max_pos[1] + 1);









	let mut pool: Vec<usize> = (0..nodes.len()).collect();
	let mut end_phase = false;

	while plant_count > 1 && !pool.is_empty()
	{
		let mut min_dist = i32::MAX;
		let mut min_value = None;

		for i in 1..pool.len()
		{
			for j in 0..i
			{
				let a = nodes[pool[i]];
				let b = nodes[pool[j]];

				let dist = distance(a.pos, b.pos);

				if dist >= min_dist
				{
					continue;
				}

				if a.linked_node == pool[j]
				{
					continue; // don't allow cycles
				}

				match (a.kind, b.kind)
				{
					(NodeKind::Seed, NodeKind::Seed) =>
					{
						if !end_phase
						{
							continue;
						}
					},
					(NodeKind::Seed, NodeKind::Start) => continue,
					(NodeKind::Start, NodeKind::Seed) => continue,
					(NodeKind::Seed, NodeKind::Plant) =>
					{
						if end_phase
						{
							if b.seed_used == 0
							{
								continue;
							}
						}
						else if b.seed_used < input.seed_capacity
						{
							continue;
						}
					},
					(NodeKind::Plant, NodeKind::Seed) =>
					{
						if end_phase
						{
							if b.seed_used == 0
							{
								continue;
							}
						}
						else if a.seed_used < input.seed_capacity
						{
							continue;
						}
					},
					_ =>
					{
						let new_seed_used = a.seed_used + b.seed_used;
						if new_seed_used > input.seed_capacity
						{
							continue;
						}
					},
				}

				min_dist = dist;
				min_value = Some((i, j));
			}
		}

		let Some((i, j)) = min_value
		else
		{
			if end_phase
			{
				println!("No more option !!");
				break;
			}
			else
			{
				end_phase = true;
				continue;
			}
		};

		let (i, j) = (usize::max(i, j), usize::min(i, j));

		let ia = pool[i];
		let ib = pool[j];

		let a = nodes[ia];
		let b = nodes[ib];





		

		for (x, y) in Bresenham::new((a.pos[0], a.pos[1]), (b.pos[0], b.pos[1]))
		{
			img.put_pixel(x as u32, max_pos[1] - y as u32, Rgb::from([255, 0, 0]));
		}





		// add link
		let new_seed_used = a.seed_used + b.seed_used;

		let linked_node_a = &mut nodes[a.linked_node];
		linked_node_a.linked_node = b.linked_node;
		if !a.has_seed
		{
			linked_node_a.seed_used = new_seed_used;
			linked_node_a.has_seed = b.has_seed;
		}

		let linked_node_b = &mut nodes[b.linked_node];
		linked_node_b.linked_node = a.linked_node;
		if !b.has_seed
		{
			linked_node_b.seed_used = new_seed_used;
			linked_node_b.has_seed = a.has_seed;
		}

		nodes[ia].state = match a.state
		{
			NodeState::Linked(fl) =>
			{
				pool.remove(i); // i > j
				if a.kind == NodeKind::Plant
				{
					plant_count -= 1;
				}
				NodeState::Full(fl, ib)
			},
			_ => NodeState::Linked(ib),
		};

		nodes[ib].state = match b.state
		{
			NodeState::Linked(fl) =>
			{
				pool.remove(j); // i > j
				if b.kind == NodeKind::Plant
				{
					plant_count -= 1;
				}
				NodeState::Full(fl, ia)
			},
			_ => NodeState::Linked(ia),
		};
	}


	



	for plant in input.plants.iter().copied()
	{
		let p = img.get_pixel(plant[0] as u32, plant[1] as u32);
		img.put_pixel(plant[0] as u32, max_pos[1] - plant[1] as u32, Rgb::from([p.0[0], 255, 0]));
	}

	for seed in input.seeds.iter().copied()
	{
		let p = img.get_pixel(seed[0] as u32, seed[1] as u32);
		img.put_pixel(seed[0] as u32, max_pos[1] - seed[1] as u32, Rgb::from([p.0[0], p.0[1], 255]));
	}

	let file_name = arg_file_name();
	let exe_name = std::env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().to_owned();
	img.save(format!("output/{file_name}_{exe_name}.png")).unwrap();




	


	dbg!(plant_count, pool.len());

	let mut prev_node = 0;

	let mut cur_node = 0;

	let mut actions = Vec::new();

	while let NodeState::Full(a, b) = nodes[cur_node].state
	{
		let next_node = if a == prev_node { b } else { a };

		let node = nodes[next_node];

		actions.push(Action
		{
			pos: node.pos,
			kind: if node.kind == NodeKind::Plant { ActionKind::Plant } else { ActionKind::Collect },
		});

		prev_node = cur_node;
		cur_node = next_node;
	}

	solve_and_write_output(&input, &actions);
}