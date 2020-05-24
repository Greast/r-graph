use crate::dev::orientation::Undirected;
use crate::dev::simple::Simple;
use crate::dev::Neighbours;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct CliqueIter<'a, Graph, Vk> {
    graph: &'a Graph,
    queue: VecDeque<&'a Vk>,
    visited: HashSet<&'a Vk>,
}

impl<'a, Graph, Vk> CliqueIter<'a, Graph, Vk>
where
    Vk: 'a + Eq + Hash,
    Graph: Neighbours<'a, Undirected, Vk>,
{
    fn complete(&self, seed: &'a Vk) -> HashSet<&'a Vk> {
        let mut clique = HashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back(seed);

        while let Some(vertex) = queue.pop_front() {
            let connections: HashSet<_> = self
                .graph
                .neighbours(vertex)
                .into_iter()
                .flatten()
                .map(|(_, x)| x)
                .collect();

            if connections.is_superset(&clique) {
                clique.insert(vertex);
                for i in connections.difference(&clique) {
                    if self.visited.contains(i) {
                        queue.push_front(i);
                    } else {
                        queue.push_back(i);
                    }
                }
            }
        }
        clique
    }
}

impl<'a, Graph, Vk> Iterator for CliqueIter<'a, Graph, Vk>
where
    Vk: 'a + Eq + Hash,
    Graph: Neighbours<'a, Undirected, Vk>,
{
    type Item = HashSet<&'a Vk>;

    fn next(&mut self) -> Option<Self::Item> {
        let seed = self.queue.pop_front()?;
        let mut complete = self.complete(seed);
        while complete.is_subset(&self.visited) {
            let seed = self.queue.pop_front()?;
            complete = self.complete(seed);
        }
        for i in complete.iter() {
            if self.visited.insert(i) {
                self.queue.push_back(i)
            }
        }
        complete.into()
    }
}

pub trait Clique<'a, Vk>
where
    Self: Sized,
{
    fn clique(&'a self, seed: &'a Vk) -> CliqueIter<'a, Self, Vk>;
}

impl<'a, Vk, Graph> Clique<'a, Vk> for Graph {
    fn clique(&'a self, seed: &'a Vk) -> CliqueIter<'a, Self, Vk> {
        CliqueIter {
            graph: self,
            queue: vec![seed].into(),
            visited: Default::default(),
        }
    }
}
