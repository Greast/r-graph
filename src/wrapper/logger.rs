use crate::dev::orientation::AddEdge;
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge,
    RemoveVertex, Vertices,
};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc::Sender;
use std::sync::Mutex;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Entries<Orientation, VertexKey, Vertex, EdgeKey, Edge> {
    RemoveVertex(VertexKey),
    RemoveEdge(EdgeKey),
    Neighbours(Orientation, VertexKey),
    AddEdge(Orientation, VertexKey, VertexKey, Edge),
    AddVertex(Vertex),
    GetVertex(VertexKey),
    GetEdge(EdgeKey),
    GetEdgeTo(EdgeKey),
    Vertices(),
    Edges(),
}

struct Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> {
    graph: Graph,
    sender: Sender<Entries<Orientation, VertexKey, Vertex, EdgeKey, Edge>>,
}

impl<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> RemoveVertex<VertexKey>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Clone,
    Graph: RemoveVertex<VertexKey>,
{
    type Output = <Graph as RemoveVertex<VertexKey>>::Output;

    fn remove_vertex(&mut self, key: &VertexKey) -> Option<Self::Output> {
        self.sender.send(Entries::RemoveVertex(key.clone()));
        self.graph.remove_vertex(key)
    }
}

impl<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> RemoveEdge<EdgeKey>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    EdgeKey: Clone,
    Graph: RemoveEdge<EdgeKey>,
{
    type Output = <Graph as RemoveEdge<EdgeKey>>::Output;

    fn remove_edge(&mut self, key: &EdgeKey) -> Option<Self::Output> {
        self.sender.send(Entries::RemoveEdge(key.clone()));
        self.graph.remove_edge(key)
    }
}

impl<'a, Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
    Neighbours<'a, Orientation, VertexKey>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    Orientation: Default + orientation::Orientation,
    VertexKey: 'a + Clone,
    Graph: Neighbours<'a, Orientation, VertexKey>,
{
    type Edge = <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge;
    type IntoIter = <Graph as Neighbours<'a, Orientation, VertexKey>>::IntoIter;

    fn neighbours(&'a self, vertex: &VertexKey) -> Option<Self::IntoIter> {
        self.sender
            .send(Entries::Neighbours(Default::default(), vertex.clone()));
        self.graph.neighbours(vertex)
    }
}

impl<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> AddEdge<Orientation, VertexKey, Edge>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    Orientation: Default + orientation::Orientation,
    VertexKey: Clone,
    Edge: Clone,
    Graph: AddEdge<Orientation, VertexKey, Edge>,
{
    type EdgeKey = <Graph as AddEdge<Orientation, VertexKey, Edge>>::EdgeKey;

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        value: Edge,
    ) -> Result<Self::EdgeKey, Edge> {
        self.sender.send(Entries::AddEdge(
            Default::default(),
            from.clone(),
            to.clone(),
            value.clone(),
        ));
        self.graph.add_edge(from, to, value)
    }
}

impl<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> AddVertex<Vertex>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: AddVertex<Vertex>,
    Vertex: Clone,
{
    type Key = <Graph as AddVertex<Vertex>>::Key;

    fn add_vertex(&mut self, vertex: Vertex) -> Result<Self::Key, Vertex> {
        self.sender.send(Entries::AddVertex(vertex.clone()));
        self.graph.add_vertex(vertex)
    }
}

impl<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> GetVertex<VertexKey>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Clone,
    Graph: GetVertex<VertexKey>,
{
    type Output = <Graph as GetVertex<VertexKey>>::Output;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.sender.send(Entries::GetVertex(key.clone()));
        self.graph.get_vertex(key)
    }
}

impl<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> GetEdge<EdgeKey>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    EdgeKey: Clone,
    Graph: GetEdge<EdgeKey>,
{
    type Output = <Graph as GetEdge<EdgeKey>>::Output;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
        self.sender.send(Entries::GetEdge(key.clone()));
        self.graph.get_edge(key)
    }
}

impl<'a, Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> GetEdgeTo<'a, EdgeKey>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: GetEdgeTo<'a, EdgeKey>,
    EdgeKey: Clone,
{
    type Output = <Graph as GetEdgeTo<'a, EdgeKey>>::Output;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.sender.send(Entries::GetEdgeTo(key.clone()));
        self.graph.get_edge_to(key)
    }
}

impl<'a, Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> Vertices<'a>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: Vertices<'a>,
{
    type Item = <Graph as Vertices<'a>>::Item;
    type Output = <Graph as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.sender.send(Entries::Vertices());
        self.graph.vertices()
    }
}

impl<'a, Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge> Edges<'a>
    for Logger<Graph, Orientation, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: Edges<'a>,
{
    type Item = <Graph as Edges<'a>>::Item;
    type Output = <Graph as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.sender.send(Entries::Edges());
        self.graph.edges()
    }
}
