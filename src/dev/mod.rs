pub mod node;
pub mod simple;
pub mod transform;
///Remove a vertex associated with the given key, along with all incoming and outgoing edges, from the graph.
pub trait RemoveVertex<Key> {
    type Output;
    fn remove_vertex(&mut self, key: &Key) -> Option<Self::Output>;
}

///Remove an edge associated with the given key.
pub trait RemoveEdge<Key> {
    type Output;
    fn remove_edge(&mut self, key: &Key) -> Option<Self::Output>;
}

///Neighbours of the vertex associated with the given key, with the Orientation type, determining if the edges are directed or not.
pub trait Neighbours<'a, Orientation, VertexKey>
where
    VertexKey: 'a,
    Orientation: orientation::Orientation,
{
    type Edge;
    type IntoIter: IntoIterator<Item = (Self::Edge, &'a VertexKey)>;
    fn neighbours(&'a self, vertex: &VertexKey) -> Option<Self::IntoIter>;
}

pub mod orientation {
    pub trait Orientation {}

    #[derive(Default)]
    pub struct Directed;
    impl Orientation for Directed {}

    #[derive(Default)]
    pub struct Undirected;
    impl Orientation for Undirected {}

    pub trait AddEdge<O: Orientation, VertexKey, Edge> {
        type EdgeKey;
        fn add_edge(
            &mut self,
            from: &VertexKey,
            to: &VertexKey,
            value: Edge,
        ) -> Result<Self::EdgeKey, Edge>;
    }
}

pub trait AddVertex<Input> {
    type Key;
    fn add_vertex(&mut self, vertex: Input) -> Result<Self::Key, Input>;
}

pub trait GetVertex<'a, Key> {
    type Output;
    fn get_vertex(&'a self, key: &Key) -> Option<Self::Output>;
}

pub trait GetEdge<'a, Key> {
    type Output;
    fn get_edge(&'a self, key: &Key) -> Option<Self::Output>;
}

pub trait GetEdgeTo<'a, Key> {
    type Output;
    fn get_edge_to(&'a self, key: &Key) -> Option<Self::Output>;
}

pub trait Vertices<'a, Key>
where
    Key: 'a,
{
    type Output: IntoIterator<Item = &'a Key>;
    fn vertices(&'a self) -> Self::Output;
}

pub trait Edges<'a, Key>
where
    Key: 'a,
{
    type Output: IntoIterator<Item = &'a Key>;
    fn edges(&'a self) -> Self::Output;
}

pub trait Merge
where
    Self: Sized,
{
    fn merge(self, _: Self) -> Result<Self, (Self, Self)>;
}
