use crate::dev::orientation::AddEdge as EdgeTrait;
use crate::dev::transform::{Transform, Transformer};
use crate::dev::{
    orientation, AddVertex, Dot, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours,
    RemoveEdge, RemoveVertex, Vertices,
};

use rand::distributions::{Distribution, Standard};
use rand::random;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

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

impl<'a, Key, Graph, EdgeKey> Edges<'a, Key> for Edge<Graph, EdgeKey>
where
    Key: 'a,
    Graph: Edges<'a, Key>,
{
    type Output = <Graph as Edges<'a, Key>>::Output;

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
            Ok(x) => Ok(x.into()),
            Err((x, y)) => Err((x.into(), y.into())),
        }
    }
}

impl<'a, VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph> Merge
    for Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Edge<Graph, EK>>
where
    Graph: 'a + Merge + GetEdge<EK>,
    Standard: Distribution<EK>,
    EKmap: Fn(EK) -> EK,
    Edge<Graph, EK>: Transform<
        VKmap,
        Vmap,
        <EKmap as Dot<EK, EK, EK, Box<dyn 'a + Fn(EK) -> EK>>>::Output,
        Emap,
        Edge<Graph, EK>,
    >,
    EK: 'a,
    E: 'a,
    V: 'a,
    VK: 'a,
    Emap: 'a,
    EKmap: 'a,
    Vmap: 'a,
    VKmap: 'a,
{
    type Output = <Edge<Graph, EK> as Merge>::Output;

    fn merge(self, other: Self) -> Result<Self::Output, (Self, Self)> {
        let rc = Rc::new(other);

        let rc1 = rc.clone();
        let closure: Box<dyn Fn(EK) -> EK> = Box::new(move |mut key| {
            while rc1.get_edge(&key).is_some() {
                key = random();
            }
            key
        });

        let this: Edge<Graph, EK> = self.map_edge_key(closure).collect();
        let other = Rc::try_unwrap(rc).ok().unwrap();
        Ok(this.merge(other.graph).ok().unwrap())
    }
}

impl<VKmap, Vmap, EKmap, Emap, Graph, EdgeKey, Graph2, EdgeKey2>
    Transform<VKmap, Vmap, EKmap, Emap, Edge<Graph, EdgeKey>> for Edge<Graph2, EdgeKey2>
where
    Graph2: Transform<VKmap, Vmap, EKmap, Emap, Graph>,
{
    fn collect(graph: Edge<Graph, EdgeKey>, maps: (VKmap, Vmap, EKmap, Emap)) -> Self {
        Graph2::collect(graph.graph, maps).into()
    }
}
