pub trait BinaryInsert<T> {
    fn binary_insert(&mut self, item: T);
}

impl<T: Ord> BinaryInsert<T> for Vec<T> {
    #[inline]
    fn binary_insert(&mut self, item: T) {
        let i = self.binary_search(&item).unwrap_or_else(|e| e);
        self.insert(i, item);
    }
}