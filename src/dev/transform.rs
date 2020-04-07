pub trait Map {
    type Mapper;
    fn map(self) -> Self::Mapper;
}

pub trait Mapper<Branch, Type, RType, Func> {
    type Output;
    fn map(self, _: Func) -> Self::Output
    where
        Func: Fn(Type) -> RType;
}

pub mod mapping {
    pub struct VertexKey;

    pub struct Vertex;

    pub struct EdgeKey;

    pub struct Edge;
}
