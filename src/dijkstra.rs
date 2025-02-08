use std::cmp::Ordering;


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
