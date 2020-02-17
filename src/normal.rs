use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::simple::Simple;
use crate::dev::*;
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::hash::Hash;

struct Normal<VertexKey, Vertex, EdgeKey, Edge>
where
    VertexKey: Eq + Hash,
    EdgeKey: Eq + Hash,
{
    graph: Simple<VertexKey, Vertex, EdgeKey, Edge>,
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
        Standard: Distribution<Vk>{
    type VertexKey = Vk;

    fn add_vertex(&mut self, vertex: V) -> Self::VertexKey {
        let id:Self::VertexKey = random();
        self.graph.insert_vertex(id.clone(),vertex);
        id
    }
}

struct VertexReference<'a, Vk, V, Ek, E >
    where
        Vk: Eq + Hash,
        Ek: Eq + Hash,{
    pub key:&'a Vk,
    graph :&'a Normal<Vk, V, Ek, E>
}

struct EdgeReference<'a, Vk, V, Ek, E >
    where
        Vk: Eq + Hash,
        Ek: Eq + Hash,{
    pub key:&'a Ek,
    graph :&'a Normal<Vk, V, Ek, E>
}

impl<'a, Vk, V, Ek, E> Reference<'a, Vk, Ek> for Normal<Vk, V, Ek, E>
    where
        Vk: 'a + Eq + Hash + Clone,
        V : 'a,
        Ek: 'a + Eq + Hash + Clone,
        E : 'a,
        Self : 'a{
    type VertexReference = Option<VertexReference<'a, Vk, V, Ek, E>>;
    type EdgeReference = Option<EdgeReference<'a, Vk, V, Ek, E>>;

    fn get_vertex(&'a self, key: &Vk) -> Self::VertexReference {
        let key = self.graph.vertices.get_key_value(key)?.0;
        Some(
            VertexReference{
                key,
                graph : self
            }
        )
    }

    fn get_edge(&'a self, key: &Ek) -> Self::EdgeReference {
        let key = self.graph.edges.get_key_value(key)?.0;
        Some(
            EdgeReference{
                key,
                graph : self
            }
        )
    }
}