/// Utility `struct` to represent an unordered pair
///
/// This struct is useful if the order of tuple elements does not matter for two  tuples
/// to be considered equal.
#[derive(Debug)]
pub struct UnorderedPair<T>(pub T, pub T);

impl<T: std::cmp::PartialEq> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        let (UnorderedPair(a1, b1), UnorderedPair(a2, b2)) = (self, other);
        (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2)
    }
}
