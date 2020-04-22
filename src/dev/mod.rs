pub mod node;
pub mod relative;
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

///Contains types and traits for edges and their orientation.
pub mod orientation {
    pub trait Orientation {}

    ///Tells the current context, that concerning edge(s) are to be interpreted as directed.
    #[derive(Default, Debug, Eq, PartialEq)]
    pub struct Directed;
    impl Orientation for Directed {}

    ///Tells the current context, that concerning edge(s) are to be interpreted as undirected/bidirected.
    #[derive(Default, Debug, Eq, PartialEq)]
    pub struct Undirected;
    impl Orientation for Undirected {}

    ///Adds the given edge to the given graph, connected to the given nodes. The orientation of this edge changes based on the given orientation type.
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

pub trait GetVertex<Key> {
    type Output;
    fn get_vertex(&self, key: &Key) -> Option<&Self::Output>;
}

pub trait GetEdge<Key> {
    type Output;
    fn get_edge(&self, key: &Key) -> Option<&Self::Output>;
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

pub trait Merge<Rhs = Self>
where
    Self: Sized,
{
    type Output;
    fn merge(self, _: Rhs) -> Result<Self::Output, (Self, Rhs)>;
}

pub trait Dot<T, R, R2, G>{
    type Output : Fn(T) -> R2;
    fn dot(self, function:G) -> Self::Output;
}

impl <T, R, R2, G, F> Dot<T, R, R2, G> for F
    where
        F : Fn(T) -> R,
        G : Fn(R) -> R2,{
    type Output = impl Fn(T) -> R2;

    fn dot(self, function: G) -> Self::Output  {
        move |x| function(self(x))
    }
}