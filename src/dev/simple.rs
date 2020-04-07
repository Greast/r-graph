use crate::dev::node::Node;
use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::transform::{mapping, Map};
use crate::dev::{
    Builder, Edges, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex, Vertices,
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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

    fn add_vertex(
        &mut self,
        (key, data): (VertexKey, Vertex),
    ) -> Result<Self::Key, (VertexKey, Vertex)> {
        if self.vertices.contains_key(&key) {
            Err((key, data))
        } else {
            self.vertices.insert(
                key.clone(),
                Node {
                    data,
                    from: Default::default(),
                    to: Default::default(),
                },
            );
            Ok(key)
        }
    }
}

impl<Vk, V, Ek, E> Edge<Directed, Vk, (Ek, E)> for Simple<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
{
    type EdgeKey = Ek;

    fn add_edge(
        &mut self,
        from: &Vk,
        to: &Vk,
        (key, data): (Ek, E),
    ) -> Result<Self::EdgeKey, (Ek, E)> {
        if !self.vertices.contains_key(&from) {
            return Err((key, data));
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

        Ok(key)
    }
}

impl<Vk, V, Ek, E> Edge<Undirected, Vk, (Ek, E)> for Simple<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
{
    type EdgeKey = Ek;

    fn add_edge(
        &mut self,
        from: &Vk,
        to: &Vk,
        (key, data): (Ek, E),
    ) -> Result<Self::EdgeKey, (Ek, E)> {
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

impl<'a, VertexKey, Vertex, EdgeKey, Edge> Vertices<'a, VertexKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + 'a,
    EdgeKey: Eq + Hash,
{
    type Output = HashSet<&'a VertexKey>;

    fn vertices(&'a self) -> Self::Output {
        self.vertices.keys().collect()
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> Edges<'a, EdgeKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash + 'a,
{
    type Output = HashSet<&'a EdgeKey>;

    fn edges(&'a self) -> Self::Output {
        self.edges.keys().collect()
    }
}

pub struct Mapper<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter>
where
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
{
    pub vertices: VertexIntoIter,
    pub edges: EdgeIntoIter,
}

impl<VertexKey, Vertex, EdgeKey, Edge> Map for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: 'static + Eq + Hash,
    EdgeKey: 'static + Eq + Hash,
    Vertex: 'static,
    Edge: 'static,
{
    type Mapper = Mapper<
        VertexKey,
        Vertex,
        EdgeKey,
        Edge,
        Box<dyn Iterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>>,
        Box<dyn Iterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>>,
    >;

    fn map(self) -> Self::Mapper {
        Mapper {
            vertices: Box::new(self.vertices.into_iter()),
            edges: Box::new(self.edges.into_iter()),
        }
    }
}
use super::transform::Mapper as MapperTrait;

impl<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter, VertexKey2, Func>
    MapperTrait<mapping::VertexKey, VertexKey, VertexKey2, Func>
    for Mapper<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter>
where
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
    <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    Func: 'static + Copy,
{
    type Output = Mapper<
        VertexKey2,
        Vertex,
        EdgeKey,
        Edge,
        Box<dyn Iterator<Item = (VertexKey2, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>>,
        Box<dyn Iterator<Item = (EdgeKey, Node<Edge, VertexKey2, VertexKey2>)>>,
    >;

    fn map(self, function: Func) -> Self::Output
    where
        Func: Fn(VertexKey) -> VertexKey2,
    {
        let vertices = Box::new(
            self.vertices
                .into_iter()
                .map(move |(key, node)| (function(key), node)),
        );
        let edges = Box::new(self.edges.into_iter().map(move |(key, node)| {
            (
                key,
                Node {
                    data: node.data,
                    from: function(node.from),
                    to: function(node.to),
                },
            )
        }));
        Mapper { vertices, edges }
    }
}

impl<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter, Vertex2, Func>
    MapperTrait<mapping::Vertex, Vertex, Vertex2, Func>
    for Mapper<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter>
where
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
    <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    Func: 'static + Copy,
{
    type Output = Mapper<
        VertexKey,
        Vertex2,
        EdgeKey,
        Edge,
        Box<dyn Iterator<Item = (VertexKey, Node<Vertex2, HashSet<EdgeKey>, HashSet<EdgeKey>>)>>,
        Box<dyn Iterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>>,
    >;

    fn map(self, function: Func) -> Self::Output
    where
        Func: Fn(Vertex) -> Vertex2,
    {
        let vertices = Box::new(self.vertices.into_iter().map(move |(key, node)| {
            (
                key,
                Node {
                    data: function(node.data),
                    from: node.from,
                    to: node.to,
                },
            )
        }));
        Mapper {
            vertices,
            edges: Box::new(self.edges.into_iter()),
        }
    }
}

impl<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter, EdgeKey2, Func>
    MapperTrait<mapping::EdgeKey, EdgeKey, EdgeKey2, Func>
    for Mapper<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter>
where
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
    <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    Func: 'static + Copy,
    EdgeKey2: Eq + Hash,
{
    type Output = Mapper<
        VertexKey,
        Vertex,
        EdgeKey2,
        Edge,
        Box<
            dyn Iterator<
                Item = (
                    VertexKey,
                    Node<Vertex, HashSet<EdgeKey2>, HashSet<EdgeKey2>>,
                ),
            >,
        >,
        Box<dyn Iterator<Item = (EdgeKey2, Node<Edge, VertexKey, VertexKey>)>>,
    >;

    fn map(self, function: Func) -> Self::Output
    where
        Func: Fn(EdgeKey) -> EdgeKey2,
    {
        let vertices = Box::new(self.vertices.into_iter().map(move |(key, node)| {
            (
                key,
                Node {
                    data: node.data,
                    from: node.from.into_iter().map(function).collect(),
                    to: node.to.into_iter().map(function).collect(),
                },
            )
        }));
        let edges = Box::new(
            self.edges
                .into_iter()
                .map(move |(key, node)| (function(key), node)),
        );
        Mapper { vertices, edges }
    }
}

impl<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter, Edge2, Func>
    MapperTrait<mapping::Edge, Edge, Edge2, Func>
    for Mapper<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter>
where
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
    <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'static,
    Func: 'static + Copy,
{
    type Output = Mapper<
        VertexKey,
        Vertex,
        EdgeKey,
        Edge2,
        Box<dyn Iterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>>,
        Box<dyn Iterator<Item = (EdgeKey, Node<Edge2, VertexKey, VertexKey>)>>,
    >;

    fn map(self, function: Func) -> Self::Output
    where
        Func: Fn(Edge) -> Edge2,
    {
        let edges = Box::new(self.edges.into_iter().map(move |(key, node)| {
            (
                key,
                Node {
                    data: function(node.data),
                    from: node.from,
                    to: node.to,
                },
            )
        }));
        Mapper {
            vertices: Box::new(self.vertices.into_iter()),
            edges,
        }
    }
}
