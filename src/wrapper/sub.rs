use crate::dev::orientation::{AddEdge, Orientation};
use crate::dev::{
    orientation, AddVertex, Edges, GetEdge, GetEdgeTo, GetVertex, Neighbours, Vertices,
};
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::identity;
use std::hash::Hash;

pub struct SubGraph<'a, Graph, Graph2> {
    parent: &'a Graph,
    sub: Graph2,
}

impl<'a, Graph, Graph2, VertexKey> AddVertex<VertexKey> for SubGraph<'a, Graph, Graph2>
where
    Graph: GetVertex<VertexKey>,
    Graph2: AddVertex<(VertexKey, ())>,
{
    type Key = <Graph2 as AddVertex<(VertexKey, ())>>::Key;

    fn add_vertex(&mut self, vertex: VertexKey) -> Result<Self::Key, VertexKey> {
        if self.parent.get_vertex(&vertex).is_none() {
            Err(vertex)
        } else {
            self.sub.add_vertex((vertex, ())).map_err(|x| x.0)
        }
    }
}

impl<'a, Graph, Graph2, Orientation, VertexKey, EdgeKey> AddEdge<Orientation, VertexKey, EdgeKey>
    for SubGraph<'a, Graph, Graph2>
where
    Orientation: orientation::Orientation,
    Graph: GetEdge<EdgeKey>,
    Graph2: AddEdge<Orientation, VertexKey, (EdgeKey, ())>,
{
    type EdgeKey = <Graph2 as AddEdge<Orientation, VertexKey, (EdgeKey, ())>>::EdgeKey;

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        value: EdgeKey,
    ) -> Result<Self::EdgeKey, EdgeKey> {
        if self.parent.get_edge(&value).is_none() {
            Err(value)
        } else {
            self.sub.add_edge(from, to, (value, ())).map_err(|x| x.0)
        }
    }
}

impl<'a, Graph, Graph2, VertexKey> GetVertex<VertexKey> for SubGraph<'a, Graph, Graph2>
where
    Graph: GetVertex<VertexKey>,
    Graph2: GetVertex<VertexKey>,
{
    type Output = <Graph as GetVertex<VertexKey>>::Output;

    fn get_vertex(&self, key: &VertexKey) -> Option<&Self::Output> {
        self.sub.get_vertex(key)?;
        self.parent.get_vertex(key)
    }
}

impl<'a, Graph, Graph2, EdgeKey> GetEdge<EdgeKey> for SubGraph<'a, Graph, Graph2>
where
    Graph: GetEdge<EdgeKey>,
    Graph2: GetEdge<EdgeKey>,
{
    type Output = <Graph as GetEdge<EdgeKey>>::Output;

    fn get_edge(&self, key: &EdgeKey) -> Option<&Self::Output> {
        self.sub.get_edge(key)?;
        self.parent.get_edge(key)
    }
}

impl<'a, Graph, Graph2, EdgeKey> GetEdgeTo<'a, EdgeKey> for SubGraph<'a, Graph, Graph2>
where
    Graph: GetEdgeTo<'a, EdgeKey>,
    Graph2: GetEdgeTo<'a, EdgeKey>,
{
    type Output = <Graph as GetEdgeTo<'a, EdgeKey>>::Output;

    fn get_edge_to(&'a self, key: &EdgeKey) -> Option<Self::Output> {
        self.sub.get_edge_to(key)?;
        self.parent.get_edge_to(key)
    }
}

impl<'a, Graph, Graph2, VertexKey, Orientation> Neighbours<'a, Orientation, VertexKey>
    for SubGraph<'a, Graph, Graph2>
where
    VertexKey: 'a,
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, VertexKey>,
    Graph2: Neighbours<'a, Orientation, VertexKey>,
{
    type Edge = <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge;
    type IntoIter =
        Vec<<<Graph as Neighbours<'a, Orientation, VertexKey>>::IntoIter as IntoIterator>::Item>;

    fn neighbours(&'a self, vertex: &VertexKey) -> Option<Self::IntoIter> {
        self.sub.neighbours(vertex)?;
        let output = self
            .parent
            .neighbours(vertex)?
            .into_iter()
            .filter(|(_, x)| self.sub.neighbours(x).is_some())
            .collect();
        Some(output)
    }
}

impl<'a, Graph, Graph2> Vertices<'a> for SubGraph<'a, Graph, Graph2>
where
    Graph2: Vertices<'a>,
{
    type Item = <Graph2 as Vertices<'a>>::Item;
    type Output = <Graph2 as Vertices<'a>>::Output;

    fn vertices(&'a self) -> Self::Output {
        self.sub.vertices()
    }
}

impl<'a, Graph, Graph2> Edges<'a> for SubGraph<'a, Graph, Graph2>
where
    Graph2: Edges<'a>,
{
    type Item = <Graph2 as Edges<'a>>::Item;
    type Output = <Graph2 as Edges<'a>>::Output;

    fn edges(&'a self) -> Self::Output {
        self.sub.edges()
    }
}

impl<'a, Graph, Graph2> SubGraph<'a, Graph, Graph2> {
    pub fn adjacent<'b, Orientation, VertexKey>(
        &'b self,
    ) -> HashSet<(
        <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge,
        &VertexKey,
    )>
    where
        'b: 'a,
        VertexKey: 'a + Eq + Hash,
        Orientation: orientation::Orientation,
        Graph: Neighbours<'a, Orientation, VertexKey>,
        Graph2: Vertices<'a, Item = VertexKey>,
        <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge: Eq + Hash,
    {
        self.sub
            .vertices()
            .into_iter()
            .filter_map(|x| self.parent.neighbours(x))
            .flat_map(identity)
            .collect::<HashSet<_>>()
    }

    pub fn common_neighbours<'b, Orientation, VertexKey>(&'b self) -> HashSet<&VertexKey>
    where
        'b: 'a,
        VertexKey: 'a + Eq + Hash,
        Orientation: orientation::Orientation,
        Graph: Neighbours<'a, Orientation, VertexKey>,
        Graph2: Vertices<'a, Item = VertexKey>,
        <Graph as Neighbours<'a, Orientation, VertexKey>>::Edge: Eq + Hash,
    {
        self.vertices()
            .into_iter()
            .filter_map(|x| self.parent.neighbours(x))
            .map(|x| x.into_iter().map(|x| x.1).collect::<HashSet<_>>())
            .fold1(|x, y| {
                let mut intersection = HashSet::new();
                for i in x {
                    if y.contains(i) {
                        intersection.insert(i);
                    }
                }
                intersection
            })
            .unwrap_or_default()
    }
}

pub trait Sub<VertexKeyIntoIter, EdgeKeyIntoIter>
where
    Self: Sized,
{
    fn sub(&self) -> SubGraph<Self, Self>;
}

impl<Graph, VertexKeyIntoIter, EdgeKeyIntoIter> Sub<VertexKeyIntoIter, EdgeKeyIntoIter> for Graph
where
    Graph: Default,
{
    fn sub(&self) -> SubGraph<Self, Self> {
        SubGraph {
            parent: self,
            sub: Default::default(),
        }
    }
}
