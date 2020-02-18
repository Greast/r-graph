use crate::dev::orientation::{Directed, Edge, Undirected};
use crate::dev::simple::Simple;
use crate::dev::*;
use rand::distributions::{Distribution, Standard};
use rand::random;
use std::hash::Hash;
use std::collections::HashSet;

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

impl <'a, Vk, V, Ek, E > VertexReference<'a, Vk, V, Ek, E >
    where
        Vk: 'a + Eq + Hash + Clone,
        V : 'a,
        Ek: 'a + Eq + Hash + Clone,
        E : 'a,
        Self : 'a{
    pub fn data(&self) -> &'a V{
        &self.graph.graph.vertices.get(self.key).unwrap().data
    }
    fn to(&self) -> Vec<(EdgeReference<'a, Vk, V, Ek, E >, Self)>{
        if let Some(node) = self.graph.graph.vertices.get(self.key){
            node.to.iter()
                .flat_map(|x |self.graph.graph.edges.get(x).map(|y|(x,y)))
                .map(|(key, node)| (
                    self.graph.get_edge(key),
                    self.graph.get_vertex(&node.to)
                ))
                .flat_map(|(x,y)|
                    x.map(|a| y.map(|b| (a,b) )).flatten())
                .collect()
        }else{
            Default::default()
        }
    }
    fn from(&self) -> Vec<(EdgeReference<'a, Vk, V, Ek, E >, Self)>{
        if let Some(node) = self.graph.graph.vertices.get(self.key){
            node.from.iter()
                .flat_map(|x |self.graph.graph.edges.get(x).map(|y|(x,y)))
                .map(|(key, node)| (
                    self.graph.get_edge(key),
                    self.graph.get_vertex(&node.to)
                ))
                .flat_map(|(x,y)|
                    x.map(|a| y.map(|b| (a,b) )).flatten())
                .collect()
        }else{
            Default::default()
        }
    }

}

impl <'a, Vk, V, Ek, E > EdgeReference<'a, Vk, V, Ek, E >
    where
        Vk: Eq + Hash,
        Ek: Eq + Hash,{
    pub fn data(&self) -> &'a E{
        &self.graph.graph.edges.get(self.key).unwrap().data
    }

}

impl<'a, Vk, V, Ek, E> Neighbours<Directed> for VertexReference<'a, Vk, V, Ek, E>
    where
        Vk: 'a + Eq + Hash + Clone,
        V : 'a,
        Ek: 'a + Eq + Hash + Clone,
        E : 'a,
        Self : 'a{
    type Edge = EdgeReference<'a, Vk, V, Ek, E>;
    type IntoIter = Vec<(Self::Edge, Self)>;

    fn neighbours(&self) -> Self::IntoIter {
        self.to()
    }
}

impl<'a, Vk, V, Ek, E> Neighbours<Undirected> for VertexReference<'a, Vk, V, Ek, E>
    where
        Vk: 'a + Eq + Hash + Clone,
        V : 'a,
        Ek: 'a + Eq + Hash + Clone,
        E : 'a,
        Self : 'a{
    type Edge = EdgeReference<'a, Vk, V, Ek, E>;
    type IntoIter = Vec<(Self::Edge, Self)>;

    fn neighbours(&self) -> Self::IntoIter {
        let mut out = self.to();
        out.append(&mut self.from());
        out
    }
}

impl<'a, O, Vk, V, Ek, E> Cyclic<O> for Simple<Vk, V, Ek, E>
    where
        Vk: 'a + Eq + Hash + Clone,
        V : 'a,
        Ek: 'a + Eq + Hash + Clone,
        E : 'a,
        Self : 'a,
        O : orientation::Orientation{
    fn cyclic(&self) -> bool {
        unimplemented!()
    }
}