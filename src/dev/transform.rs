pub trait Collect {
    type Output;
    fn collect(self) -> Option<Self::Output>;
}

pub trait Map<Trans, T, R, Func> {
    type Mapper: Collect;
    fn map(self, func: Func) -> Self::Mapper;
}

pub mod transformers {
    pub struct VertexKey;
    pub struct Vertex;
    pub struct EdgeKey;
    pub struct Edge;
}
