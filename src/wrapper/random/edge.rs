use crate::dev::orientation::AddEdge as EdgeTrait;
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours,
    RemoveEdge, RemoveVertex, Vertices,
};

use rand::distributions::{Distribution, Standard};
use rand::random;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::dev::transform::{Collect, Map};

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Edge<Graph, EdgeKey = usize> {
    graph: Graph,
    edge_key: PhantomData<EdgeKey>,
}

impl<Graph, EdgeKey> Deref for Edge<Graph, EdgeKey> {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<Graph, EdgeKey> DerefMut for Edge<Graph, EdgeKey> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<Graph, EdgeKey> From<Graph> for Edge<Graph, EdgeKey> {
    fn from(graph: Graph) -> Self {
        Self {
            graph,
            edge_key: Default::default(),
        }
    }
}

impl<Graph, EdgeKey, Value> AddVertex<Value> for Edge<Graph, EdgeKey>
where
    Graph: AddVertex<Value>,
{
    type Key = <Graph as AddVertex<Value>>::Key;

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
        while let Err((_, value)) = output {
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

impl<Graph, VertexKey, EdgeKey> GetVertex<VertexKey> for Edge<Graph, EdgeKey>
where
    Graph: GetVertex<VertexKey>,
{
    type Output = <Graph as GetVertex<VertexKey>>::Output;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<Graph, EdgeKey> GetEdge<EdgeKey> for Edge<Graph, EdgeKey>
where
    Graph: GetEdge<EdgeKey>,
{
    type Output = <Graph as GetEdge<EdgeKey>>::Output;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
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

impl<'a, Graph, EdgeKey> Vertices<'a> for Edge<Graph, EdgeKey>
where
    Graph: Vertices<'a>,
{
    type Item = <Graph as Vertices<'a>>::Item;
    type Output = <Graph as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl<'a, Graph, EdgeKey> Edges<'a> for Edge<Graph, EdgeKey>
where
    Graph: Edges<'a>,
{
    type Item = <Graph as Edges<'a>>::Item;
    type Output = <Graph as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}

impl<Graph2, Graph, EdgeKey> Merge<Edge<Graph2, EdgeKey>> for Edge<Graph, EdgeKey>
where
    Graph: Merge<Graph2>,
{
    type Output = <Graph as Merge<Graph2>>::Output;

    fn merge(
        self,
        other: Edge<Graph2, EdgeKey>,
    ) -> Result<Self::Output, (Self, Edge<Graph2, EdgeKey>)> {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x),
            Err((x, y)) => Err((x.into(), y.into())),
        }
    }
}

struct EdgeTransformer<Trans, EdgeKey>{
    transformer : Trans,
    phantom:PhantomData<(EdgeKey,)>
}

impl<Trans, EdgeKey> Collect for EdgeTransformer<Trans, EdgeKey>
    where
        Trans : Collect{
    type Output = Edge<<Trans as Collect>::Output, EdgeKey>;

    fn collect(self) -> Option<Self::Output> {
        Edge{
            graph: self.transformer.collect()?,
            edge_key: PhantomData
        }.into()
    }
}

impl <'a, Type, Func, Trans, EdgeKey> Map<'a, Type, EdgeKey, EdgeKey, Func> for EdgeTransformer<Trans, EdgeKey>
    where
        Trans : Map<'a, Type, EdgeKey, EdgeKey, Func>{
    type Mapper = EdgeTransformer<<Trans as Map<'a, Type, EdgeKey, EdgeKey, Func>>::Mapper, EdgeKey>;

    fn map(self, func: &'a Func) -> Self::Mapper {
        EdgeTransformer{
            transformer: self.transformer.map(func),
            phantom: PhantomData
        }
    }
}