use crate::dev::{orientation, GetEdge, Neighbours};
use crate::extended::header::Header;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;

///Dynamic programming version of dijkstras algorithm. Allows for efficient search of multiple end points.
struct Dijkstra<'a, Graph, Key, Edge, Weight> {
    from: &'a Key,
    graph: &'a Graph,
    visited: HashMap<&'a Key, (Weight, &'a Key, &'a Edge)>,
    queue: BinaryHeap<Reverse<Header<Weight, &'a Key>>>,
}

impl<'a, Graph, Key, Edge, Weight> Dijkstra<'a, Graph, Key, Edge, Weight>
where
    Key: Eq + Hash,
{
    pub fn new<Orientation>(graph: &'a Graph, from: &'a Key) -> Self
    where
        Weight: Ord + Clone,
        Orientation: orientation::Orientation,
        Graph:
            Neighbours<'a, Orientation, Key, Edge = &'a Edge> + GetEdge<'a, Edge, Output = Weight>,
    {
        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();

        for (edge, to) in graph.neighbours(from).into_iter().flatten() {
            if let Some(weight) = graph.get_edge(edge) {
                visited.insert(to, (weight.clone(), from, edge));
                queue.push(Reverse(Header(weight, to)))
            }
        }
        Self {
            from,
            graph,
            visited,
            queue,
        }
    }

    pub fn path(&self, mut to: &'a Key) -> Option<Vec<&(Weight, &Key, &Edge)>> {
        let mut output = Vec::new();
        while let Some(path) = self.visited.get(to) {
            to = path.1;
            output.push(path);
        }
        if self.from == to {
            output.reverse();
            Some(output)
        } else {
            None
        }
    }

    fn cache_path<Orientation>(&mut self, to: &Key)
    where
        Orientation: orientation::Orientation,
        Weight: Ord + Clone + Add,
        <Weight as Add>::Output: Into<Weight>,
        Graph:
            Neighbours<'a, Orientation, Key, Edge = &'a Edge> + GetEdge<'a, Edge, Output = Weight>,
    {
        while let Some(Reverse(Header(weight, from))) = self.queue.pop() {
            for (edge, to) in self.graph.neighbours(from).into_iter().flatten() {
                let weight = (self.visited.get(from).unwrap().0.clone() + weight.clone()).into();

                if !self.visited.contains_key(to) {
                    self.visited.insert(to, (weight.clone(), from, edge));
                    self.queue.push(Reverse(Header(weight, to)));
                }
            }
            if to == from {
                break;
            }
        }
    }

    pub fn to<Orientation>(&mut self, to: &'a Key) -> Option<Vec<&(Weight, &Key, &Edge)>>
    where
        Orientation: orientation::Orientation,
        Weight: Ord + Clone + Add,
        <Weight as Add>::Output: Into<Weight>,
        Graph:
            Neighbours<'a, Orientation, Key, Edge = &'a Edge> + GetEdge<'a, Edge, Output = Weight>,
    {
        if !self.visited.contains_key(to) {
            self.cache_path(to);
        }
        self.path(to)
    }
}
