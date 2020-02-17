pub mod simple;

pub trait Reference<'a, VertexKey, EdgeKey> {
    type VertexReference;
    type EdgeReference;
    fn get_vertex(&'a self, vertex: &VertexKey) -> Self::VertexReference;
    fn get_edge(&'a self, edge: &EdgeKey) -> Self::EdgeReference;
}

pub mod unique {
    pub trait Vertex {}
    pub trait Edge {}
}

pub mod orientation {
    pub trait Orientation {}

    pub struct Directed;
    impl Orientation for Directed {}

    pub struct Undirected;
    impl Orientation for Undirected {}

    pub trait Edge<O: Orientation> {
        type VertexKey;
        type EdgeKey;
        type Edge;
        fn add_edge(
            &mut self,
            from: &Self::VertexKey,
            to: &Self::VertexKey,
            value: Self::Edge,
        ) -> Self::EdgeKey;
    }
}

pub trait Builder<V> {
    type VertexKey;
    fn add_vertex(&mut self, vertex: V) -> Self::VertexKey;
}

pub trait Neighbours {
    type Vertex;
    type Edge;
    type Iter: IntoIterator<Item = (Self::Vertex, Self::Edge)>;
    fn neighbours(&self) -> Self::Iter;
}
