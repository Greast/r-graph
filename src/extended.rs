use crate::dev::orientation::Orientation;
use crate::dev::Neighbours;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;

trait Breath<O: Orientation>: Sized {
    type Iter: IntoIterator<Item = Self>;
    fn breath(self, to: Self) -> Self::Iter;
}

impl<Node, O: Orientation> Breath<O> for Node
where
    Self: Neighbours<O> + Clone + Eq + Hash,
{
    type Iter = Vec<Self>;

    fn breath(self, to: Self) -> Self::Iter {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(x) = queue.pop_front() {
            if x == to {
                break;
            } else {
                for (e, n) in x.neighbours() {
                    if !visited.contains_key(&n) {
                        queue.push_back(n.clone());
                        visited.insert(n, x.clone());
                    }
                }
            }
        }
        if visited.contains_key(&to) {
            let mut output = vec![to];
            while let Some(x) = visited.remove(output.last().unwrap()) {
                output.push(x);
            }
            output
        } else {
            Default::default()
        }
    }
}

trait Dijsktra<O: Orientation>: Sized {
    type Iter: IntoIterator<Item = Self>;
    fn dijsktra(self, to: Self) -> Self::Iter;
}

struct Header<H, B>(H, B);

impl<H, B> PartialEq for Header<H, B>
where
    H: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<H, B> Eq for Header<H, B> where H: Eq {}

impl<H, B> PartialOrd for Header<H, B>
where
    H: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<H, B> Ord for Header<H, B>
where
    H: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<Node, O: Orientation> Dijsktra<O> for Node
where
    Self: Neighbours<O> + Clone + Eq + Hash,
    <Self as Neighbours<O>>::Edge: Ord,
{
    type Iter = Vec<Self>;

    fn dijsktra(self, to: Self) -> Self::Iter {
        let mut visited = HashMap::new();
        let mut queue = BinaryHeap::new();
        if self != to {
            for (e, n) in self.neighbours() {
                if !visited.contains_key(&n) {
                    queue.push(Header(e, n.clone()));
                    visited.insert(n, self.clone());
                }
            }
        }

        while let Some(Header(p, x)) = queue.pop() {
            if x == to {
                break;
            } else {
                for (e, n) in x.neighbours() {
                    if !visited.contains_key(&n) {
                        queue.push(Header(e, n.clone()));
                        visited.insert(n, x.clone());
                    }
                }
            }
        }

        if visited.contains_key(&to) {
            let mut output = vec![to];
            while let Some(x) = visited.remove(output.last().unwrap()) {
                output.push(x);
            }
            output
        } else {
            Default::default()
        }
    }
}
