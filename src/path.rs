use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::{orientation, Builder, Cyclic, Getter, Reference, Neighbours, Travel};
use std::collections::HashMap;
use std::hash::Hash;

struct Path<VertexKey, EdgeKey, Edge, Graph>
where
    VertexKey: Eq + Hash,
    Edge: Eq + Hash,
{
    graph: Graph,
    paths: HashMap<VertexKey, HashMap<Edge, EdgeKey>>,
}

impl<VertexKey, Vertex, EdgeKey, Edge, Graph> Builder<Vertex>
    for Path<VertexKey, EdgeKey, Edge, Graph>
where
    VertexKey: Eq + Hash + Clone,
    Edge: Eq + Hash,
    Graph: Builder<Vertex, VertexKey = VertexKey>,
{
    type VertexKey = <Graph as Builder<Vertex>>::VertexKey;

    fn add_vertex(&mut self, vertex: Vertex) -> Self::VertexKey {
        let id = self.graph.add_vertex(vertex);
        self.paths.insert(id.clone(), Default::default());
        id
    }
}

impl<VertexKey, EdgeKey, Edge, Graph> orientation::Edge<Directed>
    for Path<VertexKey, EdgeKey, Edge, Graph>
where
    VertexKey: Eq + Hash + Clone,
    Edge: Eq + Hash + Clone,
    EdgeKey: Clone,
    Graph: orientation::Edge<Directed, VertexKey = VertexKey, EdgeKey = EdgeKey, Edge = Edge>,
{
    type VertexKey = VertexKey;
    type EdgeKey = Option<EdgeKey>;
    type Edge = Edge;

    fn add_edge(
        &mut self,
        from: &Self::VertexKey,
        to: &Self::VertexKey,
        value: Self::Edge,
    ) -> Self::EdgeKey {
        if self.paths.contains_key(from) && self.paths.contains_key(to) {
            let map = self.paths.get_mut(from).unwrap();
            if map.contains_key(&value) {
                None
            } else {
                let id = self.graph.add_edge(from, to, value.clone());
                map.insert(value, id.clone());
                Some(id)
            }
        } else {
            None
        }
    }
}

impl<VertexKey, EdgeKey, Edge, Graph> orientation::Edge<Undirected>
    for Path<VertexKey, EdgeKey, Edge, Graph>
where
    VertexKey: Eq + Hash + Clone,
    Edge: Eq + Hash + Clone,
    EdgeKey: Clone,
    Graph: orientation::Edge<Undirected, VertexKey = VertexKey, EdgeKey = EdgeKey, Edge = Edge>,
{
    type VertexKey = VertexKey;
    type EdgeKey = Option<EdgeKey>;
    type Edge = Edge;

    fn add_edge(
        &mut self,
        from: &Self::VertexKey,
        to: &Self::VertexKey,
        value: Self::Edge,
    ) -> Self::EdgeKey {
        if self.paths.contains_key(from) && self.paths.contains_key(to) {
            if self.paths.get_mut(from).unwrap().contains_key(&value)
                || self.paths.get_mut(to).unwrap().contains_key(&value)
            {
                None
            } else {
                let id = self.graph.add_edge(from, to, value.clone());
                self.paths
                    .get_mut(from)
                    .unwrap()
                    .insert(value.clone(), id.clone());
                self.paths.get_mut(to).unwrap().insert(value, id.clone());
                Some(id)
            }
        } else {
            None
        }
    }
}

struct VertexReference<'a, VertexKey, EdgeKey, Edge, Graph>
    where
        VertexKey: Eq + Hash,
        Edge: Eq + Hash,{
    key : VertexKey,
    graph : &'a Path<VertexKey, EdgeKey, Edge, Graph>
}

impl<'a, VertexKey, EdgeKey, Edge, Graph> Reference<'a> for VertexReference<'a, VertexKey, EdgeKey, Edge, Graph>
    where
        VertexKey: Eq + Hash,
        Edge: Eq + Hash,
        Graph : Getter<'a, VertexKey, EdgeKey>,
        <Graph as Getter<'a, VertexKey, EdgeKey>>::VertexReference : Reference<'a, Key = VertexKey>{
    type Key = &'a VertexKey;
    type Data = <<Graph as Getter<'a, VertexKey, EdgeKey>>::VertexReference as Reference<'a>>::Data;

    fn key(&'a self) -> Self::Key {
        &self.key
    }

    fn data(&'a self) -> Self::Data {
        self.graph.get_vertex(&self.key).data()
    }
}

impl<'a, VertexKey, EdgeKey, Edge, Graph, Orientation> Neighbours<Orientation> for VertexReference<'a, VertexKey, EdgeKey, Edge, Graph>
    where
        VertexKey: Eq + Hash,
        Edge: Eq + Hash,
        Orientation : orientation::Orientation,
        Graph : Getter<'a, VertexKey, EdgeKey>,
        <Graph as Getter<'a, VertexKey, EdgeKey>>::VertexReference : Neighbours<Orientation> + Reference<'a, Key = VertexKey>{

    type Edge = <<Graph as Getter<'a, VertexKey, EdgeKey>>::VertexReference as Neighbours<Orientation>>::Edge;
    type IntoIter = Vec<(Self::Edge, Self)>;

    fn neighbours(&self) -> Self::IntoIter {
        self.graph.get_vertex(&self.key)
            .neighbours()
            .into_iter()
            .map(|(edge, vertex)|(
                edge,
                Self{
                    key : vertex.key(),
                    graph : self.graph
                }
            )
            )
            .collect()
    }
}
impl<'a, VertexKey, EdgeKey, Edge, Graph> Getter<'a, VertexKey, EdgeKey>
    for Path<VertexKey, EdgeKey, Edge, Graph>
where
    VertexKey: Eq + Hash,
    Edge: Eq + Hash,
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

impl<Orientation, VertexKey, EdgeKey, Edge, Graph> Cyclic<Orientation>
    for Path<VertexKey, EdgeKey, Edge, Graph>
where
    VertexKey: Eq + Hash,
    Edge: Eq + Hash,
    Orientation: orientation::Orientation,
    Graph: Cyclic<Orientation>,
{
    fn cyclic(&self) -> bool {
        self.graph.cyclic()
    }
}
