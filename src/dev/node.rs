#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Node<Data, From, To> {
    pub data: Data,
    pub from: From,
    pub to: To,
}
