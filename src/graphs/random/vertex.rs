use crate::dev::orientation::Edge;
use crate::dev::{orientation, Builder, Edges, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex, Vertices, Merge};
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Vertex<Graph, VertexKey = usize> {
    graph: Graph,
    vertex_key: PhantomData<VertexKey>,
}

impl<Graph, VertexKey> Deref for Vertex<Graph, VertexKey> {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<Graph, VertexKey> DerefMut for Vertex<Graph, VertexKey> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<Graph, VertexKey> From<Graph> for Vertex<Graph, VertexKey> {
    fn from(graph: Graph) -> Self {
        Self {
            graph,
            vertex_key: Default::default(),
        }
    }
}

impl<Graph, VertexKey, Value> Builder<Value> for Vertex<Graph, VertexKey>
where
    Graph: Builder<(VertexKey, Value)>,
    Standard: Distribution<VertexKey>,
{
    type Key = <Graph as Builder<(VertexKey, Value)>>::Key;

    fn add_vertex(&mut self, value: Value) -> Result<Self::Key, Value> {
        let mut output = self.graph.add_vertex((random(), value));
        while let Err((_, value)) = output {
            output = self.graph.add_vertex((random(), value));
        }
        output.map_err(|x| x.1)
    }
}

impl<Graph, VertexKey, Value, Orientation> Edge<Orientation, VertexKey, Value>
    for Vertex<Graph, VertexKey>
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

impl<Graph, VertexKey> RemoveVertex<VertexKey> for Vertex<Graph, VertexKey>
where
    Graph: RemoveVertex<VertexKey>,
{
    type Output = <Graph as RemoveVertex<VertexKey>>::Output;

    fn remove_vertex(&mut self, key: &VertexKey) -> Option<Self::Output> {
        self.graph.remove_vertex(key)
    }
}

impl<Graph, EdgeKey, VertexKey> RemoveEdge<EdgeKey> for Vertex<Graph, VertexKey>
where
    Graph: RemoveEdge<EdgeKey>,
{
    type Output = <Graph as RemoveEdge<EdgeKey>>::Output;

    fn remove_edge(&mut self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.remove_edge(key)
    }
}

impl<'a, Graph, VertexKey> GetVertex<'a, VertexKey> for Vertex<Graph, VertexKey>
where
    Graph: GetVertex<'a, VertexKey>,
{
    type Output = <Graph as GetVertex<'a, VertexKey>>::Output;

    fn get_vertex(&'a self, key: &VertexKey) -> Option<Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<'a, Graph, EdgeKey, VertexKey> GetEdge<'a, EdgeKey> for Vertex<Graph, VertexKey>
where
    Graph: GetEdge<'a, EdgeKey>,
{
    type Output = <Graph as GetEdge<'a, EdgeKey>>::Output;

    fn get_edge(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.get_edge(key)
    }
}

impl<'a, Graph, EdgeKey, VertexKey> GetEdgeTo<'a, EdgeKey> for Vertex<Graph, VertexKey>
where
    Graph: GetEdgeTo<'a, EdgeKey>,
{
    type Output = <Graph as GetEdgeTo<'a, EdgeKey>>::Output;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.graph.get_edge_to(key)
    }
}

impl<'a, Graph, VertexKey, Orientation> Neighbours<'a, Orientation, VertexKey>
    for Vertex<Graph, VertexKey>
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

impl<'a, Key, Graph, VertexKey> Vertices<'a, Key> for Vertex<Graph, VertexKey>
where
    Key: 'a,
    Graph: Vertices<'a, Key>,
{
    type Output = <Graph as Vertices<'a, Key>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl<'a, Key, Graph, VertexKey> Edges<'a, Key> for Vertex<Graph, VertexKey>
where
    Key: 'a,
    Graph: Edges<'a, Key>,
{
    type Output = <Graph as Edges<'a, Key>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}

impl<Graph, VertexKey> Merge for Vertex<Graph, VertexKey>
    where
        Graph: Merge,
{
    fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x.into()),
            Err((x, y)) => Err((
                x.into(),
                y.into(),
            )),
        }
    }
}

