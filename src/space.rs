use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::simple::Simple;
use crate::dev::{orientation, Builder, Cyclic, Getter};
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

struct Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
{
    graph: Graph,
    values: HashMap<Vertex, VertexKey>,
}

impl<VertexKey, Vertex, Graph> Default for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
    Graph: Default,
{
    fn default() -> Self {
        Self {
            graph: Default::default(),
            values: Default::default(),
        }
    }
}

impl<VertexKey, Vertex, Graph> Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
{
    fn new(graph: Graph) -> Self {
        Self {
            graph,
            values: Default::default(),
        }
    }
}

impl<VertexKey, Vertex, Graph> Deref for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
{
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<VertexKey, Vertex, Graph> DerefMut for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<VertexKey, Vertex, Graph> Builder<Vertex> for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash + Clone,
    VertexKey: Eq + Hash + Clone,
    Graph: Builder<Vertex, VertexKey = VertexKey>,
{
    type VertexKey = Option<VertexKey>;

    fn add_vertex(&mut self, vertex: Vertex) -> Self::VertexKey {
        if self.values.contains_key(&vertex) {
            None
        } else {
            let id = self.deref_mut().add_vertex(vertex.clone());
            self.values.insert(vertex, id.clone());
            Some(id)
        }
    }
}

impl<Orientation, VertexKey, Vertex, Graph> Edge<Orientation> for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
    Graph: Edge<Orientation>,
    Orientation: orientation::Orientation,
    <Graph as Edge<Orientation>>::VertexKey: Eq + Hash + Clone,
    <Graph as Edge<Orientation>>::EdgeKey: Eq + Hash + Clone,
    Standard: Distribution<<Graph as Edge<Orientation>>::EdgeKey>,
{
    type VertexKey = <Graph as Edge<Orientation>>::VertexKey;
    type EdgeKey = <Graph as Edge<Orientation>>::EdgeKey;
    type Edge = <Graph as Edge<Orientation>>::Edge;

    fn add_edge(
        &mut self,
        from: &Self::VertexKey,
        to: &Self::VertexKey,
        value: Self::Edge,
    ) -> Self::EdgeKey {
        self.graph.add_edge(from, to, value)
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Graph> Getter<'a, VertexKey, EdgeKey>
    for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
    Graph: Getter<'a, VertexKey, EdgeKey>,
{
    type VertexReference = <Graph as Getter<'a, VertexKey, EdgeKey>>::VertexReference;
    type EdgeReference = <Graph as Getter<'a, VertexKey, EdgeKey>>::EdgeReference;

    fn get_vertex(&'a self, vertex: &VertexKey) -> Self::VertexReference {
        self.graph.get_vertex(vertex)
    }

    fn get_edge(&'a self, edge: &EdgeKey) -> Self::EdgeReference {
        self.graph.get_edge(edge)
    }
}

impl<Orientation, VertexKey, Vertex, Graph> Cyclic<Orientation> for Space<VertexKey, Vertex, Graph>
where
    Vertex: Eq + Hash,
    Orientation: orientation::Orientation,
    Graph: Cyclic<Orientation>,
{
    fn cyclic(&self) -> bool {
        self.graph.cyclic()
    }
}
