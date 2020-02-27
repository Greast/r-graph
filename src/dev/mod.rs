use crate::dev::orientation::Orientation;

pub mod simple;

pub trait Reference<'a>{
    type Key;
    type Data;
    fn key(&'a self) -> Self::Key;
    fn data(&'a self) -> Self::Data;
}

impl <'a, I:Reference<'a>> Reference<'a> for Option<I>{
    type Key = Option<<I as Reference<'a>>::Key>;
    type Data = Option<<I as Reference<'a>>::Data>;

    fn key(&'a self) -> Self::Key {
        self.as_ref().map(Reference::key)
    }

    fn data(&'a self) -> Self::Data {
        self.as_ref().map(Reference::data)
    }
}


pub trait Getter<'a, VertexKey, EdgeKey> {
    type VertexReference;
    type EdgeReference;
    fn get_vertex(&'a self, vertex: &VertexKey) -> Self::VertexReference;
    fn get_edge(&'a self, edge: &EdgeKey) -> Self::EdgeReference;
}

pub mod orientation {
    pub trait Orientation {}

    #[derive(Default)]
    pub struct Directed;
    impl Orientation for Directed {}

    #[derive(Default)]
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

pub trait Neighbours<O: Orientation>
where
    Self: Sized,
{
    type Edge;
    type IntoIter: IntoIterator<Item = (Self::Edge, Self)>;
    fn neighbours(&self) -> Self::IntoIter;
}

pub trait Travel<Edge>{
    fn travel(&self, edge:Edge) -> Self;
}

pub trait Cyclic<O: Orientation> {
    fn cyclic(&self) -> bool;
}
