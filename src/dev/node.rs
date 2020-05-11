#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Node<Data, From, To> {
    pub data: Data,
    pub from: From,
    pub to: To,
}

impl<Data, Edge> Node<Data, Edge, Edge> {
    pub fn other(&self, key: &Edge) -> &Edge
    where
        Edge: PartialEq,
    {
        if key == &self.from {
            &self.to
        } else {
            &self.from
        }
    }
}
