use crate::dev::{orientation, GetEdge, Neighbours};
use crate::extended::header::Header;
use crate::extended::path::{Path, PathFinder};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::AddAssign;

///Dynamic programming version of dijkstras algorithm. Allows for efficient search of multiple end points.
struct Dijkstra<'a, Graph, Vertex, Edge, Weight, Orientation> {
    from: &'a Vertex,
    graph: &'a Graph,
    visited: HashMap<&'a Vertex, (Weight, &'a Vertex, &'a Edge)>,
    queue: BinaryHeap<Reverse<Header<Weight, &'a Vertex>>>,
    orientation: PhantomData<Orientation>,
}

impl<'a, Graph, Vertex, Edge, Weight, Orientation>
    Dijkstra<'a, Graph, Vertex, Edge, Weight, Orientation>
where
    Vertex: Eq + Hash,
{
    pub fn new(graph: &'a Graph, from: &'a Vertex) -> Self
    where
        Weight: 'a + Ord + Clone + Default,
        Orientation: orientation::Orientation,
        Graph:
            Neighbours<'a, Orientation, Vertex, Edge = &'a Edge> + GetEdge<Edge, Output = Weight>,
    {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse(Header(Default::default(), from)));

        let visited = HashMap::new();

        Self {
            from,
            graph,
            visited,
            queue,
            orientation: Default::default(),
        }
    }

    pub fn finder(&self, mut to: &'a Vertex) -> Option<Vec<&(Weight, &'a Vertex, &'a Edge)>> {
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

    fn cache_path(&mut self, to: &Vertex)
    where
        Orientation: orientation::Orientation,
        Graph: Neighbours<'a, Orientation, Vertex, Edge = &'a Edge> + GetEdge<Edge>,
        <Graph as GetEdge<Edge>>::Output: Clone,
        Weight: Ord + Clone + AddAssign + AddAssign<<Graph as GetEdge<Edge>>::Output>,
    {
        while let Some(Reverse(Header(mut weight, from))) = self.queue.pop() {
            for (edge, to) in self.graph.neighbours(from).into_iter().flatten() {
                if let Some((w, _, _)) = self.visited.get(from) {
                    weight.add_assign(w.clone());
                }

                if let Some(w) = self.graph.get_edge(edge) {
                    weight.add_assign(w.clone());
                }

                if !self.visited.contains_key(to) {
                    self.queue.push(Reverse(Header(weight.clone(), to)));
                }

                self.visited.insert(to, (weight.clone(), from, edge));
            }
            if let Some((max_weight, _, _)) = self.visited.get(to) {
                if let Some(Reverse(Header(weight, _))) = self.queue.peek() {
                    if max_weight < weight {
                        break;
                    }
                }
            }
        }
    }
}

impl<'a, Vertex, Edge, Graph, Weight, Orientation> Path<'a, Vertex, Edge>
    for Dijkstra<'a, Graph, Vertex, Edge, Weight, Orientation>
where
    Vertex: Eq + Hash,
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, Vertex, Edge = &'a Edge> + GetEdge<Edge>,
    <Graph as GetEdge<Edge>>::Output: Clone,
    Weight: Ord + Clone + AddAssign + AddAssign<<Graph as GetEdge<Edge>>::Output>,
{
    type IntoIter = Vec<(&'a Vertex, &'a Edge)>;

    fn to(&mut self, to: &'a Vertex) -> Option<Self::IntoIter> {
        if !self.visited.contains_key(to) {
            self.cache_path(to);
        }
        self.finder(to)
            .map(|vec| vec.into_iter().map(|(_, v, e)| (*v, *e)).collect())
    }
}

impl<'a, Graph, Vertex, Edge, Weight, Orientation>
    PathFinder<'a, Vertex, Dijkstra<'a, Graph, Vertex, Edge, Weight, Orientation>> for Graph
where
    Weight: 'a + Ord + Clone + Default,
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, Vertex, Edge = &'a Edge> + GetEdge<Edge, Output = Weight>,
    Vertex: 'a + Eq + Hash,
{
    fn path(&'a self, from: &'a Vertex) -> Dijkstra<'a, Graph, Vertex, Edge, Weight, Orientation> {
        Dijkstra::new(self, from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::orientation::{AddEdge, Directed};
    use crate::dev::simple::Simple;
    use crate::dev::AddVertex;
    use crate::extended::path::Path;
    use crate::wrapper::oriented::{Orient, Oriented};

    #[test]
    fn disconnected() {
        let mut disconnected: Oriented<Simple<_, _, i32, i32>, _> =
            Oriented::new(Simple::default(), Directed);

        let a = disconnected.add_vertex((0, 10)).unwrap().clone();
        let b = disconnected.add_vertex((1, 11)).unwrap().clone();

        let mut path: Dijkstra<_, _, _, _, _> = disconnected.path(&a);
        assert_eq!(path.to(&b), None);
    }

    #[test]
    fn single_edge() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", 0));

        let mut path: Dijkstra<_, _, _, _, _> = connected.path(&a);

        assert_eq!(path.to(&b), Some(vec![(&"V0", &"E0")]));
    }

    #[test]
    fn multiple_edges() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();
        let c = connected.add_vertex(("V2", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", 0));
        let _ = connected.add_edge(&b, &c, ("E1", 1));

        let mut path: Dijkstra<_, _, _, _, _> = connected.path(&a);

        assert_eq!(path.to(&c), Some(vec![(&"V0", &"E0"), (&"V1", &"E1")]));
    }

    #[test]
    fn complex_graph() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();
        let c = connected.add_vertex(("V2", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", 0));
        let _ = connected.add_edge(&b, &c, ("E1", 0));
        let _ = connected.add_edge(&a, &c, ("E2", 3));

        let mut path: Dijkstra<_, _, _, _, _> = connected.path(&a);

        assert_eq!(path.to(&c), Some(vec![(&"V0", &"E0"), (&"V1", &"E1")]));
    }

    #[test]
    fn multiple_paths() {
        let mut connected = Simple::default().orient(Directed);

        let a = connected.add_vertex(("V0", ())).unwrap().clone();
        let b = connected.add_vertex(("V1", ())).unwrap().clone();
        let c = connected.add_vertex(("V2", ())).unwrap().clone();

        let _ = connected.add_edge(&a, &b, ("E0", 0));
        let _ = connected.add_edge(&b, &c, ("E1", 1));

        let mut path: Dijkstra<_, _, _, _, _> = connected.path(&a);

        assert_eq!(path.to(&b), Some(vec![(&"V0", &"E0")]));

        assert_eq!(path.to(&c), Some(vec![(&"V0", &"E0"), (&"V1", &"E1")]));
    }
}
