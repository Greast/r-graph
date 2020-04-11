use crate::dev::{Vertices, Neighbours, orientation};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

trait Cyclic<'a, Orientation, Vertex>{
    fn cyclic(&'a self) -> bool;
}

fn take_random<V>(hash_set:&mut HashSet<V>) -> Option<V>
    where
        V : Clone + Eq + Hash{
    let value = hash_set.iter().next()?.clone();
    hash_set.remove(&value);
    Some(value)

}

impl<'a, Orientation, Vertex, Graph> Cyclic<'a, Orientation, Vertex> for Graph
    where
        Orientation : orientation::Orientation,
        Vertex : 'a + Eq + Hash,
        Self : Vertices<'a, Vertex> + Neighbours<'a, Orientation, Vertex>{

    fn cyclic(&'a self) -> bool {
        let mut queue = VecDeque::new();
        let mut vertices : HashSet<_> = self.vertices().into_iter().collect();
        while let Some(cluster) = take_random(&mut vertices){
            queue.push_back(cluster);
            while let Some(vertex) = queue.pop_front(){
                for (_, vert) in self.neighbours(vertex).into_iter().flatten(){
                    if !vertices.remove(vert){
                        return true;
                    }
                    queue.push_back(vert);
                }
            }
        }
        false
    }
}