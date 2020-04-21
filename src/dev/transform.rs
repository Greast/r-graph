use std::convert::identity;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use crate::dev::Dot;

struct Mapper<VKmap, Vmap, EKmap, Emap, VK, V, EK, E> {
    vk_map: VKmap,
    v_map: Vmap,
    ek_map: EKmap,
    e_map: Emap,
    phantom: (
        PhantomData<VK>,
        PhantomData<V>,
        PhantomData<EK>,
        PhantomData<E>,
    ),
}

impl<VK, V, EK, E> Default
    for Mapper<fn(VK) -> VK, fn(V) -> V, fn(EK) -> EK, fn(E) -> E, VK, V, EK, E>
{
    fn default() -> Self {
        Mapper {
            vk_map: identity,
            v_map: identity,
            ek_map: identity,
            e_map: identity,
            phantom: Default::default(),
        }
    }
}

pub struct Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph> {
    mapper: Mapper<VKmap, Vmap, EKmap, Emap, VK, V, EK, E>,
    pub graph: Graph,
}

impl<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph> Deref for Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph>{
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph> DerefMut for Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph>{

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}



impl<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph>
    Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph> {
    pub fn map_vertex_key<G, T, R>(
        self,
        function: G,
    ) -> Transformer<<VKmap as Dot<VK, T, R, G>>::Output, Vmap, EKmap , Emap, VK, V, R, E, Graph>
        where
            VKmap : Dot<VK, T, R, G>

    {
        let vk_map = self.mapper.vk_map;
        Transformer {
            mapper: Mapper {
                vk_map: vk_map.dot(function),
                v_map: self.mapper.v_map,
                ek_map: self.mapper.ek_map,
                e_map: self.mapper.e_map,
                phantom: Default::default(),
            },
            graph: self.graph,
        }
    }
    /*
    pub fn map_vertex_key<'a, Func, T, R>(
        self,
        function: Func,
    ) -> Transformer<Box<dyn 'a + Fn(T) -> R>, Vmap, EKmap, Emap, R, V, EK, E, Graph>
    where
        Func: 'a + Fn(VK) -> R,
        VKmap: Fn(T) -> VK,
        Graph: 'a,
        E: 'a,
        EK: 'a,
        V: 'a,
        VK: 'a,
        Emap: 'a,
        EKmap: 'a,
        Vmap: 'a,
        VKmap: 'a,

    {
        let vk_map = self.mapper.vk_map;
        Transformer {
            mapper: Mapper {
                vk_map: Box::new(move |x| function((vk_map)(x))),
                v_map: self.mapper.v_map,
                ek_map: self.mapper.ek_map,
                e_map: self.mapper.e_map,
                phantom: Default::default(),
            },
            graph: self.graph,
        }
    }

    pub fn map_vertex<Func, T, R>(
        self,
        function: Func,
    ) -> Transformer<VKmap, Box<dyn 'a + Fn(T) -> R>, EKmap, Emap, VK, R, EK, E, Graph>
    where
        Func: 'a + Fn(V) -> R,
        Vmap: Fn(T) -> V,
    {
        let v_map = self.mapper.v_map;
        Transformer {
            mapper: Mapper {
                vk_map: self.mapper.vk_map,
                v_map: Box::new(move |x| function((v_map)(x))),
                ek_map: self.mapper.ek_map,
                e_map: self.mapper.e_map,
                phantom: Default::default(),
            },
            graph: self.graph,
        }
    }
    */
    pub fn map_edge_key<G, T, R>(
        self,
        function: G,
    ) -> Transformer<VKmap, Vmap, <EKmap as Dot<EK, T, R, G>>::Output, Emap, VK, V, R, E, Graph>
        where
            EKmap : Dot<EK, T, R, G>

    {
        let ek_map = self.mapper.ek_map;
        Transformer {
            mapper: Mapper {
                vk_map: self.mapper.vk_map,
                v_map: self.mapper.v_map,
                ek_map: ek_map.dot(function),
                e_map: self.mapper.e_map,
                phantom: Default::default(),
            },
            graph: self.graph,
        }
    }


/*
    pub fn map_edge<Func, T, R>(
        self,
        function: Func,
    ) -> Transformer<VKmap, Vmap, EKmap, Box<dyn 'a + Fn(T) -> R>, VK, V, EK, R, Graph>
    where
        Func: 'a + Fn(V) -> R,
        Emap: Fn(T) -> V,
    {
        let e_map = self.mapper.e_map;
        Transformer {
            mapper: Mapper {
                vk_map: self.mapper.vk_map,
                v_map: self.mapper.v_map,
                ek_map: self.mapper.ek_map,
                e_map: Box::new(move |x| function((e_map)(x))),
                phantom: Default::default(),
            },
            graph: self.graph,
        }
    }
    */
}

pub trait Transform<VKmap, Vmap, EKmap, Emap, Graph>
where
    Self: Sized,
{
    fn collect(graph: Graph, _: (VKmap, Vmap, EKmap, Emap)) -> Self;
    fn transform< VK, V, EK, E>(
        self,
    ) -> Transformer<fn(VK) -> VK, fn(V) -> V, fn(EK) -> EK, fn(E) -> E, VK, V, EK, E, Self> {
        Transformer {
            mapper: Default::default(),
            graph: self,
        }
    }
}


impl<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph>
    Transformer<VKmap, Vmap, EKmap, Emap, VK, V, EK, E, Graph>
{
    pub fn collect<Graph2>(self) -> Graph2
    where
        Graph2: Transform<VKmap, Vmap, EKmap, Emap, Graph>,
    {
        let mapper = self.mapper;

        let tuple = (mapper.vk_map, mapper.v_map, mapper.ek_map, mapper.e_map);

        Graph2::collect(self.graph, tuple)
    }
}
