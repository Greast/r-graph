use crate::dev::orientation::{AddEdge, Orientation};
use crate::dev::{AddVertex, RemoveEdge, RemoveVertex};

use std::ops::Deref;

struct Transaction<'a, Graph> {
    graph: Graph,
    cleaner: Vec<Box<dyn 'a + Fn(&mut Graph)>>,
    failed: bool,
}

impl<'a, Graph> Deref for Transaction<'a, Graph> {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<'a, Graph> Transaction<'a, Graph> {
    pub fn reroll(&mut self) {
        self.failed = true;
        for function in self.cleaner.drain(0..) {
            function(&mut self.graph);
        }
    }
    pub fn commit(self) -> Graph {
        self.graph
    }
}

impl<'a, Input, Graph> AddVertex<Input> for Transaction<'a, Graph>
where
    Graph: AddVertex<Input> + RemoveVertex<<Graph as AddVertex<Input>>::Key>,
    <Graph as AddVertex<Input>>::Key: 'a + Clone,
{
    type Key = <Graph as AddVertex<Input>>::Key;

    fn add_vertex(&mut self, vertex: Input) -> Result<Self::Key, Input> {
        if self.failed {
            Err(vertex)
        } else {
            let output = self.graph.add_vertex(vertex);

            if let Ok(key) = output {
                let keyc = key.clone();
                let closure = move |graph: &mut Graph| {
                    graph.remove_vertex(&keyc);
                };
                self.cleaner.push(Box::new(closure));
                Ok(key)
            } else {
                self.reroll();
                output
            }
        }
    }
}

impl<'a, O, Vk, Input, Graph> AddEdge<O, Vk, Input> for Transaction<'a, Graph>
where
    O: Orientation,
    Graph: AddEdge<O, Vk, Input> + RemoveEdge<<Graph as AddEdge<O, Vk, Input>>::EdgeKey>,
    <Graph as AddEdge<O, Vk, Input>>::EdgeKey: 'a + Clone,
{
    type EdgeKey = <Graph as AddEdge<O, Vk, Input>>::EdgeKey;

    fn add_edge(&mut self, from: &Vk, to: &Vk, value: Input) -> Result<Self::EdgeKey, Input> {
        if self.failed {
            Err(value)
        } else {
            let output = self.graph.add_edge(from, to, value);

            if let Ok(key) = output {
                let keyc = key.clone();
                let closure = move |graph: &mut Graph| {
                    graph.remove_edge(&keyc);
                };
                self.cleaner.push(Box::new(closure));
                Ok(key)
            } else {
                self.reroll();
                output
            }
        }
    }
}
