pub mod breadth;
pub mod dijkstra;

trait Path<'a, Vertex, Edge>
where
    Vertex: 'a,
    Edge: 'a,
{
    type IntoIter: IntoIterator<Item = (&'a Vertex, &'a Edge)>;
    fn to(&'a mut self, _: &'a Vertex) -> Option<Self::IntoIter>;
}
