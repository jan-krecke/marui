#[derive(Debug)]
pub enum UnorderedPair<T> {
    Pair(T, T),
}

impl<T: std::cmp::PartialEq> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UnorderedPair::Pair(a1, b1), UnorderedPair::Pair(a2, b2)) => {
                (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2)
            }
        }
    }
}
