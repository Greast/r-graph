use crate::dev::{orientation, Neighbours};
use crate::extended::path::{Path, PathFinder};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

///Dynamic programming version of breadth first search. Allows for efficient search of multiple end points.
pub struct Breadth<'a, Graph, Key, Edge, Orientation> {
    graph: &'a Graph,
    from: &'a Key,
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
    pub fn find(&self, mut to: &'a Key) -> Option<Vec<(&'a Key, &'a Edge)>> {
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
    Vertex: 'a + Eq + Hash,
    Edge: 'a,
{
    type IntoIter = Vec<(&'a Vertex, &'a Edge)>;

    fn to(&mut self, to: &'a Vertex) -> Option<Self::IntoIter> {
        if !self.visited.contains_key(to) {
            self.cache_path(to);
        }
        self.find(to)
    }
}

impl<'a, Graph, Vertex, Edge, Orientation>
    PathFinder<'a, Vertex, Breadth<'a, Graph, Vertex, Edge, Orientation>> for Graph
where
    Vertex: 'a + Eq + Hash,
{
    fn path(&'a self, from: &'a Vertex) -> Breadth<'a, Graph, Vertex, Edge, Orientation> {
        Breadth::new(self, from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::orientation::{AddEdge, Directed};
    use crate::dev::simple::Simple;
    use crate::dev::AddVertex;
    use crate::wrapper::oriented::{Orient, Oriented};

    #[test]
    fn disconnected() {
        let mut disconnected: Oriented<Simple<_, _, (), ()>, _> =
            Oriented::new(Simple::default(), Directed);

        let a = disconnected.add_vertex((0, ())).unwrap().clone();
        let b = disconnected.add_vertex((1, ())).unwrap().clone();

        let mut breadth: Breadth<_, _, _, _> = disconnected.path(&a);

        assert_eq!(breadth.to(&b), None);
    }

    #[test]
    fn single_edge() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", ()));

        let mut breadth: Breadth<_, _, _, _> = connected.path(&a);

        assert_eq!(breadth.to(&b), Some(vec![(&"V0", &"E0")]));
    }

    #[test]
    fn multiple_edges() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();
        let c = connected.add_vertex(("V2", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", ()));
        let _ = connected.add_edge(&b, &c, ("E1", ()));

        let mut breadth: Breadth<_, _, _, _> = connected.path(&a);

        assert_eq!(breadth.to(&c), Some(vec![(&"V0", &"E0"), (&"V1", &"E1")]));
    }

    #[test]
    fn complex_graph() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();
        let c = connected.add_vertex(("V2", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", ()));
        let _ = connected.add_edge(&b, &c, ("E1", ()));
        let _ = connected.add_edge(&a, &c, ("E2", ()));

        let mut breadth: Breadth<_, _, _, _> = connected.path(&a);

        assert_eq!(breadth.to(&c), Some(vec![(&"V0", &"E2")]));
    }

    #[test]
    fn multiple_paths() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();
        let c = connected.add_vertex(("V2", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", ()));
        let _ = connected.add_edge(&b, &c, ("E1", ()));

        let mut breadth: Breadth<_, _, _, _> = connected.path(&a);

        assert_eq!(breadth.to(&b), Some(vec![(&"V0", &"E0")]));

        assert_eq!(breadth.to(&c), Some(vec![(&"V0", &"E0"), (&"V1", &"E1")]));
    }
}
