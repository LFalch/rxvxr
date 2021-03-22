use std::ops::Sub;

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

pub trait SubAbs {
    fn sub_abs(self, other: Self) -> Self;
}

impl<T: Ord + Sub<Output=T>> SubAbs for T {
    #[inline]
    fn sub_abs(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}