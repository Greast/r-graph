use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::dev::node::Node;
use std::hash::Hash;

#[derive(Debug)]
struct Relative<Vertex, EdgeKey, Edge>
    where
        EdgeKey : Eq + Hash{
    pub node : <Self as Deref>::Target
}

impl<Vertex, EdgeKey, Edge> Clone for Relative<Vertex, EdgeKey, Edge>
    where
        EdgeKey : Eq + Hash{
    fn clone(&self) -> Self {
        Relative{
            node : self.node.clone(),
        }
    }
}

impl<Vertex, EdgeKey, Edge> Deref for Relative<Vertex, EdgeKey, Edge>
    where
        EdgeKey : Eq + Hash{
    type Target = Arc<
        RwLock<
            Node<
                Vertex,
                HashMap<EdgeKey, Self>,
                HashMap<EdgeKey, (Edge, Self)>
            >>>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<Vertex, EdgeKey, Edge> DerefMut for Relative<Vertex, EdgeKey, Edge>
    where
        EdgeKey : Eq + Hash{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl<Vertex, EdgeKey, Edge> Relative<Vertex, EdgeKey, Edge>
    where
        EdgeKey : Eq + Hash{
    pub fn new(vertex:Vertex) -> Self{
        let node = Node{
            data: vertex,
            from: Default::default(),
            to: Default::default()
        };

        Relative{
            node: Arc::new(RwLock::new(node))
        }
    }

    pub fn merge(&mut self, (key,value):(EdgeKey, Edge), other:Self) -> bool
        where
            EdgeKey : Clone{
        let to_conflict = self.node.write().unwrap().to.contains_key(&key);
        let from_conflict = self.node.write().unwrap().from.contains_key(&key);
        let conflict = to_conflict || from_conflict;
        if !conflict{
            other.node.write().unwrap().from.insert(key.clone(), self.clone());
            self.node.write().unwrap().to.insert(key ,(value, other));
        }
        conflict
    }
}