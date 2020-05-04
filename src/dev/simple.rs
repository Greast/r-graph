use crate::dev::node::Node;
use crate::dev::orientation::{AddEdge, Directed, Undirected};

use crate::dev::transform::Transform;
use crate::dev::{
    AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex, Vertices,
};
use std::collections::hash_map::Keys;
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

impl<VertexKey, Vertex, EdgeKey, Edge> AddVertex<(VertexKey, Vertex)>
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

impl<Vk, V, Ek, E> AddEdge<Directed, Vk, (Ek, E)> for Simple<Vk, V, Ek, E>
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
        self.vertices.get_mut(&to).unwrap().from.insert(key.clone());

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

impl<Vk, V, Ek, E> AddEdge<Undirected, Vk, (Ek, E)> for Simple<Vk, V, Ek, E>
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
        let output =
            AddEdge::<Directed, Vk, (Ek, E)>::add_edge(self, from, to, (key.clone(), data))?;
        self.vertices
            .get_mut(&from)
            .unwrap()
            .from
            .insert(key.clone());
        self.vertices.get_mut(&to).unwrap().to.insert(key);
        Ok(output)
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

impl<'a, VertexKey, Vertex, EdgeKey, Edge> GetVertex<VertexKey>
    for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    type Output = Vertex;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.vertices.get(key).map(|node| &node.data)
    }
}

impl<VertexKey, Vertex, EdgeKey, Edge> GetEdge<EdgeKey> for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    type Output = Edge;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
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
        let to = self.vertices.get(vertex)?.to.iter().flat_map(|key| {
            Some((key, self.edges.get(key)?.other(vertex)))
        });

        let from = self.vertices.get(vertex)?.from.iter().flat_map(|key| {
            Some((key, self.edges.get(key)?.other(vertex)))
        });

        to.chain(from).collect::<Vec<_>>().into()
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> Vertices<'a> for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + 'a,
    Vertex: 'a,
    EdgeKey: 'a + Eq + Hash,
{
    type Item = VertexKey;
    type Output = Keys<'a, VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>>;

    fn vertices(&'a self) -> Self::Output {
        self.vertices.keys()
    }
}

impl<'a, VertexKey, Vertex, EdgeKey, Edge> Edges<'a> for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + 'a,
    EdgeKey: Eq + Hash + 'a,
    Edge: 'a,
{
    type Item = EdgeKey;
    type Output = Keys<'a, EdgeKey, Node<Edge, VertexKey, VertexKey>>;

    fn edges(&'a self) -> Self::Output {
        self.edges.keys()
    }
}

impl<VK, V, EK, E, VKmap, Vmap, EKmap, Emap, VK2, V2, EK2, E2>
    Transform<VKmap, Vmap, EKmap, Emap, Simple<VK, V, EK, E>> for Simple<VK2, V2, EK2, E2>
where
    VK2: Eq + Hash,
    EK2: Eq + Hash,
    VK: Eq + Hash,
    EK: Eq + Hash,
    VKmap: Fn(VK) -> VK2 + Clone,
    Vmap: Fn(V) -> V2,
    EKmap: Fn(EK) -> EK2 + Clone,
    Emap: Fn(E) -> E2,
{
    fn collect(
        graph: Simple<VK, V, EK, E>,
        (vk_map, v_map, ek_map, e_map): (VKmap, Vmap, EKmap, Emap),
    ) -> Self {
        let vertices = graph
            .vertices
            .into_iter()
            .map(|(key, node)| {
                (
                    vk_map.clone()(key),
                    Node {
                        data: v_map(node.data),
                        from: node.from.into_iter().map(ek_map.clone()).collect(),
                        to: node.to.into_iter().map(ek_map.clone()).collect(),
                    },
                )
            })
            .collect();

        let edges = graph
            .edges
            .into_iter()
            .map(|(key, node)| {
                (
                    ek_map(key),
                    Node {
                        data: e_map(node.data),
                        from: vk_map.clone()(node.from),
                        to: vk_map(node.to),
                    },
                )
            })
            .collect();
        Self { vertices, edges }
    }
}
