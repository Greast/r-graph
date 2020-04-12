use crate::dev::{Vertices, Neighbours, orientation};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use crate::dev::orientation::Directed;

trait Cycle<'a, Orientation, Vertex>{
    fn cycle(&'a self) -> bool;
}

fn take_random<V>(hash_set:&mut HashSet<V>) -> Option<V>
    where
        V : Clone + Eq + Hash{
    let value = hash_set.iter().next()?.clone();
    hash_set.remove(&value);
    Some(value)

}

impl<'a, Vertex, Graph> Cycle<'a, Directed, Vertex> for Graph
    where
        Vertex : 'a + Eq + Hash,
        Self : Vertices<'a, Vertex> + Neighbours<'a, Directed, Vertex>{

    fn cycle(&'a self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::simple::Simple;
    use crate::wrapper::oriented::Orient;
    use crate::dev::AddVertex;
    use crate::dev::orientation::AddEdge;

    #[test]
    fn simple_cycle() {
        let mut graph = Simple::default().orient(Directed);
        let a = graph.add_vertex((0,())).unwrap();
        graph.add_edge(&a,&a, (0,())).unwrap();

        assert!(graph.cycle())
    }
}