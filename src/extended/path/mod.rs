pub mod breadth;
pub mod dijkstra;
pub mod a_star;

trait Path<'a, Vertex, Edge>
where
    Vertex: 'a,
    Edge: 'a,
{
    type IntoIter: IntoIterator<Item = (&'a Vertex, &'a Edge)>;
    fn to(&mut self, _: &'a Vertex) -> Option<Self::IntoIter>;
}

trait PathFinder<'a, Key, Finder> {
    fn path(&'a self, from: &'a Key) -> Finder;
}
