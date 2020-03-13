use crate::dev::{orientation, GetEdge, GetEdgeTo, GetVertex, Neighbours};

pub struct VertexReference<'a, Key, Graph> {
    pub key: &'a Key,
    pub graph: &'a Graph,
}

trait VertexRef<'a, 'b, Key>
where
    Self: Sized,
{
    fn vertex_ref(&'a self, key: &'b Key) -> VertexReference<'a, Key, Self>;
}

impl<'a, Key, Graph> VertexRef<'a, 'a, Key> for Graph {
    fn vertex_ref(&'a self, key: &'a Key) -> VertexReference<'a, Key, Self> {
        VertexReference { key, graph: self }
    }
}

pub struct EdgeReference<'a, Key, Graph> {
    pub key: &'a Key,
    pub graph: &'a Graph,
}

trait EdgeRef<'a, 'b, Key>
where
    Self: Sized,
{
    fn edge_ref(&'a self, key: &'b Key) -> EdgeReference<'a, Key, Self>;
}

impl<'a, Key, Graph> VertexReference<'a, Key, Graph> {
    pub fn neighbours<Orientation>(&self) -> Option<Vec<Self>>
    where
        Orientation: orientation::Orientation + Clone,
        Graph: Neighbours<'a, Orientation, Key>,
    {
        self.graph
            .neighbours(self.key)?
            .into_iter()
            .map(|(_, vertex)| Self {
                key: vertex,
                graph: self.graph,
            })
            .collect::<Vec<_>>()
            .into()
    }

    pub fn travel<Orientation, Edge, NewKey>(
        &self,
        edge: &'a Edge,
    ) -> Option<VertexReference<NewKey, Graph>>
    where
        Edge: 'a,
        NewKey: 'a,
        Graph: GetEdgeTo<'a, (&'a Key, &'a Edge), Output = &'a NewKey>,
    {
        self.graph
            .get_edge_to(&(self.key, edge))
            .map(|key| VertexReference {
                key,
                graph: self.graph,
            })
    }
}
