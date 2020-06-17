pub mod edge;
pub mod vertex;

use crate::dev::orientation::{AddEdge, Undirected};
use crate::dev::transform::{Collect, Map};
use crate::dev::{orientation, AddVertex, GetEdge, Merge};
pub use edge::Edge;
use itertools::Itertools;
use rand;
use rand::distributions::{Distribution, Standard};
use std::collections::HashMap;
use std::hash::Hash;
use std::process::Output;
use std::rc::Rc;
pub use vertex::Vertex;
use rand::Rng;

fn safe_map<'a, G1, G2, E, T>(
    this: G1,
    other: G2,
) -> Result<<G1 as Merge<G2>>::Output, Option<(G1, G2)>>
where
    T: 'a + Eq + Hash + Clone,
    G1: Map<E, T, T, Box<dyn 'a + FnMut(T) -> T>>,
    G1: Merge<G2>,
    <G1 as Map<E, T, T, Box<dyn 'a + FnMut(T) -> T>>>::Mapper: Collect<Output = G1>,
    Standard: Distribution<T>,
    G2: 'a + GetEdge<T>,
{
    let seen = Rc::new(HashMap::new());
    let reference = Rc::new(other);

    let mut r1 = seen.clone();
    let r2 = reference.clone();

    let closure = move |old: T| {
        let mut key = old.clone();
        while r1.contains_key(&key) && r2.get_edge(&key).is_some() {
            key = rand::random();
        }
        Rc::get_mut(&mut r1).unwrap().insert(key.clone(), old);
        key
    };
    let graph = this
        .map(Box::new(closure))
        .collect()
        .map_or(Err(None), Ok)?;

    let other = Rc::try_unwrap(reference).ok().map_or(Err(None), Ok)?;

    let output = graph.merge(other);

    match output {
        Err((x, other)) => {
            let mut flipped: HashMap<_, _> = Rc::try_unwrap(seen)
                .ok()
                .map_or(Err(None), Ok)?
                .into_iter()
                .map(|(x, y)| (y, x))
                .collect();

            let closure = move |key| flipped.remove(&key).unwrap_or(key);

            let graph = x.map(Box::new(closure)).collect().unwrap();

            Err(Some((graph, other)))
        }
        Ok(output) => Ok(output.into()),
    }
}

pub fn random<Graph, VertexKey>(size: usize, density: f64) -> Graph
where
    VertexKey: Clone,
    Graph: Default + AddVertex<(), Key = VertexKey> + AddEdge<Undirected, VertexKey, ()>,
{
    let mut graph = Graph::default();

    let vec = (0..size)
        .scan(&mut graph, |state, element| state.add_vertex(()).ok())
        .collect::<Vec<_>>();

    vec.clone().into_iter()
        .combinations(2)
        .filter_map(|mut slice|{
            let y = slice.pop()?;
            let x = slice.pop()?;
            Some((x,y))
        })
        .fold(graph, |mut state, (x,y)| {
            if rand::thread_rng().gen_range(0f64, 1f64 ) <= density{
                state.add_edge(&x, &y, ());
            }
            state
        })
}
