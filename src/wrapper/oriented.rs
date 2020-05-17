use crate::dev::orientation::AddEdge;
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours, RemoveEdge,
    RemoveVertex, Vertices,
};
use std::ops::{Deref, DerefMut};
use crate::dev::transform::{Collect, Map};

pub trait Orient<Orientation>
where
    Self: Sized,
{
    fn orient(self, orientation: Orientation) -> Oriented<Self, Orientation>;
}

impl<T, O> Orient<O> for T {
    fn orient(self, orientation: O) -> Oriented<Self, O> {
        Oriented {
            graph: self,
            orientation,
        }
    }
}

///This graph fixes the edge orientation for the supplied graph.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Oriented<Graph, Orientation> {
    graph: Graph,
    orientation: Orientation,
}

impl<Graph, Orientation> Deref for Oriented<Graph, Orientation> {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<Graph, Orientation> DerefMut for Oriented<Graph, Orientation> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<Graph, Orientation> Oriented<Graph, Orientation> {
    pub fn new(graph: Graph, orientation: Orientation) -> Self {
        Self { graph, orientation }
    }
}

impl<Graph, Orientation, Value> AddVertex<Value> for Oriented<Graph, Orientation>
where
    Graph: AddVertex<Value>,
{
    type Key = <Graph as AddVertex<Value>>::Key;

    fn add_vertex(&mut self, value: Value) -> Result<Self::Key, Value> {
        self.graph.add_vertex(value)
    }
}

impl<Graph, VertexKey, Value, Orientation> AddEdge<Orientation, VertexKey, Value>
    for Oriented<Graph, Orientation>
where
    Orientation: orientation::Orientation,
    Graph: AddEdge<Orientation, VertexKey, Value>,
{
    type EdgeKey = <Graph as AddEdge<Orientation, VertexKey, Value>>::EdgeKey;

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        value: Value,
    ) -> Result<Self::EdgeKey, Value> {
        self.graph.add_edge(from, to, value)
    }
}

impl<Graph, Orientation, VertexKey> RemoveVertex<VertexKey> for Oriented<Graph, Orientation>
where
    Graph: RemoveVertex<VertexKey>,
{
    type Output = <Graph as RemoveVertex<VertexKey>>::Output;

    fn remove_vertex(&mut self, key: &VertexKey) -> Option<Self::Output> {
        self.graph.remove_vertex(key)
    }
}

impl<Graph, Orientation, EdgeKey> RemoveEdge<EdgeKey> for Oriented<Graph, Orientation>
where
    Graph: RemoveEdge<EdgeKey>,
{
    type Output = <Graph as RemoveEdge<EdgeKey>>::Output;

    fn remove_edge(&mut self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.remove_edge(key)
    }
}

impl<Graph, Orientation, VertexKey> GetVertex<VertexKey> for Oriented<Graph, Orientation>
where
    Graph: GetVertex<VertexKey>,
{
    type Output = <Graph as GetVertex<VertexKey>>::Output;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<Graph, Orientation, EdgeKey> GetEdge<EdgeKey> for Oriented<Graph, Orientation>
where
    Graph: GetEdge<EdgeKey>,
{
    type Output = <Graph as GetEdge<EdgeKey>>::Output;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
        self.graph.get_edge(key)
    }
}

impl<'a, Graph, Orientation, EdgeKey> GetEdgeTo<'a, EdgeKey> for Oriented<Graph, Orientation>
where
    Graph: GetEdgeTo<'a, EdgeKey>,
{
    type Output = <Graph as GetEdgeTo<'a, EdgeKey>>::Output;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.get_edge_to(key)
    }
}

impl<'a, Graph, VertexKey, Orientation> Neighbours<'a, Orientation, VertexKey>
    for Oriented<Graph, Orientation>
where
    VertexKey: 'a,
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, VertexKey>,
{
    type Edge = <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge;
    type IntoIter = <Graph as Neighbours<'a, Orientation, VertexKey>>::IntoIter;

    fn neighbours(&'a self, key: &VertexKey) -> Option<Self::IntoIter> {
        self.graph.neighbours(key)
    }
}

impl<'a, Graph, Orientation> Vertices<'a> for Oriented<Graph, Orientation>
where
    Graph: Vertices<'a>,
{
    type Item = <Graph as Vertices<'a>>::Item;
    type Output = <Graph as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl<'a, Graph, Orientation> Edges<'a> for Oriented<Graph, Orientation>
where
    Graph: Edges<'a>,
{
    type Item = <Graph as Edges<'a>>::Item;
    type Output = <Graph as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}

impl<Graph2, Graph, Orientation> Merge<Oriented<Graph2, Orientation>>
    for Oriented<Graph, Orientation>
where
    Graph: Merge<Graph2>,
{
    type Output = <Graph as Merge<Graph2>>::Output;

    fn merge(
        self,
        other: Oriented<Graph2, Orientation>,
    ) -> Result<Self::Output, (Self, Oriented<Graph2, Orientation>)> {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x),
            Err((x, y)) => Err((x.orient(self.orientation), y.orient(other.orientation))),
        }
    }
}

struct OrientedTransformer<Trans, Orientation>{
    transformer : Trans,
    orientation : Orientation,
}

impl<Trans, Orientation> Collect for OrientedTransformer<Trans, Orientation>
    where
        Trans : Collect{
    type Output = Oriented<<Trans as Collect>::Output, Orientation>;

    fn collect(self) -> Option<Self::Output> {
        let orientation = self.orientation;
        self.transformer.collect().map(move |x|x.orient(orientation))
    }
}

impl<'a, Type, T, R, Func, Trans, Orientation> Map<'a, Type, T, R, Func> for OrientedTransformer<Trans, Orientation>
    where
        Trans : Map<'a, Type, T, R, Func>{
    type Mapper = OrientedTransformer<<Trans as Map<'a, Type, T, R, Func>>::Mapper, Orientation>;

    fn map(self, func: Func) -> Self::Mapper {
        let transformer = self.transformer.map(func);
        OrientedTransformer{
            transformer,
            orientation: self.orientation
        }
    }
}