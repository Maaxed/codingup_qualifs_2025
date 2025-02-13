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
pub fn prim2(input: &Input, pos: QPos, plants: &[[i32;2]]) -> i32
{
	if plants.is_empty()
	{
		return 0;
	}

	let mut graph: Vec<(usize, usize, i32)> = (0..plants.len())
		.map(|index|
		{
			let (_, dist) = pos.apply_plant(input, plants[index]);
			(index, 0, dist)
		})
		.collect();
	let mut tree = vec![pos];

	let mut tree_dist = 0;

	for _i in 0..plants.len()
	{
		let (best_graph_index, (best_plant_index, best_tree_index, _dist)) = graph.iter().enumerate().min_by_key(|(_, (_, _, dist))| *dist).unwrap();

		let (new_pos, new_dist) = tree[*best_tree_index].apply_plant(input, plants[*best_plant_index]);

		graph.swap_remove(best_graph_index);

		tree_dist += new_dist;
		tree.push(new_pos);

		for (plant_index, tree_index, dist) in graph.iter_mut()
		{
			let (_, new_dist) = new_pos.apply_plant(input, plants[*plant_index]);
			if new_dist < *dist
			{
				*tree_index = tree.len()-1;
				*dist = new_dist;
			}
		}
	}

	assert!(graph.is_empty());
	assert_eq!(tree.len(), plants.len()+1);

	tree_dist
}