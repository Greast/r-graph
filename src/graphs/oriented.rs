use crate::dev::{Builder, orientation, RemoveVertex, RemoveEdge, GetVertex, GetEdge, GetEdgeTo, Neighbours, Vertices, Edges};
use crate::dev::orientation::Edge;


pub trait Orient<Orientation>
    where Self : Sized{
    fn orient(self, orientation:Orientation) -> Oriented<Self, Orientation>;
}

impl<T, O> Orient<O> for T{
    fn orient(self, orientation: O) -> Oriented<Self, O> {
        Oriented{
            graph : self,
            orientation
        }
    }
}

///This graph fixes the edge orientation for the supplied graph.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Oriented<Graph, Orientation>{
    graph : Graph,
    orientation : Orientation,
}


impl<Graph, Orientation> Oriented<Graph, Orientation> {
    pub fn new(graph: Graph, orientation:Orientation) -> Self {
        Self {
            graph,
            orientation
        }
    }
}

impl<Graph, Orientation, Value> Builder<Value> for Oriented<Graph, Orientation>
    where
        Graph: Builder<Value>,
{
    type Key = <Graph as Builder<Value>>::Key;

    fn add_vertex(&mut self, value: Value) -> Result<Self::Key, Value> {
        self.graph.add_vertex(value)
    }
}

impl<Graph, VertexKey, Value, Orientation> Edge<Orientation, VertexKey, Value>
for Oriented<Graph, Orientation>
    where
        Orientation: orientation::Orientation,
        Graph: Edge<Orientation, VertexKey, Value>,
{
    type EdgeKey = <Graph as Edge<Orientation, VertexKey, Value>>::EdgeKey;

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

impl<'a, Graph, Orientation, VertexKey> GetVertex<'a, VertexKey> for Oriented<Graph, Orientation>
    where
        Graph: GetVertex<'a, VertexKey>,
{
    type Output = <Graph as GetVertex<'a, VertexKey>>::Output;

    fn get_vertex(&'a self, key: &VertexKey) -> Option<Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<'a, Graph, Orientation, EdgeKey> GetEdge<'a, EdgeKey> for Oriented<Graph, Orientation>
    where
        Graph: GetEdge<'a, EdgeKey>,
{
    type Output = <Graph as GetEdge<'a, EdgeKey>>::Output;

    fn get_edge(&'a self, key: &EdgeKey) -> Option<Self::Output> {
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

impl<'a, Graph, VertexKey, Orientation> Neighbours<'a, Orientation, VertexKey> for Oriented<Graph, Orientation>
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

impl <'a, Key, Graph, Orientation> Vertices<'a, Key> for Oriented<Graph, Orientation>
    where
        Key : 'a,
        Graph : Vertices<'a, Key>{
    type Output = <Graph as Vertices<'a, Key>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl <'a, Key, Graph, Orientation> Edges<'a, Key> for Oriented<Graph, Orientation>
    where
        Key : 'a,
        Graph : Edges<'a, Key>{
    type Output = <Graph as Edges<'a, Key>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}