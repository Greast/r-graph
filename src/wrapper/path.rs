use crate::dev::orientation::AddEdge;
use crate::dev::transform::{Collect, Map};
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours, RemoveEdge,
    RemoveVertex, Vertices,
};
use std::ops::{Deref, DerefMut};

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

impl<Key, Graph> GetVertex<Key> for Path<Graph>
where
    Graph: GetVertex<Key>,
{
    type Output = <Graph as GetVertex<Key>>::Output;

    fn get_vertex(&self, key: &Key) -> Option<&Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<Key, Graph> GetEdge<Key> for Path<Graph>
where
    Graph: GetEdge<Key>,
{
    type Output = <Graph as GetEdge<Key>>::Output;

    fn get_edge(&self, key: &Key) -> Option<&Self::Output> {
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

impl<'a, Graph> Vertices<'a> for Path<Graph>
where
    Graph: Vertices<'a>,
{
    type Item = <Graph as Vertices<'a>>::Item;
    type Output = <Graph as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl<'a, Graph> Edges<'a> for Path<Graph>
where
    Graph: Edges<'a>,
{
    type Item = <Graph as Edges<'a>>::Item;
    type Output = <Graph as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}

impl<Graph2, Graph> Merge<Path<Graph2>> for Path<Graph>
where
    Graph: Merge<Graph2>,
{
    type Output = <Graph as Merge<Graph2>>::Output;

    fn merge(self, other: Path<Graph2>) -> Result<Self::Output, (Self, Path<Graph2>)> {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x.into()),
            Err((x, y)) => Err((x.into(), y.into())),
        }
    }
}

impl<'a, Type, T, R, Func, Graph> Map<Type, T, R, Func> for Path<Graph>
where
    Graph: Map<Type, T, R, Func>,
{
    type Mapper = PathTransformer<<Graph as Map<Type, T, R, Func>>::Mapper>;

    fn map(self, func: Func) -> Self::Mapper {
        let transformer = self.graph.map(func);
        PathTransformer { transformer }
    }
}

pub struct PathTransformer<Trans> {
    transformer: Trans,
}

impl<Trans> Collect for PathTransformer<Trans>
where
    Trans: Collect,
{
    type Output = Path<<Trans as Collect>::Output>;

    fn collect(self) -> Option<Self::Output> {
        self.transformer.collect().map(Into::into)
    }
}

impl<'a, Type, T, R, Func, Trans> Map<Type, T, R, Func> for PathTransformer<Trans>
where
    Trans: Map<Type, T, R, Func>,
{
    type Mapper = PathTransformer<<Trans as Map<Type, T, R, Func>>::Mapper>;

    fn map(self, func: Func) -> Self::Mapper {
        let transformer = self.transformer.map(func);
        PathTransformer { transformer }
    }
}
