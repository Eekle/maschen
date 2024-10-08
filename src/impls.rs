use crate::types;

#[cfg(feature = "std")]
impl<T> types::Stack for Vec<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) -> Result<(), types::Error> {
        self.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> types::Stack for &mut T
where
    T: types::Stack,
{
    type Item = T::Item;

    fn push(&mut self, value: Self::Item) -> Result<(), types::Error> {
        (**self).push(value)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        (**self).pop()
    }
}

#[cfg(feature = "std")]
impl<T: types::Token> crate::ShuntingYard<Vec<T>, Vec<T>, Vec<usize>> {
    pub fn new() -> Self {
        crate::ShuntingYard::new_with_storage(vec![], vec![], vec![])
    }
}

#[cfg(feature = "std")]
impl<T: types::Token> Default for crate::ShuntingYard<Vec<T>, Vec<T>, Vec<usize>> {
    fn default() -> Self {
        Self::new()
    }
}
