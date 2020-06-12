use std::collections::HashSet;
use std::hash::Hash;

pub mod clique;
pub mod cyclic;
pub mod header;
pub mod path;

fn take_random<V>(hash_set: &mut HashSet<V>) -> Option<V>
where
    V: Clone + Eq + Hash,
{
    let value = hash_set.iter().next()?.clone();
    hash_set.remove(&value);
    Some(value)
}
