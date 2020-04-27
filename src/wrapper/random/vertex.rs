use crate::dev::orientation::AddEdge;
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

impl<Graph2, Graph, VertexKey> Merge<Vertex<Graph2, VertexKey>> for Vertex<Graph, VertexKey>
where
    Graph: Merge<Graph2>,
{
    type Output = <Graph as Merge<Graph2>>::Output;
    fn merge(
        self,
        other: Vertex<Graph2, VertexKey>,
    ) -> Result<Self::Output, (Self, Vertex<Graph2, VertexKey>)> {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x.into()),
            Err((x, y)) => Err((x.into(), y.into())),
        }
    }
}

impl<'a, VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph> Merge
    for Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Vertex<Graph, VK>>
where
    Graph: 'a + Merge + GetVertex<VK>,
    Standard: Distribution<VK>,
    VKmap: Fn(VK) -> VK,
    Vertex<Graph, VK>: Transform<
        <VKmap as Dot<VK, VK, VK, Box<dyn 'a + Fn(VK) -> VK>>>::Output,
        Vmap,
        EKmap,
        Emap,
        Vertex<Graph, VK>,
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
    type Output = <Vertex<Graph, VK> as Merge>::Output;

    fn merge(self, other: Self) -> Result<Self::Output, (Self, Self)> {
        let rc = Rc::new(other);

        let rc1 = rc.clone();
        let closure: Box<dyn Fn(VK) -> VK> = Box::new(move |mut key| {
            while rc1.get_vertex(&key).is_some() {
                key = random();
            }
            key
        });

        let this: Vertex<Graph, VK> = self.map_vertex_key(closure).collect();
        let other = Rc::try_unwrap(rc).ok().unwrap();
        Ok(this.merge(other.graph).ok().unwrap())
    }
}
impl<VKmap, Vmap, EKmap, Emap, Graph, VertexKey, Graph2, VertexKey2>
    Transform<VKmap, Vmap, EKmap, Emap, Vertex<Graph, VertexKey>> for Vertex<Graph2, VertexKey2>
where
    Graph2: Transform<VKmap, Vmap, EKmap, Emap, Graph>,
{
    fn collect(graph: Vertex<Graph, VertexKey>, maps: (VKmap, Vmap, EKmap, Emap)) -> Self {
        Graph2::collect(graph.graph, maps).into()
    }
}
