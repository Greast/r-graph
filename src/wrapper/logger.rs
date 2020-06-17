use crate::dev::orientation::AddEdge;
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Merge, Neighbours, RemoveEdge,
    RemoveVertex, Vertices,
};

use crate::dev::transform::{Collect, Map};
use crate::wrapper::sender;
use std::marker::PhantomData;
use std::time::Instant;

pub trait Log<Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Self: Sized,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    fn log(self, sender: Sender) -> Logger<Self, Sender, VertexKey, Vertex, EdgeKey, Edge>;
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> Log<Sender, VertexKey, Vertex, EdgeKey, Edge>
    for Graph
where
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    fn log(self, sender: Sender) -> Logger<Self, Sender, VertexKey, Vertex, EdgeKey, Edge> {
        Logger {
            graph: self,
            sender,
            phantom: PhantomData,
        }
    }
}

pub enum Entries<VertexKey, Vertex, EdgeKey, Edge> {
    RemoveVertex(VertexKey),
    RemoveEdge(EdgeKey),
    Neighbours(Box<dyn orientation::Orientation>, VertexKey),
    AddEdge(
        Box<dyn orientation::Orientation>,
        VertexKey,
        VertexKey,
        Edge,
    ),
    AddVertex(Vertex),
    GetVertex(VertexKey),
    GetEdge(EdgeKey),
    GetEdgeTo(EdgeKey),
    Vertices(),
    Edges(),
}

pub struct Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> {
    graph: Graph,
    pub sender: Sender,
    phantom: PhantomData<(VertexKey, Vertex, EdgeKey, Edge)>,
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
    Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
{
    pub fn send(&self, entry: Entries<VertexKey, Vertex, EdgeKey, Edge>)
    where
        Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
    {
        #[allow(unused_must_use)]
        {
            self.sender.send(entry);
        }
    }
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> RemoveVertex<VertexKey>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Clone,
    Graph: RemoveVertex<VertexKey>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Output = <Graph as RemoveVertex<VertexKey>>::Output;

    fn remove_vertex(&mut self, key: &VertexKey) -> Option<Self::Output> {
        self.send(Entries::RemoveVertex(key.clone()));
        self.graph.remove_vertex(key)
    }
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> RemoveEdge<EdgeKey>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    EdgeKey: Clone,
    Graph: RemoveEdge<EdgeKey>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Output = <Graph as RemoveEdge<EdgeKey>>::Output;

    fn remove_edge(&mut self, key: &EdgeKey) -> Option<Self::Output> {
        self.send(Entries::RemoveEdge(key.clone()));
        self.graph.remove_edge(key)
    }
}

impl<'a, Graph, Orientation, Sender, VertexKey, Vertex, EdgeKey, Edge>
    Neighbours<'a, Orientation, VertexKey>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Orientation: 'static + Default + orientation::Orientation,
    VertexKey: 'a + Clone,
    Graph: Neighbours<'a, Orientation, VertexKey>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Edge = <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge;
    type IntoIter = <Graph as Neighbours<'a, Orientation, VertexKey>>::IntoIter;

    fn neighbours(&'a self, vertex: &VertexKey) -> Option<Self::IntoIter> {
        self.send(Entries::Neighbours(
            Box::new(Orientation::default()),
            vertex.clone(),
        ));
        self.graph.neighbours(vertex)
    }
}

impl<Graph, Orientation, Sender, VertexKey, Vertex, EdgeKey, Edge>
    AddEdge<Orientation, VertexKey, Edge>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Orientation: 'static + Default + orientation::Orientation,
    VertexKey: Clone,
    Edge: Clone,
    Graph: AddEdge<Orientation, VertexKey, Edge>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type EdgeKey = <Graph as AddEdge<Orientation, VertexKey, Edge>>::EdgeKey;

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        value: Edge,
    ) -> Result<Self::EdgeKey, Edge> {
        self.send(Entries::AddEdge(
            Box::new(Orientation::default()),
            from.clone(),
            to.clone(),
            value.clone(),
        ));
        self.graph.add_edge(from, to, value)
    }
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> AddVertex<Vertex>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: AddVertex<Vertex>,
    Vertex: Clone,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Key = <Graph as AddVertex<Vertex>>::Key;

    fn add_vertex(&mut self, vertex: Vertex) -> Result<Self::Key, Vertex> {
        self.send(Entries::AddVertex(vertex.clone()));
        self.graph.add_vertex(vertex)
    }
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> GetVertex<VertexKey>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Clone,
    Graph: GetVertex<VertexKey>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Output = <Graph as GetVertex<VertexKey>>::Output;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.send(Entries::GetVertex(key.clone()));
        self.graph.get_vertex(key)
    }
}

impl<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> GetEdge<EdgeKey>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    EdgeKey: Clone,
    Graph: GetEdge<EdgeKey>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Output = <Graph as GetEdge<EdgeKey>>::Output;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
        self.send(Entries::GetEdge(key.clone()));
        self.graph.get_edge(key)
    }
}

impl<'a, Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> GetEdgeTo<'a, EdgeKey>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: GetEdgeTo<'a, EdgeKey>,
    EdgeKey: Clone,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Output = <Graph as GetEdgeTo<'a, EdgeKey>>::Output;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.send(Entries::GetEdgeTo(key.clone()));
        self.graph.get_edge_to(key)
    }
}

impl<'a, Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> Vertices<'a>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: Vertices<'a>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Item = <Graph as Vertices<'a>>::Item;
    type Output = <Graph as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.send(Entries::Vertices());
        self.graph.vertices()
    }
}

impl<'a, Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> Edges<'a>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: Edges<'a>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Item = <Graph as Edges<'a>>::Item;
    type Output = <Graph as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.send(Entries::Edges());
        self.graph.edges()
    }
}
impl<Graph2, Graph, Sender, Sender2, VertexKey, Vertex, EdgeKey, Edge>
    Merge<Logger<Graph2, Sender2, VertexKey, Vertex, EdgeKey, Edge>>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: Merge<Graph2>,
    Sender: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
    Sender2: sender::Sender<Entries<VertexKey, Vertex, EdgeKey, Edge>>,
{
    type Output = <Graph as Merge<Graph2>>::Output;

    fn merge(
        self,
        other: Logger<Graph2, Sender2, VertexKey, Vertex, EdgeKey, Edge>,
    ) -> Result<
        Self::Output,
        (
            Self,
            Logger<Graph2, Sender2, VertexKey, Vertex, EdgeKey, Edge>,
        ),
    > {
        let output = self.graph.merge(other.graph);
        match output {
            Ok(x) => Ok(x),
            Err((x, y)) => Err((x.log(self.sender), y.log(other.sender))),
        }
    }
}

impl<'a, Type, T, Func, Graph, Sender, VertexKey, Vertex, EdgeKey, Edge> Map<Type, T, T, Func>
    for Logger<Graph, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Graph: Map<Type, T, T, Func>,
{
    type Mapper = LoggerTransformer<
        <Graph as Map<Type, T, T, Func>>::Mapper,
        Sender,
        VertexKey,
        Vertex,
        EdgeKey,
        Edge,
    >;

    fn map(self, func: Func) -> Self::Mapper {
        LoggerTransformer {
            transformer: self.graph.map(func),
            sender: self.sender,
            phantom: PhantomData,
        }
    }
}

pub struct LoggerTransformer<Trans, Sender, VertexKey, Vertex, EdgeKey, Edge> {
    transformer: Trans,
    sender: Sender,
    phantom: PhantomData<(VertexKey, Vertex, EdgeKey, Edge)>,
}

impl<Trans, Sender, VertexKey, Vertex, EdgeKey, Edge> Collect
    for LoggerTransformer<Trans, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Trans: Collect,
{
    type Output = Logger<<Trans as Collect>::Output, Sender, VertexKey, Vertex, EdgeKey, Edge>;

    fn collect(self) -> Option<Self::Output> {
        Logger {
            graph: self.transformer.collect()?,
            sender: self.sender,
            phantom: PhantomData,
        }
        .into()
    }
}

impl<'a, Type, T, Func, Trans, Sender, VertexKey, Vertex, EdgeKey, Edge> Map<Type, T, T, Func>
    for LoggerTransformer<Trans, Sender, VertexKey, Vertex, EdgeKey, Edge>
where
    Trans: Map<Type, T, T, Func>,
{
    type Mapper = LoggerTransformer<
        <Trans as Map<Type, T, T, Func>>::Mapper,
        Sender,
        VertexKey,
        Vertex,
        EdgeKey,
        Edge,
    >;

    fn map(self, func: Func) -> Self::Mapper {
        LoggerTransformer {
            transformer: self.transformer.map(func),
            sender: self.sender,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::orientation::Directed;
    use crate::dev::simple::Simple;
    use crate::wrapper::oriented::Orient;
    use std::sync::mpsc::channel;

    #[test]
    fn add_vertex() {
        let (sender, receiver) = channel();
        let simple: Simple<_, _, usize, ()> = Simple::default();
        let mut graph: Logger<_, _, usize, _, usize, ()> = simple.log(sender);

        graph.add_vertex((0, ()));

        assert!(match receiver.recv().unwrap() {
            Entries::AddVertex((0, ())) => true,
            _ => false,
        });
    }

    #[test]
    fn add_edge() {
        let (sender, receiver) = channel();
        let simple: Simple<_, _, _, _> = Simple::default();
        let log: Logger<_, _, usize, _, usize, _> = simple.log(sender);
        let mut graph = log.orient(Directed);

        let a = graph.add_vertex((0, ())).unwrap();
        let b = graph.add_vertex((1, ())).unwrap();

        let _edge = graph.add_edge(&a, &b, ("", ()));

        assert!(match receiver.recv().unwrap() {
            Entries::AddVertex((0, ())) => true,
            _ => false,
        });
        assert!(match receiver.recv().unwrap() {
            Entries::AddVertex((1, ())) => true,
            _ => false,
        });
        assert!(match receiver.recv().unwrap() {
            Entries::AddEdge(_, _a, _b, ("", ())) => true,
            _ => false,
        });
    }
}
