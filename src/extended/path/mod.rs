pub mod a_star;
pub mod breadth;
pub mod dijkstra;

pub trait Path<'a, Vertex, Edge>
where
    Vertex: 'a,
    Edge: 'a,
{
    type IntoIter: IntoIterator<Item = (&'a Vertex, &'a Edge)>;
    fn to(&mut self, _: &'a Vertex) -> Option<Self::IntoIter>;
}

pub trait PathFinder<'a, Key, Finder> {
    fn path(&'a self, from: &'a Key) -> Finder;
}

pub struct DistanceFunctor<'a, Graph, Dist> {
    graph: &'a Graph,
    dist: Dist,
}

pub trait PathDistanceFinder<'a, Dist, Graph> {
    fn dist(&'a self, dist: Dist) -> DistanceFunctor<'a, Graph, Dist>;
}

impl<'a, Dist, Graph> PathDistanceFinder<'a, Dist, Graph> for Graph {
    fn dist(&'a self, dist: Dist) -> DistanceFunctor<'a, Graph, Dist> {
        DistanceFunctor { graph: self, dist }
    }
}
