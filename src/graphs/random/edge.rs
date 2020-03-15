use crate::dev::orientation::Edge as EdgeTrait;
use crate::dev::{orientation, Builder, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex, Edges, Vertices};
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Edge<Graph, EdgeKey = usize> {
    graph: Graph,
    edge_key: PhantomData<EdgeKey>,
}

impl<Graph, EdgeKey> From<Graph> for Edge<Graph, EdgeKey> {
    fn from(graph: Graph) -> Self {
        Self {
            graph,
            edge_key: Default::default(),
        }
    }
}

impl<Graph, EdgeKey, Value> Builder<Value> for Edge<Graph, EdgeKey>
where
    Graph: Builder<Value>,
{
    type Key = <Graph as Builder<Value>>::Key;

    fn add_vertex(&mut self, value: Value) -> Result<Self::Key, Value> {
        self.graph.add_vertex(value)
    }
}

impl<Graph, VertexKey, EdgeKey, Value, Orientation> EdgeTrait<Orientation, VertexKey, Value>
    for Edge<Graph, EdgeKey>
where
    Orientation: orientation::Orientation,
    Graph: EdgeTrait<Orientation, VertexKey, (EdgeKey, Value)>,
    Standard: Distribution<EdgeKey>,
{
    type EdgeKey = <Graph as EdgeTrait<Orientation, VertexKey, (EdgeKey, Value)>>::EdgeKey;

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        value: Value,
    ) -> Result<Self::EdgeKey, Value> {

        let mut output = self.graph.add_edge(from, to, (random(), value));
        while let Err((_, value)) = output{
            output = self.graph.add_edge(from, to, (random(), value));
        }
        output.map_err(|x| x.1)
    }
}

impl<Graph, VertexKey, EdgeKey> RemoveVertex<VertexKey> for Edge<Graph, EdgeKey>
where
    Graph: RemoveVertex<VertexKey>,
{
    type Output = <Graph as RemoveVertex<VertexKey>>::Output;

    fn remove_vertex(&mut self, key: &VertexKey) -> Option<Self::Output> {
        self.graph.remove_vertex(key)
    }
}

impl<Graph, EdgeKey> RemoveEdge<EdgeKey> for Edge<Graph, EdgeKey>
where
    Graph: RemoveEdge<EdgeKey>,
{
    type Output = <Graph as RemoveEdge<EdgeKey>>::Output;

    fn remove_edge(&mut self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.remove_edge(key)
    }
}

impl<'a, Graph, VertexKey, EdgeKey> GetVertex<'a, VertexKey> for Edge<Graph, EdgeKey>
where
    Graph: GetVertex<'a, VertexKey>,
{
    type Output = <Graph as GetVertex<'a, VertexKey>>::Output;

    fn get_vertex(&'a self, key: &VertexKey) -> Option<Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<'a, Graph, EdgeKey> GetEdge<'a, EdgeKey> for Edge<Graph, EdgeKey>
where
    Graph: GetEdge<'a, EdgeKey>,
{
    type Output = <Graph as GetEdge<'a, EdgeKey>>::Output;

    fn get_edge(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.get_edge(key)
    }
}

impl<'a, Graph, EdgeKey> GetEdgeTo<'a, EdgeKey> for Edge<Graph, EdgeKey>
where
    Graph: GetEdgeTo<'a, EdgeKey>,
{
    type Output = <Graph as GetEdgeTo<'a, EdgeKey>>::Output;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.get_edge_to(key)
    }
}

impl<'a, Graph, VertexKey, EdgeKey, Orientation> Neighbours<'a, Orientation, VertexKey>
    for Edge<Graph, EdgeKey>
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

impl <'a, Key, Graph, EdgeKey> Vertices<'a, Key> for Edge<Graph, EdgeKey>
    where
        Key : 'a,
        Graph : Vertices<'a, Key>{
    type Output = <Graph as Vertices<'a, Key>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl <'a, Key, Graph, EdgeKey> Edges<'a, Key> for Edge<Graph, EdgeKey>
    where
        Key : 'a,
        Graph : Edges<'a, Key>{
    type Output = <Graph as Edges<'a, Key>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}