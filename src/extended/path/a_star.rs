use crate::extended::path::{Path, DistanceFunctor, PathFinder};
use std::marker::PhantomData;
use crate::dev::{Neighbours, orientation};

pub struct AStarFinder<'a, Graph, Function, Distance, VertexKey, EdgeKey, Orientation>{
    distance_functor : &'a DistanceFunctor<'a, Graph, Function>,
    from : &'a VertexKey,
    phantom : PhantomData<(Function, EdgeKey, Orientation)>
}

impl<'a, VertexKey, Graph, Function, Distance, EdgeKey, Orientation> PathFinder<'a, VertexKey, AStarFinder<'a, Graph, Function, Distance, VertexKey, EdgeKey, Orientation>> for DistanceFunctor<'a, Graph, Function>{
    fn path(&'a self, from: &'a VertexKey) -> AStarFinder<'a, Graph, Function, Distance, VertexKey, EdgeKey, Orientation> {
        AStarFinder{
            distance_functor: self,
            from,
            phantom: PhantomData
        }
    }
}

impl<'a, VertexKey, EdgeKey, Graph, Function, Distance, Orientation> Path<'a, VertexKey, EdgeKey> for AStarFinder<'a, Graph, Function, Distance, VertexKey, EdgeKey, Orientation>{
    type IntoIter = ();

    fn to(&mut self, to: &'a VertexKey) -> Option<Self::IntoIter> {
        self.
    }
}