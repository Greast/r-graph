pub mod edge;
pub mod vertex;

pub use edge::Edge;
pub use vertex::Vertex;
use crate::dev::{Merge, GetEdge};
use std::hash::Hash;
use crate::dev::transform::{Map, Collect};
use rand::distributions::{Standard, Distribution};
use std::collections::HashMap;
use std::rc::Rc;
use rand::random;

fn safe_map<'a, G1, G2, E, T>(this:G1, other:G2) -> Result<<G1 as Merge<G2>>::Output, Option<(G1, G2)>>
    where
        T: 'a + Eq + Hash + Clone,
        G1: Map<E, T, T, Box<dyn 'a + FnMut(T) -> T>>,
        G1: Merge<G2>,
        <G1 as Map<E,T,T,Box<dyn 'a + FnMut(T) -> T>>>::Mapper: Collect<Output = G1>,
        Standard: Distribution<T>,
        G2: 'a + GetEdge<T>,
{
    let mut seen = Rc::new(HashMap::new());
    let reference = Rc::new(other);

    let mut r1 = seen.clone();
    let r2 = reference.clone();

    let closure = move |old: T| {
        let mut key = old.clone();
        while r1.contains_key(&key) && r2.get_edge(&key).is_some() {
            key = random();
        }
        Rc::get_mut(&mut r1).unwrap().insert(key.clone(), old);
        key
    };
    let graph = this.map(Box::new(closure)).collect().map_or(Err(None), Ok)?;

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