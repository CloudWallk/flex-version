/// Extension trait for splitting a prefix from a collection.
pub trait SplitPrefix {
    type Item;

    /// If the collection starts with one or more items as determined by the predicate,
    /// split that prefix from the collection. Otherwise, returns None.
    fn split_prefix(&self, pred: impl FnMut(Self::Item) -> bool) -> Option<(&Self, &Self)>;
}

impl SplitPrefix for str {
    type Item = char;

    fn split_prefix(&self, mut pred: impl FnMut(char) -> bool) -> Option<(&Self, &Self)> {
        match self.find(|c| !pred(c)) {
            Some(0) => None,                     // Prefix did not match.
            Some(ix) => Some(self.split_at(ix)), // Prefix matched.
            None => Some((self, "")),            // The entire string matched.
        }
    }
}
