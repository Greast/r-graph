use crate::dev::{Vertices, Neighbours};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use crate::dev::orientation::{Directed, Undirected};
use std::fmt::Debug;

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

impl<'a, Vertex, Graph> Cycle<'a, Undirected, Vertex> for Graph
    where
        Vertex : 'a + Eq + Hash + Debug,
        Self : Vertices<'a, Vertex> + Neighbours<'a, Undirected, Vertex>,
        <Self as Neighbours<'a, Undirected, Vertex>>::Edge : Hash + Eq + Debug {

    fn cycle(&'a self) -> bool {
        let mut queue = VecDeque::new();

        let mut vertices : HashSet<_> = self.vertices().into_iter().collect();

        while let Some(cluster) = take_random(&mut vertices){

            queue.push_back((None, cluster));

            while let Some((mut from, vertex)) = queue.pop_front(){
                for (edge, vert) in self.neighbours(vertex).into_iter().flatten(){
                    if from.as_ref().map(|x| x == &edge).is_some(){
                        from.take();
                        continue;
                    }
                    if !vertices.remove(vert){
                        return true;
                    }
                    queue.push_back((Some(edge),vert));
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
    fn simple_directed_cycle() {
        let mut graph = Simple::default().orient(Directed);
        let a = graph.add_vertex((0,())).unwrap();
        graph.add_edge(&a,&a, (0,())).unwrap();

        assert!(graph.cycle())
    }
    #[test]
    fn simple_undirected_cycle() {
        let mut graph = Simple::default().orient(Undirected);
        let a = graph.add_vertex((0,())).unwrap();
        graph.add_edge(&a,&a, (0,())).unwrap();

        assert!(graph.cycle())
    }
    #[test]
    fn simple_undirected_non_cycle() {
        let mut graph = Simple::default().orient(Undirected);

        let a = graph.add_vertex((0,())).unwrap();
        let b = graph.add_vertex((1,())).unwrap();

        graph.add_edge(&a,&b, (0,())).unwrap();

        assert!(!graph.cycle())
    }


}