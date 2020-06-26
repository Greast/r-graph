use crate::dev::orientation::{AddEdge, Orientation, Undirected};

use crate::dev::transform::transformers::VertexKey;
use crate::dev::{orientation, AddVertex, GetVertex, Neighbours, Vertices};
use crate::extended::take_random;
use crate::wrapper::sub::{intersection, SubGraph};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque, BTreeSet, BTreeMap};
use std::hash::Hash;
use std::marker::PhantomData;

fn maximal_clique<'a, G, O, T>(graph: &'a G, mut sub_set: BTreeSet<&'a T>) -> BTreeSet<&'a T>
where
    T: 'a + Eq + Hash + Ord,
    O: Orientation,
    G: 'a + Neighbours<'a, O, T>,
{
    let mut common = sub_set
        .iter()
        .filter_map(|x| graph.neighbours(x))
        .map(|x| x.into_iter().map(|x| x.1).collect::<HashSet<_>>())
        .fold1(intersection)
        .unwrap_or_default();

    while let Some(from) = take_random(&mut common) {
        sub_set.insert(from);
        common = graph
            .neighbours(from)
            .into_iter()
            .flatten()
            .map(|x| x.1)
            .filter(|x| common.contains(x))
            .collect();
    }
    sub_set
}

pub struct CliqueIter<'a, Graph, VertexKey, Orientation> {
    graph: &'a Graph,
    queue: VecDeque<BTreeSet<&'a VertexKey>>,
    visited: BTreeSet<BTreeSet<&'a VertexKey>>,
    phantom: PhantomData<(Orientation,)>,
}

impl<'a, Graph, VertexKey, Orientation> Iterator for CliqueIter<'a, Graph, VertexKey, Orientation>
where
    VertexKey: 'a + Eq + Ord + Hash,
    Orientation: orientation::Orientation,
    Graph: 'a + Neighbours<'a, Orientation, VertexKey>,
    BTreeSet<&'a VertexKey>: Eq + Ord + Clone,
{
    type Item = BTreeSet<&'a VertexKey>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut clique = maximal_clique(self.graph, self.queue.pop_front()?);
        while !self.visited.insert(clique.clone()) {
            clique = maximal_clique(self.graph, self.queue.pop_front()?);
        }
        let candidates: BTreeMap<_, BTreeSet<_>> = clique
            .iter()
            .filter_map(|x| self.graph.neighbours(x).map(|y| (x, y)))
            .fold(BTreeMap::new(), |mut state, (&from, neighbours)| {
                for (_, i) in neighbours {
                    state.entry(from).or_default().insert(i);
                }
                state
            });
        for (from, mut neighbours) in candidates {
            neighbours.insert(from);
            self.queue.push_back(neighbours);
        }
        clique.into()
    }
}

pub trait Clique<'a, VertexKey, Orientation>
where
    Self: Sized,
{
    fn clique(&'a self, seed: &'a VertexKey) -> CliqueIter<'a, Self, VertexKey, Orientation>;
}

impl<'a, VertexKey, Orientation, Graph> Clique<'a, VertexKey, Orientation> for Graph
where
    VertexKey: 'a + Eq + Ord,
    Orientation: orientation::Orientation,
    Graph: 'a + Neighbours<'a, Orientation, VertexKey>,
{
    fn clique(&'a self, seed: &'a VertexKey) -> CliqueIter<'a, Self, VertexKey, Orientation> {
        let mut set = BTreeSet::new();
        set.insert(seed);
        CliqueIter {
            graph: self,
            queue: vec![set].into(),
            visited: Default::default(),
            phantom: PhantomData,
        }
    }
}