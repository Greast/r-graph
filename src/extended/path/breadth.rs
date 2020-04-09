use crate::dev::{orientation, Neighbours};
use crate::extended::path::Path;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

///Dynamic programming version of breadth first search. Allows for efficient search of multiple end points.
struct Breadth<'a, Graph, Key, Edge, Orientation> {
    from: &'a Key,
    graph: &'a Graph,
    visited: HashMap<&'a Key, (&'a Key, &'a Edge)>,
    queue: VecDeque<&'a Key>,
    orientation: PhantomData<Orientation>,
}

impl<'a, Graph, Key, Edge, Orientation> Breadth<'a, Graph, Key, Edge, Orientation>
where
    Key: Eq + Hash,
{
    pub fn new(graph: &'a Graph, from: &'a Key) -> Self
    where
        Key: Eq + Hash,
    {
        let mut queue = VecDeque::new();
        queue.push_back(from);
        Self {
            from,
            graph,
            visited: Default::default(),
            queue,
            orientation: Default::default(),
        }
    }

    ///Construct a path from the already visited nodes, this function cannot find a path to a node it has yet to reach.
    pub fn path(&self, mut to: &'a Key) -> Option<Vec<(&Key, &Edge)>> {
        let mut output = Vec::new();
        while let Some(path) = self.visited.get(to) {
            to = path.0;
            output.push(*path);
        }
        if self.from == to {
            output.reverse();
            Some(output)
        } else {
            None
        }
    }

    fn cache_path(&mut self, to: &Key)
    where
        Orientation: orientation::Orientation,
        Graph: Neighbours<'a, Orientation, Key, Edge = &'a Edge>,
    {
        while let Some(from) = self.queue.pop_front() {
            for (edge, to) in self.graph.neighbours(from).into_iter().flatten() {
                if !self.visited.contains_key(to) {
                    self.visited.insert(to, (from, edge));
                    self.queue.push_back(to);
                }
            }
            if to == from {
                break;
            }
        }
    }
}

impl<'a, Vertex, Edge, Graph, Orientation> Path<'a, Vertex, Edge>
    for Breadth<'a, Graph, Vertex, Edge, Orientation>
where
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, Vertex, Edge = &'a Edge>,
    &'a Vertex: Eq + Hash,
    Vertex: Eq + Hash,
{
    type IntoIter = Vec<(&'a Vertex, &'a Edge)>;

    fn to(&'a mut self, to: &'a Vertex) -> Option<Self::IntoIter> {
        if !self.visited.contains_key(to) {
            self.cache_path(to);
        }
        self.path(to)
    }
}

/*
trait BreadthSearch<'a, Key, Edge, Orientation>
where
    Self: Sized,
{
    fn breadth_search(&'a self, key: &'a Key) -> Breadth<'a, Self, Key, Edge>;
}

impl<'a, Graph, Key, Edge, Orientation> BreadthSearch<'a, Key, Edge, Orientation> for Graph
where
    Key: 'a + Eq + Hash,
    Orientation: orientation::Orientation,
    Self: Neighbours<'a, Orientation, Key>,
{
    fn breadth_search(&'a self, from: &'a Key) -> Breadth<'a, Self, Key, Edge> {
        Breadth::new(self, from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::orientation::Directed;
    use crate::dev::simple::Simple;
    use crate::dev::AddVertex;
    use crate::wrapper::oriented::Oriented;

    #[test]
    fn simple_test() {
        let mut disconnected: Oriented<Simple<_, _, (), ()>, _> =
            Oriented::new(Simple::default(), Directed);
        let a = disconnected.add_vertex((0, ())).unwrap().clone();
        let b = disconnected.add_vertex((1, ())).unwrap().clone();

        assert_eq!(disconnected.breadth_search(&a).to(&b), None)
    }
}
*/
