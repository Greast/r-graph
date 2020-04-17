use crate::dev::orientation::AddEdge;
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours, RemoveEdge,
    RemoveVertex, Vertices,
};
use std::ops::{Deref, DerefMut};
use crate::dev::transform::Transform;

///The edges for this graph are hashed along with its from-node, allowing for wrapper such as those found in deterministic automaton.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Path<Graph> {
    graph: Graph,
}

impl<Graph> From<Graph> for Path<Graph> {
    fn from(graph: Graph) -> Self {
        Self { graph }
    }
}

impl<Graph> Deref for Path<Graph> {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<Graph> DerefMut for Path<Graph> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<Graph, Input> AddVertex<Input> for Path<Graph>
where
    Graph: AddVertex<Input>,
{
    type Key = <Graph as AddVertex<Input>>::Key;

    fn add_vertex(&mut self, vertex: Input) -> Result<Self::Key, Input> {
        self.graph.add_vertex(vertex)
    }
}

impl<Orientation, Graph, VertexKey, EdgeKey, Value>
    AddEdge<Orientation, VertexKey, (EdgeKey, Value)> for Path<Graph>
where
    Orientation: orientation::Orientation,
    Graph: AddEdge<
        Orientation,
        VertexKey,
        ((VertexKey, EdgeKey), Value),
        EdgeKey = (VertexKey, EdgeKey),
    >,
    VertexKey: Clone,
{
    type EdgeKey = (VertexKey, EdgeKey);

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        (key, value): (EdgeKey, Value),
    ) -> Result<Self::EdgeKey, (EdgeKey, Value)> {
        self.graph
            .add_edge(from, to, ((from.clone(), key), value))
            .map_err(|((_, x), y)| (x, y))
    }
}

impl<Key, Graph> RemoveVertex<Key> for Path<Graph>
where
    Graph: RemoveVertex<Key>,
{
    type Output = <Graph as RemoveVertex<Key>>::Output;

    fn remove_vertex(&mut self, key: &Key) -> Option<Self::Output> {
        self.graph.remove_vertex(key)
    }
}

impl<Key, Graph> RemoveEdge<Key> for Path<Graph>
where
    Graph: RemoveEdge<Key>,
{
    type Output = <Graph as RemoveEdge<Key>>::Output;

    fn remove_edge(&mut self, key: &Key) -> Option<Self::Output> {
        self.graph.remove_edge(key)
    }
}

impl<'a, Key, Graph> GetVertex<'a, Key> for Path<Graph>
where
    Graph: GetVertex<'a, Key>,
{
    type Output = <Graph as GetVertex<'a, Key>>::Output;

    fn get_vertex(&'a self, key: &Key) -> Option<Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<'a, Key, Graph> GetEdge<'a, Key> for Path<Graph>
where
    Graph: GetEdge<'a, Key>,
{
    type Output = <Graph as GetEdge<'a, Key>>::Output;

    fn get_edge(&'a self, key: &Key) -> Option<Self::Output> {
        self.graph.get_edge(key)
    }
}

impl<'a, Key, Graph> GetEdgeTo<'a, Key> for Path<Graph>
where
    Graph: GetEdgeTo<'a, Key>,
{
    type Output = <Graph as GetEdgeTo<'a, Key>>::Output;

    fn get_edge_to(&'a self, key: &Key) -> Option<Self::Output> {
        self.graph.get_edge_to(key)
    }
}

impl<'a, Orientation, Key, Graph> Neighbours<'a, Orientation, Key> for Path<Graph>
where
    Key: 'a,
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, Key>,
{
    type Edge = <Graph as Neighbours<'a, Orientation, Key>>::Edge;
    type IntoIter = <Graph as Neighbours<'a, Orientation, Key>>::IntoIter;

    fn neighbours(&'a self, key: &Key) -> Option<Self::IntoIter> {
        self.graph.neighbours(key)
    }
}

impl<'a, Key, Graph> Vertices<'a, Key> for Path<Graph>
where
    Key: 'a,
    Graph: Vertices<'a, Key>,
{
    type Output = <Graph as Vertices<'a, Key>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl<'a, Key, Graph> Edges<'a, Key> for Path<Graph>
where
    Key: 'a,
    Graph: Edges<'a, Key>,
{
    type Output = <Graph as Edges<'a, Key>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}

impl<Graph> Merge for Path<Graph>
where
    Graph: Merge,
{
    fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x.into()),
            Err((x, y)) => Err((x.into(), y.into())),
        }
    }
}

impl<VKmap, Vmap, EKmap, Emap, Graph, Graph2>
Transform<VKmap, Vmap, EKmap, Emap, Path<Graph>>
for Path<Graph2>
    where
        Graph2 : Transform<VKmap, Vmap, EKmap, Emap, Graph>{
    fn collect(graph: Path<Graph>, function: (VKmap, Vmap, EKmap, Emap)) -> Self {
        Graph2::collect(graph.graph, function).into()
    }
}
