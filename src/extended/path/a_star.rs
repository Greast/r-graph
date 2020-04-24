use crate::dev::{orientation, GetEdge, GetVertex, Neighbours};
use crate::extended::header::Header;
use crate::extended::path::{DistanceFunctor, Path, PathFinder};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::AddAssign;

pub struct AStarFinder<'a, Graph, Function, Weight, VertexKey, EdgeKey, Orientation> {
    distance_functor: &'a DistanceFunctor<'a, Graph, Function>,
    from: &'a VertexKey,
    phantom: PhantomData<(Function, Weight, EdgeKey, Orientation)>,
}

impl<'a, Graph, Function, Weight, VertexKey, EdgeKey, Orientation>
    AStarFinder<'a, Graph, Function, Weight, VertexKey, EdgeKey, Orientation>
where
    VertexKey: 'a + Hash + Eq,
    EdgeKey: 'a,
    Orientation: orientation::Orientation,

    Graph: GetVertex<VertexKey>
        + GetEdge<EdgeKey>
        + Neighbours<'a, Orientation, VertexKey, Edge = &'a EdgeKey>,
    <Graph as GetEdge<EdgeKey>>::Output: Clone,
    Weight: Clone + Ord + Default + AddAssign + AddAssign<<Graph as GetEdge<EdgeKey>>::Output>,
    Function: Fn(
        &'a <Graph as GetVertex<VertexKey>>::Output,
        &'a <Graph as GetVertex<VertexKey>>::Output,
    ) -> Weight,
{
    pub fn finder(
        &mut self,
        mut to: &'a VertexKey,
    ) -> Option<Vec<(Weight, &'a VertexKey, &'a EdgeKey)>> {
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse(Header(Weight::default(), self.from)));

        let mut visited: HashMap<&'a VertexKey, (Weight, &'a VertexKey, &'a EdgeKey)> =
            HashMap::new();

        while let Some(Reverse(Header(mut weight, from))) = priority_queue.pop() {
            for (edge, to) in self
                .distance_functor
                .graph
                .neighbours(from)
                .into_iter()
                .flatten()
            {
                if let Some((w, _, _)) = visited.get(from) {
                    weight.add_assign(w.clone());
                }

                if let Some(w) = self.distance_functor.graph.get_edge(edge) {
                    weight.add_assign(w.clone());
                }

                weight.add_assign((self.distance_functor.dist)(
                    self.distance_functor.graph.get_vertex(from).unwrap(),
                    self.distance_functor.graph.get_vertex(to).unwrap(),
                ))
            }

            if from == to {
                break;
            }
        }

        let mut output = Vec::new();
        while let Some(path) = visited.remove(to) {
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
}

impl<'a, VertexKey, Graph, Function, Weight, EdgeKey, Orientation>
    PathFinder<
        'a,
        VertexKey,
        AStarFinder<'a, Graph, Function, Weight, VertexKey, EdgeKey, Orientation>,
    > for DistanceFunctor<'a, Graph, Function>
{
    fn path(
        &'a self,
        from: &'a VertexKey,
    ) -> AStarFinder<'a, Graph, Function, Weight, VertexKey, EdgeKey, Orientation> {
        AStarFinder {
            distance_functor: self,
            from,
            phantom: PhantomData,
        }
    }
}

impl<'a, VertexKey, EdgeKey, Graph, Function, Weight, Orientation> Path<'a, VertexKey, EdgeKey>
    for AStarFinder<'a, Graph, Function, Weight, VertexKey, EdgeKey, Orientation>
where
    VertexKey: 'a + Hash + Eq,
    EdgeKey: 'a,
    Orientation: orientation::Orientation,

    Graph: GetVertex<VertexKey>
        + GetEdge<EdgeKey>
        + Neighbours<'a, Orientation, VertexKey, Edge = &'a EdgeKey>,
    <Graph as GetEdge<EdgeKey>>::Output: Clone,
    Weight: Clone + Ord + Default + AddAssign + AddAssign<<Graph as GetEdge<EdgeKey>>::Output>,
    Function: Fn(
        &'a <Graph as GetVertex<VertexKey>>::Output,
        &'a <Graph as GetVertex<VertexKey>>::Output,
    ) -> Weight,
{
    type IntoIter = Vec<(&'a VertexKey, &'a EdgeKey)>;

    fn to(&mut self, to: &'a VertexKey) -> Option<Self::IntoIter> {
        self.finder(to)
            .map(|vec| vec.into_iter().map(|(_, v, e)| (v, e)).collect())
    }
}
