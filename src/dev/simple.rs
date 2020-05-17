use crate::dev::node::Node;
use crate::dev::orientation::{AddEdge, Directed, Undirected};

use crate::dev::transform::{transformers, Collect, Map};
use crate::dev::{
    AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex, Vertices,
};
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

///A simple graph implementation, where the key for each edge and vertex has to be supplied.
#[derive(Clone, Debug, Eq, PartialEq)]
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
        EdgeKey: Eq + Hash,{
    fn default() -> Self {
        Self{
            vertices:HashMap::new(),
            edges:HashMap::new(),
        }
    }
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
        let to = self
            .vertices
            .get(vertex)?
            .to
            .iter()
            .flat_map(|key| Some((key, self.edges.get(key)?.other(vertex))));

        let from = self
            .vertices
            .get(vertex)?
            .from
            .iter()
            .flat_map(|key| Some((key, self.edges.get(key)?.other(vertex))));

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

struct SimpleTransformer<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
{
    pub vertices: VertexIntoIter,
    pub edges: EdgeIntoIter,
    phantom: PhantomData<(VertexKey, Vertex, EdgeKey, Edge)>,
}

impl<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge> Collect
    for SimpleTransformer<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
{
    type Output = Simple<VertexKey, Vertex, EdgeKey, Edge>;

    fn collect(self) -> Option<Self::Output> {
        let vertices = self.vertices.into_iter().try_fold(HashMap::new(), |mut map,(key, value)|{
            if map.insert(key, value).is_none(){
                Some(map)
            } else {
                None
            }
        })?;

        let edges = self.edges.into_iter().try_fold(HashMap::new(), |mut map,(key, value)|{
            if map.insert(key, value).is_none(){
                Some(map)
            } else {
                None
            }
        })?;

        Self::Output{
            vertices,
            edges
        }.into()
    }
}

impl<'a, Func, VertexKey2, VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
    Map<'a, transformers::VertexKey, VertexKey, VertexKey2, Func>
    for SimpleTransformer<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey2: Eq + Hash,
    EdgeKey: 'a + Eq + Hash,
    Func: 'a + Fn(VertexKey) -> VertexKey2 + Clone,
    VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
    <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
    EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
    <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
{
    type Mapper = SimpleTransformer<
        Box<
            dyn 'a
                + Iterator<Item = (VertexKey2, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
        >,
        Box<dyn 'a + Iterator<Item = (EdgeKey, Node<Edge, VertexKey2, VertexKey2>)>>,
        VertexKey2,
        Vertex,
        EdgeKey,
        Edge,
    >;

    fn map(self, func: Func) -> Self::Mapper {
        let g = func.clone();
        let vertices = Box::new(
            self.vertices
                .into_iter()
                .map(move |(key, data)| (g(key), data)),
        );

        let edges = Box::new(self.edges.into_iter().map(move |(key, node)| {
            (
                key,
                Node {
                    data: node.data,
                    from: func(node.from),
                    to: func(node.to),
                },
            )
        }));

        SimpleTransformer {
            vertices,
            edges,
            phantom: PhantomData,
        }
    }
}

impl<'a, Func, Vertex2, VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
Map<'a, transformers::Vertex, VertexKey, VertexKey, Func>
for SimpleTransformer<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
    where
        VertexKey: Eq + Hash,
        EdgeKey: 'a + Eq + Hash,
        Func: 'a + Fn(Vertex) -> Vertex2,
        VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
        <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
        EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
        <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
{
    type Mapper = SimpleTransformer<
        Box<
            dyn 'a
            + Iterator<Item = (VertexKey, Node<Vertex2, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
        >,
        Box<dyn 'a + Iterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>>,
        VertexKey,
        Vertex2,
        EdgeKey,
        Edge,
    >;

    fn map(self, func: Func) -> Self::Mapper {
        let vertices = Box::new(
            self.vertices
                .into_iter()
                .map(move |(key, node)| (key, Node{
                    data: func(node.data),
                    from: node.from,
                    to: node.to
                })),
        );

        let edges = Box::new(self.edges.into_iter());

        SimpleTransformer {
            vertices,
            edges,
            phantom: PhantomData,
        }
    }
}

impl<'a, Func, EdgeKey2, VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
Map<'a, transformers::EdgeKey, EdgeKey, EdgeKey2, Func>
for SimpleTransformer<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
    where
        VertexKey: Eq + Hash,
        EdgeKey2: 'a + Eq + Hash,
        Func: 'a + Fn(EdgeKey) -> EdgeKey2 + Clone,
        VertexIntoIter: IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
        <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
        EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
        <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
{
    type Mapper = SimpleTransformer<
        Box<
            dyn 'a
            + Iterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey2>, HashSet<EdgeKey2>>)>,
        >,
        Box<dyn 'a + Iterator<Item = (EdgeKey2, Node<Edge, VertexKey, VertexKey>)>>,
        VertexKey,
        Vertex,
        EdgeKey2,
        Edge,
    >;

    fn map(self, func: Func) -> Self::Mapper {
        let g = func.clone();
        let vertices = Box::new(
            self.vertices
                .into_iter()
                .map(move |(key, node)|(key, Node{
                    data: node.data,
                    from: node.from.into_iter().map(g.clone()).collect(),
                    to: node.to.into_iter().map(g.clone()).collect()
                }))
        );

        let edges = Box::new(self.edges.into_iter().map(move |(key, node)| {
            (
                func(key),
                Node {
                    data: node.data,
                    from: node.from,
                    to: node.to,
                },
            )
        }));

        SimpleTransformer {
            vertices,
            edges,
            phantom: PhantomData,
        }
    }
}

impl<'a, Func, Edge2, VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
Map<'a, transformers::Edge, VertexKey, VertexKey, Func>
for SimpleTransformer<VertexIntoIter, EdgeIntoIter, VertexKey, Vertex, EdgeKey, Edge>
    where
        VertexKey: Eq + Hash,
        EdgeKey: 'a + Eq + Hash,
        Func: 'a + Fn(Edge) -> Edge2,
        VertexIntoIter:
        IntoIterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
        <VertexIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
        EdgeIntoIter: IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>,
        <EdgeIntoIter as std::iter::IntoIterator>::IntoIter: 'a,
{
    type Mapper = SimpleTransformer<
        Box<
            dyn 'a
            + Iterator<Item = (VertexKey, Node<Vertex, HashSet<EdgeKey>, HashSet<EdgeKey>>)>,
        >,
        Box<dyn 'a + Iterator<Item = (EdgeKey, Node<Edge2, VertexKey, VertexKey>)>>,
        VertexKey,
        Vertex,
        EdgeKey,
        Edge2,
    >;

    fn map(self, func: Func) -> Self::Mapper {
        let vertices = Box::new(self.vertices.into_iter());

        let edges = Box::new(self.edges.into_iter().map(move |(key, node)|(
            key, Node{
                data: func(node.data),
                from: node.from,
                to: node.to
            }
            )));

        SimpleTransformer {
            vertices,
            edges,
            phantom: PhantomData,
        }
    }
}