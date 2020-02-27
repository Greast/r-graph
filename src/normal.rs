use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::simple::Simple;
use crate::dev::*;
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub struct Normal<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    graph: Simple<VertexKey, Vertex, EdgeKey, Edge>,
}

impl<VertexKey, Vertex, EdgeKey, Edge> Default for Normal<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    fn default() -> Self {
        Self {
            graph: Default::default(),
        }
    }
}

impl<Vk, V, Ek, E> Edge<Directed> for Normal<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
    Standard: Distribution<Ek>,
{
    type VertexKey = Vk;
    type EdgeKey = Ek;
    type Edge = E;

    fn add_edge(
        &mut self,
        from: &Self::VertexKey,
        to: &Self::VertexKey,
        value: Self::Edge,
    ) -> Self::EdgeKey {
        let id: Self::EdgeKey = random();
        self.graph
            .insert_directed_edge(from.clone(), to.clone(), id.clone(), value);
        id
    }
}

impl<Vk, V, Ek, E> Edge<Undirected> for Normal<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
    Standard: Distribution<Ek>,
{
    type VertexKey = Vk;
    type EdgeKey = Ek;
    type Edge = E;

    fn add_edge(
        &mut self,
        from: &Self::VertexKey,
        to: &Self::VertexKey,
        value: Self::Edge,
    ) -> Self::EdgeKey {
        let id: Self::EdgeKey = random();
        self.graph
            .insert_undirected_edge(from.clone(), to.clone(), id.clone(), value);
        id
    }
}

impl<Vk, V, Ek, E> Builder<V> for Normal<Vk, V, Ek, E>
where
    Vk: Eq + Hash + Clone,
    Ek: Eq + Hash + Clone,
    Standard: Distribution<Vk>,
{
    type VertexKey = Vk;

    fn add_vertex(&mut self, vertex: V) -> Self::VertexKey {
        let id: Self::VertexKey = random();
        self.graph.insert_vertex(id.clone(), vertex);
        id
    }
}

pub struct VertexReference<'a, Vk, V, Ek, E>
where
    Vk: Eq + Hash,
    Ek: Eq + Hash,
{
    key: &'a Vk,
    graph: &'a Normal<Vk, V, Ek, E>,
}

impl<'a, Vk, V, Ek, E> Reference<'a> for VertexReference<'a, Vk, V, Ek, E>
    where
        Vk: Eq + Hash,
        Ek: Eq + Hash,{
    type Key = &'a Vk;
    type Data = &'a V;

    fn key(&'a self) -> Self::Key {
        self.key
    }

    fn data(&'a self) -> Self::Data {
        &self.graph.graph.vertices.get(self.key).unwrap().data
    }
}

pub struct EdgeReference<'a, Vk, V, Ek, E>
where
    Vk: Eq + Hash,
    Ek: Eq + Hash,
{
    key: &'a Ek,
    graph: &'a Normal<Vk, V, Ek, E>,
}

impl<'a, Vk, V, Ek, E> Reference<'a> for EdgeReference<'a, Vk, V, Ek, E>
    where
        Vk: Eq + Hash,
        Ek: Eq + Hash,{
    type Key = &'a Ek;
    type Data = &'a E;

    fn key(&'a self) -> Self::Key {
        self.key
    }

    fn data(&'a self) -> Self::Data {
        &self.graph.graph.edges.get(self.key).unwrap().data
    }
}

impl<'a, Vk, V, Ek, E> Getter<'a, Vk, Ek> for Normal<Vk, V, Ek, E>
where
    Vk: 'a + Eq + Hash + Clone,
    V: 'a,
    Ek: 'a + Eq + Hash + Clone,
    E: 'a,
    Self: 'a,
{
    type VertexReference = Option<VertexReference<'a, Vk, V, Ek, E>>;
    type EdgeReference = Option<EdgeReference<'a, Vk, V, Ek, E>>;

    fn get_vertex(&'a self, key: &Vk) -> Self::VertexReference {
        let key = self.graph.vertices.get_key_value(key)?.0;
        Some(VertexReference { key, graph: self })
    }

    fn get_edge(&'a self, key: &Ek) -> Self::EdgeReference {
        let key = self.graph.edges.get_key_value(key)?.0;
        Some(EdgeReference { key, graph: self })
    }
}

impl<'a, Vk, V, Ek, E> VertexReference<'a, Vk, V, Ek, E>
where
    Vk: 'a + Eq + Hash + Clone,
    V: 'a,
    Ek: 'a + Eq + Hash + Clone,
    E: 'a,
    Self: 'a,
{
    fn to(&self) -> Vec<(EdgeReference<'a, Vk, V, Ek, E>, Self)> {
        if let Some(node) = self.graph.graph.vertices.get(self.key) {
            node.to
                .iter()
                .flat_map(|x| self.graph.graph.edges.get(x).map(|y| (x, y)))
                .map(|(key, node)| (self.graph.get_edge(key), self.graph.get_vertex(&node.to)))
                .flat_map(|(x, y)| x.and_then(|a| y.map(|b| (a, b))))
                .collect()
        } else {
            Default::default()
        }
    }
    fn from(&self) -> Vec<(EdgeReference<'a, Vk, V, Ek, E>, Self)> {
        if let Some(node) = self.graph.graph.vertices.get(self.key) {
            node.from
                .iter()
                .flat_map(|x| self.graph.graph.edges.get(x).map(|y| (x, y)))
                .map(|(key, node)| (self.graph.get_edge(key), self.graph.get_vertex(&node.to)))
                .flat_map(|(x, y)| x.and_then(|a| y.map(|b| (a, b))))
                .collect()
        } else {
            Default::default()
        }
    }
}

impl<'a, Vk, V, Ek, E> EdgeReference<'a, Vk, V, Ek, E>
where
    Vk: Eq + Hash,
    Ek: Eq + Hash,
{
    pub fn data(&self) -> &'a E {
        &self.graph.graph.edges.get(self.key).unwrap().data
    }
}

impl<'a, Vk, V, Ek, E> Neighbours<Directed> for VertexReference<'a, Vk, V, Ek, E>
where
    Vk: 'a + Eq + Hash + Clone,
    V: 'a,
    Ek: 'a + Eq + Hash + Clone,
    E: 'a,
    Self: 'a,
{
    type Edge = EdgeReference<'a, Vk, V, Ek, E>;
    type IntoIter = Vec<(Self::Edge, Self)>;

    fn neighbours(&self) -> Self::IntoIter {
        self.to()
    }
}

impl<'a, Vk, V, Ek, E> Neighbours<Undirected> for VertexReference<'a, Vk, V, Ek, E>
where
    Vk: 'a + Eq + Hash + Clone,
    V: 'a,
    Ek: 'a + Eq + Hash + Clone,
    E: 'a,
    Self: 'a,
{
    type Edge = EdgeReference<'a, Vk, V, Ek, E>;
    type IntoIter = Vec<(Self::Edge, Self)>;

    fn neighbours(&self) -> Self::IntoIter {
        let mut out = self.to();
        out.append(&mut self.from());
        out
    }
}

use std::collections::hash_map::RandomState;
use std::iter::FromIterator;

fn take_random<K>(set: &mut HashSet<K>) -> Option<K>
where
    K: Eq + Hash + Clone,
{
    let e = set.iter().next()?.clone();
    set.take(&e)
}

impl<'a, Vk, V, Ek, E> Cyclic<Directed> for Normal<Vk, V, Ek, E>
where
    Vk: 'a + Eq + Hash + Clone,
    V: 'a,
    Ek: 'a + Eq + Hash + Clone,
    E: 'a,
    Self: 'a,
{
    fn cyclic(&self) -> bool {
        let mut unvisited: HashSet<_, RandomState> = HashSet::from_iter(self.graph.vertices.keys());
        while let Some(element) = take_random(&mut unvisited) {
            let mut queue = VecDeque::new();
            queue.push_front(element);

            while let Some(element) = queue.pop_front() {
                let iter = Neighbours::<Directed>::neighbours(&self.get_vertex(element).unwrap())
                    .into_iter();
                for e in iter {
                    if !unvisited.remove(&e.1.key) {
                        return true;
                    }
                    queue.push_back(e.1.key);
                }
            }
        }
        false
    }
}

impl<'a, Vk, V, Ek, E> Cyclic<Undirected> for Normal<Vk, V, Ek, E>
where
    Vk: 'a + Eq + Hash + Clone,
    V: 'a,
    Ek: 'a + Eq + Hash + Clone,
    E: 'a,
    Self: 'a,
{
    fn cyclic(&self) -> bool {
        let mut unvisited: HashSet<_, RandomState> = HashSet::from_iter(self.graph.vertices.keys());
        let mut traveled = HashSet::new();
        while let Some(element) = take_random(&mut unvisited) {
            let mut queue = VecDeque::new();
            queue.push_front(element);

            while let Some(element) = queue.pop_front() {
                let iter = Neighbours::<Directed>::neighbours(&self.get_vertex(element).unwrap())
                    .into_iter();

                for e in iter {
                    if traveled.contains(e.0.key) {
                        continue;
                    }

                    if !unvisited.remove(&e.1.key) {
                        return true;
                    }
                    queue.push_back(e.1.key);
                    traveled.insert(e.0.key);
                }
            }
        }
        false
    }
}
