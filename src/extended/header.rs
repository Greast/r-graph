use std::cmp::Ordering;

#[derive(Default, Copy, Clone, Debug)]
pub struct Header<Head, Body>(pub Head, pub Body);

impl<Head, Body> PartialEq for Header<Head, Body>
where
    Head: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<Head, Body> Eq for Header<Head, Body> where Head: Eq {}

impl<Head, Body> PartialOrd for Header<Head, Body>
where
    Head: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Head, Body> Ord for Header<Head, Body>
where
    Head: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
