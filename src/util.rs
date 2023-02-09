#[derive(Debug)]
pub enum UnordTuple {
    Pair(String, String),
}

impl PartialEq for UnordTuple {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UnordTuple::Pair(a1, b1), UnordTuple::Pair(a2, b2)) => {
                (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2)
            }
        }
    }
}
