use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::{Builder, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Node<Data, From, To> {
    pub data: Data,
    pub from: From,
    pub to: To,
}

///A simple graph implementation, where the key for each edge and vertex has to be supplied.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    pub vertices: HashMap<VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>>,
    pub edges: HashMap<EdgeKey, Node<Edge, VertexKey, VertexKey>>,
}

impl<VertexKey, Vertex, EdgeKey, Edge> Builder<(VertexKey, Vertex)>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + Clone,
    EdgeKey: Eq + Hash + Clone,
{
    type Key = VertexKey;

    fn add_vertex(&mut self, (key, data): (VertexKey, Vertex)) -> Option<Self::Key> {
        if self.vertices.contains_key(&key) {
            None
        } else {
            self.vertices.insert(
                key.clone(),
                Node {
                    data,
                    from: Default::default(),
                    to: Default::default(),
                },
            );
            Some(key)
        }
    }
}

impl<Vk, V, Ek, E> Edge<Directed, Vk, (Ek, E)> for Simple<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
{
    type EdgeKey = Ek;

    fn add_edge(&mut self, from: &Vk, to: &Vk, (key, data): (Ek, E)) -> Option<Self::EdgeKey> {
        if !self.vertices.contains_key(&from) {
            return None;
        }

        self.vertices.get_mut(&from).unwrap().to.insert(key.clone());

        self.edges.insert(
            key.clone(),
            Node {
                data,
                from: from.clone(),
                to: to.clone(),
            },
        );

        key.into()
    }
}

impl<Vk, V, Ek, E> Edge<Undirected, Vk, (Ek, E)> for Simple<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
{
    type EdgeKey = Ek;

    fn add_edge(&mut self, from: &Vk, to: &Vk, (key, data): (Ek, E)) -> Option<Self::EdgeKey> {
        let output = Edge::<Directed, Vk, (Ek, E)>::add_edge(self, from, to, (key.clone(), data));
        self.vertices.get_mut(&to).unwrap().from.insert(key);
        output
    }
}

pub type RemovedVertex<VertexKey, Vertex, EdgeKey, Edge> = (
    VertexKey,
    Node<
        Vertex,
        Vec<(EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
        Vec<(EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
    >,
);

impl<VertexKey, Vertex, EdgeKey, Edge> RemoveVertex<VertexKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    type Output = RemovedVertex<VertexKey, Vertex, EdgeKey, Edge>;

    fn remove_vertex(&mut self, key: &VertexKey) -> Option<Self::Output> {
        let (key, node) = self.vertices.remove_entry(key)?;

        let new_node = Node {
            data: node.data,
            from: node
                .from
                .into_iter()
                .flat_map(|key| self.remove_edge(&key))
                .collect(),
            to: node
                .to
                .into_iter()
                .flat_map(|key| self.remove_edge(&key))
                .collect(),
        };

        Some((key, new_node))
    }
}

impl<VertexKey, Vertex, EdgeKey, Edge> RemoveEdge<EdgeKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    type Output = (EdgeKey, Node<Edge, VertexKey, VertexKey>);

    fn remove_edge(&mut self, key: &EdgeKey) -> Option<Self::Output> {
        self.edges.remove_entry(key)
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> GetVertex<'a, VertexKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
    Vertex: 'a,
{
    type Output = &'a Vertex;

    fn get_vertex(&'a self, key: &VertexKey) -> Option<Self::Output> {
        self.vertices.get(key).map(|node| &node.data)
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> GetEdge<'a, EdgeKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
    Edge: 'a,
{
    type Output = &'a Edge;

    fn get_edge(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.edges.get(key).map(|node| &node.data)
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> GetEdgeTo<'a, EdgeKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
    VertexKey: 'a,
{
    type Output = &'a VertexKey;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.edges.get(key).map(|node| &node.to)
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> Neighbours<'a, Directed, VertexKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + 'a,
    EdgeKey: Eq + Hash + 'a,
{
    type Edge = &'a EdgeKey;
    type IntoIter = Vec<(Self::Edge, &'a VertexKey)>;

    fn neighbours(&'a self, vertex: &VertexKey) -> Option<Self::IntoIter> {
        self.vertices
            .get(vertex)?
            .to
            .iter()
            .flat_map(|key| Some((key, &self.edges.get(key)?.to)))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> Neighbours<'a, Undirected, VertexKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + 'a,
    EdgeKey: Eq + Hash + 'a,
{
    type Edge = &'a EdgeKey;
    type IntoIter = Vec<(Self::Edge, &'a VertexKey)>;

    fn neighbours(&'a self, vertex: &VertexKey) -> Option<Self::IntoIter> {
        let to = self
            .vertices
            .get(vertex)?
            .to
            .iter()
            .flat_map(|key| Some((key, &self.edges.get(key)?.to)));

        let from = self
            .vertices
            .get(vertex)?
            .from
            .iter()
            .flat_map(|key| Some((key, &self.edges.get(key)?.to)));

        to.chain(from).collect::<Vec<_>>().into()
    }
}
