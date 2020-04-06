use crate::dev::node::Node;

pub struct EdgeGraph<VertexKey, Vertex, EdgeKey, Edge, VertexIntoIter, EdgeIntoIter >
    where
        VertexIntoIter : IntoIterator<Item = (VertexKey, Vertex)>,
        EdgeIntoIter : IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>{
    pub vertex_into_iter : VertexIntoIter,
    pub edge_into_iter : EdgeIntoIter,
}

pub trait IntoEdgeGraph<VertexKey, Vertex, EdgeKey, Edge>{
    type VertexIntoIter : IntoIterator<Item = (VertexKey, Vertex)>;
    type EdgeIntoIter : IntoIterator<Item = (EdgeKey, Node<Edge, VertexKey, VertexKey>)>;
    fn edge_graph(self) -> EdgeGraph<VertexKey, Vertex, EdgeKey, Edge, Self::VertexIntoIter, Self::EdgeIntoIter>;
}
