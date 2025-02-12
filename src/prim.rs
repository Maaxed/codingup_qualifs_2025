use std::collections::HashSet;

use crate::io::Input;
use crate::quantum::QPos;


pub fn get_min(input: &Input, pos: QPos, graph: &HashSet<usize>, plants: &[[i32; 2]]) -> Option<(usize, i32)>
{
	graph.iter().map(|index|
	{
		let (_, dist) = pos.apply_plant(input, plants[*index]);
		(*index, dist)
	}).min_by_key(|(_, dist)| *dist)
}

// prim's algorithm
fn compute_tree(input: &Input, tree: &mut Vec<(QPos, (usize, i32))>, graph: &mut HashSet<usize>, plants: &[[i32; 2]]) -> i32
{
	let mut tree_dist = 0;

	for i in 0..
		{
			let (best_tree_pos, (best_plant_index, _dist)) = tree.iter().min_by_key(|(_, (_, dist))| *dist).unwrap();

			let best_plant_index = *best_plant_index;

			graph.remove(&best_plant_index);

			let (new_pos, new_dist) = best_tree_pos.apply_plant(input, plants[best_plant_index]);

			tree_dist += new_dist;

			if i >= plants.len()-1
			{
				tree.push((new_pos, (0, 0)));
				break;
			}

			for (tree_pos, (plant_index, dist)) in tree.iter_mut()
			{
				if *plant_index == best_plant_index
				{
					(*plant_index, *dist) = get_min(input, *tree_pos, &*graph, plants).unwrap();
				}
			}

			tree.push((new_pos, get_min(input, new_pos, &*graph, plants).unwrap()));
		}
	tree_dist
}


pub fn prim(input: &Input, pos: QPos, plants: &[[i32;2]]) -> i32
{
	if plants.is_empty()
	{
		return 0;
	}

	let mut graph: HashSet<usize> = (0..plants.len()).collect();
	let mut tree = vec![(pos, get_min(input, pos, &graph, plants).unwrap())];

	let tree_dist = compute_tree(input, &mut tree, &mut graph, plants);

	assert!(graph.is_empty());
	assert_eq!(tree.len(), plants.len()+1);

	tree_dist
}




// prim's algorithm
pub fn prim2(input: &Input, pos: QPos, plants: &[[i32;2]], seeds: &[[i32;2]]) -> i32
{
	if plants.is_empty()
	{
		return 0;
	}

	let mut graph: HashSet<usize> = (0..plants.len()).collect();
	let mut tree = vec![(pos, get_min(input, pos, &graph, plants).unwrap())];

	let mut tree_dist = compute_tree(input, &mut tree, &mut graph, plants);

	assert!(graph.is_empty());
	assert_eq!(tree.len(), plants.len()+1);

	let collect_count = ((plants.len().max(1) - 1) / input.seed_capacity as usize).min(seeds.len()-1);

	if collect_count > 0 && seeds.is_empty()
	{
		let mut best_seeds: Vec<_> = seeds.iter().map(|seed|
		{
			tree.iter()
				.map(|(pos, _)|
				{
					let (_, dist) = pos.apply_seed(*seed);
					dist
				})
				.min().unwrap()
		}).collect();

		best_seeds.select_nth_unstable(collect_count);

		for dist in &best_seeds[..collect_count]
		{
			tree_dist += dist;
		}
	}

	tree_dist
}