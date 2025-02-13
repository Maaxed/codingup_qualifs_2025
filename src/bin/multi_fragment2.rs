use codingup_qualifs::{distance, solve_and_write_output, Action, ActionKind};
use codingup_qualifs::io::read_input;


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
	state: NodeState,
	linked_node: usize,
}


fn main()
{
	let input = read_input().unwrap();

	let mut nodes: Vec<Node> = vec![
		Node
		{
			pos: [0; 2],
			state: NodeState::Linked(0),
			linked_node: 0,
		},
	];

	for plant in input.plants.iter().copied()
	{
		let index = nodes.len();
		nodes.push(Node
		{
			pos: plant,
			state: NodeState::NotLinked,
			linked_node: index,
		});
	}

	let mut pool: Vec<usize> = (0..nodes.len()).collect();

	while pool.len() <= 1
	{
		let mut min_dist = i32::MAX;
		let mut min_value = None;

		for i in 1..pool.len()
		{
			for j in 0..i
			{
				let a = nodes[pool[i]];
				let b = nodes[pool[j]];

				if a.linked_node == pool[j]
				{
					continue; // don't allow cycles
				}

				let dist = distance(a.pos, b.pos);

				if dist >= min_dist
				{
					continue;
				}

				min_dist = dist;
				min_value = Some((i, j));
			}
		}

		let Some((i, j)) = min_value
		else
		{
			println!("No more option !!");
			break;
		};

		let (i, j) = (usize::max(i, j), usize::min(i, j));

		let ia = pool[i];
		let ib = pool[j];

		let a = nodes[ia];
		let b = nodes[ib];

		// add link
		let linked_node_a = &mut nodes[a.linked_node];
		linked_node_a.linked_node = b.linked_node;

		let linked_node_b = &mut nodes[b.linked_node];
		linked_node_b.linked_node = a.linked_node;

		nodes[ia].state = match a.state
		{
			NodeState::Linked(fl) =>
			{
				pool.remove(i); // i > j
				NodeState::Full(fl, ib)
			},
			_ => NodeState::Linked(ib),
		};

		nodes[ib].state = match b.state
		{
			NodeState::Linked(fl) =>
			{
				pool.remove(j); // i > j
				NodeState::Full(fl, ia)
			},
			_ => NodeState::Linked(ia),
		};
	}

	let mut prev_node = 0;

	let mut cur_node = 0;

	let mut actions = Vec::new();

	let mut seed_storage = input.seed_capacity;

	let mut seeds = input.seeds.clone();

	while let NodeState::Full(a, b) = nodes[cur_node].state
	{
		let next_node = if a == prev_node { b } else { a };

		let node = nodes[next_node];

		if seed_storage == 0
		{
			seed_storage = input.seed_capacity;
			// find seed to collect
			
			let cur_node = nodes[cur_node];

			let Some((seed_index, seed)) = seeds.iter().enumerate().min_by_key(|(_, seed)|
			{
				distance(cur_node.pos, **seed) + distance(node.pos, **seed)
			})
			else
			{
				break;
			};
			
			actions.push(Action
			{
				pos: *seed,
				kind: ActionKind::Collect,
			});

			seeds.swap_remove(seed_index);
		}

		seed_storage -= 1;

		actions.push(Action
		{
			pos: node.pos,
			kind: ActionKind::Plant,
		});

		prev_node = cur_node;
		cur_node = next_node;
	}

	solve_and_write_output(&input, &actions);
}