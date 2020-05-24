use crate::dev::orientation::AddEdge;
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours, RemoveEdge,
    RemoveVertex, Vertices,
};
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::dev::transform::{transformers, Collect, Map};
use crate::wrapper::random::safe_map;
use std::hash::Hash;

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

impl<Graph, VertexKey, Value> AddVertex<Value> for Vertex<Graph, VertexKey>
where
    Graph: AddVertex<(VertexKey, Value)>,
    Standard: Distribution<VertexKey>,
{
    type Key = <Graph as AddVertex<(VertexKey, Value)>>::Key;

    fn add_vertex(&mut self, value: Value) -> Result<Self::Key, Value> {
        let mut output = self.graph.add_vertex((random(), value));
        while let Err((_, value)) = output {
            output = self.graph.add_vertex((random(), value));
        }
        output.map_err(|x| x.1)
    }
}

impl<Graph, VertexKey, Value, Orientation> AddEdge<Orientation, VertexKey, Value>
    for Vertex<Graph, VertexKey>
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

impl<Graph, VertexKey> GetVertex<VertexKey> for Vertex<Graph, VertexKey>
where
    Graph: GetVertex<VertexKey>,
{
    type Output = <Graph as GetVertex<VertexKey>>::Output;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<Graph, EdgeKey, VertexKey> GetEdge<EdgeKey> for Vertex<Graph, VertexKey>
where
    Graph: GetEdge<EdgeKey>,
{
    type Output = <Graph as GetEdge<EdgeKey>>::Output;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
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

impl<'a, Graph, VertexKey> Vertices<'a> for Vertex<Graph, VertexKey>
where
    Graph: Vertices<'a>,
{
    type Item = <Graph as Vertices<'a>>::Item;
    type Output = <Graph as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.graph.vertices()
    }
}

impl<'a, Graph, VertexKey> Edges<'a> for Vertex<Graph, VertexKey>
where
    Graph: Edges<'a>,
{
    type Item = <Graph as Edges<'a>>::Item;
    type Output = <Graph as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.graph.edges()
    }
}

impl<'a, Graph2, Graph, VertexKey> Merge<Vertex<Graph2, VertexKey>> for Vertex<Graph, VertexKey>
where
    VertexKey: 'a + Eq + Hash + Clone,
    Graph: Map<
        transformers::VertexKey,
        VertexKey,
        VertexKey,
        Box<dyn 'a + FnMut(VertexKey) -> VertexKey>,
    >,
    <Graph as Map<
        transformers::VertexKey,
        VertexKey,
        VertexKey,
        Box<dyn 'a + FnMut(VertexKey) -> VertexKey>,
    >>::Mapper: Collect<Output = Graph>,
    Graph: Merge<Graph2>,
    Standard: Distribution<VertexKey>,
    Graph2: 'a + GetEdge<VertexKey>,
{
    type Output = Vertex<<Graph as Merge<Graph2>>::Output, VertexKey>;

    fn merge(
        self,
        other: Vertex<Graph2, VertexKey>,
    ) -> Result<Self::Output, (Self, Vertex<Graph2, VertexKey>)> {
        safe_map(self.graph, other.graph)
            .map(Vertex::from)
            .map_err(|opt| {
                let (x, y) = opt.unwrap();
                (x.into(), y.into())
            })
    }
}

struct VertexTransformer<Trans, VertexKey> {
    transformer: Trans,
    phantom: PhantomData<(VertexKey,)>,
}

impl<Trans, VertexKey> Collect for VertexTransformer<Trans, VertexKey>
where
    Trans: Collect,
{
    type Output = Vertex<<Trans as Collect>::Output, VertexKey>;

    fn collect(self) -> Option<Self::Output> {
        Vertex {
            graph: self.transformer.collect()?,
            vertex_key: PhantomData,
        }
        .into()
    }
}

impl<'a, Type, Func, Trans, VertexKey> Map<Type, VertexKey, VertexKey, Func>
    for VertexTransformer<Trans, VertexKey>
where
    Trans: Map<Type, VertexKey, VertexKey, Func>,
{
    type Mapper =
        VertexTransformer<<Trans as Map<Type, VertexKey, VertexKey, Func>>::Mapper, VertexKey>;

    fn map(self, func: Func) -> Self::Mapper {
        VertexTransformer {
            transformer: self.transformer.map(func),
            phantom: PhantomData,
        }
    }
}
