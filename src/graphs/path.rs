use crate::dev::orientation::Edge;
use crate::dev::{
    orientation, Builder, GetEdge, GetEdgeTo, GetVertex, Neighbours, RemoveEdge, RemoveVertex,
};

///The edges for this graph are hashed along with its from-node, allowing for graphs such as those found in deterministic automaton.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Path<Graph> {
    graph: Graph,
}

impl<Graph, Input> Builder<Input> for Path<Graph>
where
    Graph: Builder<Input>,
{
    type Key = <Graph as Builder<Input>>::Key;

    fn add_vertex(&mut self, vertex: Input) -> Result<Self::Key, Input> {
        self.graph.add_vertex(vertex)
    }
}

impl<Orientation, Graph, VertexKey, EdgeKey, Value> Edge<Orientation, VertexKey, (EdgeKey, Value)>
    for Path<Graph>
where
    Orientation: orientation::Orientation,
    Graph:
        Edge<Orientation, VertexKey, ((VertexKey, EdgeKey), Value), EdgeKey = (VertexKey, EdgeKey)>,
    VertexKey: Clone,
{
    type EdgeKey = (VertexKey, EdgeKey);

    fn add_edge(
        &mut self,
        from: &VertexKey,
        to: &VertexKey,
        (key, value): (EdgeKey, Value),
    ) -> Result<Self::EdgeKey, (EdgeKey, Value)> {
        self.graph.add_edge(from, to, ((from.clone(), key), value)).map_err(|((_,x),y)|(x,y))
    }
}

impl<Key, Graph> RemoveVertex<Key> for Path<Graph>
where
    Graph: RemoveVertex<Key>,
{
    type Output = <Graph as RemoveVertex<Key>>::Output;

    fn remove_vertex(&mut self, key: &Key) -> Option<Self::Output> {
        self.graph.remove_vertex(key)
    }
}

impl<Key, Graph> RemoveEdge<Key> for Path<Graph>
where
    Graph: RemoveEdge<Key>,
{
    type Output = <Graph as RemoveEdge<Key>>::Output;

    fn remove_edge(&mut self, key: &Key) -> Option<Self::Output> {
        self.graph.remove_edge(key)
    }
}

impl<'a, Key, Graph> GetVertex<'a, Key> for Path<Graph>
where
    Graph: GetVertex<'a, Key>,
{
    type Output = <Graph as GetVertex<'a, Key>>::Output;

    fn get_vertex(&'a self, key: &Key) -> Option<Self::Output> {
        self.graph.get_vertex(key)
    }
}

impl<'a, Key, Graph> GetEdge<'a, Key> for Path<Graph>
where
    Graph: GetEdge<'a, Key>,
{
    type Output = <Graph as GetEdge<'a, Key>>::Output;

    fn get_edge(&'a self, key: &Key) -> Option<Self::Output> {
        self.graph.get_edge(key)
    }
}

impl<'a, Key, Graph> GetEdgeTo<'a, Key> for Path<Graph>
where
    Graph: GetEdgeTo<'a, Key>,
{
    type Output = <Graph as GetEdgeTo<'a, Key>>::Output;

    fn get_edge_to(&'a self, key: &Key) -> Option<Self::Output> {
        self.graph.get_edge_to(key)
    }
}

impl<'a, Orientation, Key, Graph> Neighbours<'a, Orientation, Key> for Path<Graph>
where
    Key: 'a,
    Orientation: orientation::Orientation,
    Graph: Neighbours<'a, Orientation, Key>,
{
    type Edge = <Graph as Neighbours<'a, Orientation, Key>>::Edge;
    type IntoIter = <Graph as Neighbours<'a, Orientation, Key>>::IntoIter;

    fn neighbours(&'a self, key: &Key) -> Option<Self::IntoIter> {
        self.graph.neighbours(key)
    }
}
