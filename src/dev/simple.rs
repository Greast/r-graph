use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Default, Eq, PartialEq)]
pub struct Node<Data, From, To> {
    pub data: Data,
    pub from: From,
    pub to: To,
}

///A simple graph implementation, where the key for each edge and vertex has to be supplied.
#[derive(Eq, PartialEq)]
pub struct Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    pub vertices: HashMap<VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>>,
    pub edges: HashMap<EdgeKey, Node<Edge, VertexKey, VertexKey>>,
}

impl<VertexKey, Vertex, EdgeKey, Edge> Default for Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}
impl<VertexKey, Vertex, EdgeKey, Edge> Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash + Clone,
    EdgeKey: Eq + Hash + Clone,
{
    ///Insert a given vertex with a key, should the key already exists, the current data associated with the key will be replaced with the new data, and the old be returned.
    pub fn insert_vertex(&mut self, key: VertexKey, mut data: Vertex) -> Option<Vertex> {
        if let Some(node) = self.vertices.get_mut(&key) {
            std::mem::swap(&mut node.data, &mut data);
            Some(data)
        } else {
            self.vertices.insert(
                key.clone(),
                Node {
                    data,
                    from: Default::default(),
                    to: Default::default(),
                },
            );
            None
        }
    }
    ///Insert a undirected edge such that both the to- and from- vertex has a reference to the edge, should the edge-key already exists, it will be deleted.
    pub fn insert_undirected_edge(
        &mut self,
        from: VertexKey,
        to: VertexKey,
        key: EdgeKey,
        data: Edge,
    ) -> Option<Edge> {
        let output = self.insert_directed_edge(from, to.clone(), key.clone(), data);
        self.vertices.get_mut(&to).unwrap().from.insert(key);
        output
    }

    ///Insert a directed edge, such that only the from-vertex has any reference to the edge, should the edge-key already exists, it will be deleted.
    pub fn insert_directed_edge(
        &mut self,
        from: VertexKey,
        to: VertexKey,
        key: EdgeKey,
        data: Edge,
    ) -> Option<Edge> {
        if !self.vertices.contains_key(&from) {
            return None;
        }

        let output = self.remove_edge(&key);

        self.vertices.get_mut(&from).unwrap().to.insert(key.clone());

        self.edges.insert(key, Node { data, from, to });

        output
    }
}

impl<VertexKey, Vertex, EdgeKey, Edge> Simple<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    ///Remove an edge by its id.
    pub fn remove_edge(&mut self, edge: &EdgeKey) -> Option<Edge> {
        let node = self.edges.remove(&edge)?;
        if self.vertices.get_mut(&node.to).is_some() {
            let from = self.vertices.get_mut(&node.from)?;
            from.from.remove(&edge);
            Some(node.data)
        } else {
            None
        }
    }

    ///Remove a vertex by its id, along with all nodes to and from this vertex.
    pub fn remove_vertex(&mut self, vertex: &VertexKey) -> Option<Vertex> {
        let node = self.vertices.remove(vertex)?;
        for edge in node.from.into_iter().chain(node.to.into_iter()) {
            self.remove_edge(&edge);
        }
        Some(node.data)
    }
}
