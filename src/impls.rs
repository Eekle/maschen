use crate::types;

impl<T> types::Stack for Vec<T> {
    type Item = T;

    fn push(&mut self, value: Self::Item) -> Result<(), types::Error> {
        self.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    fn peek(&self) -> Option<&Self::Item> {
        self.last()
    }
}

impl<T> types::Stack for &mut T
where
    T: types::Stack,
{
    type Item = T::Item;

    fn push(&mut self, value: Self::Item) -> Result<(), crate::Error> {
        (**self).push(value)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        (**self).pop()
    }

    fn peek(&self) -> Option<&Self::Item> {
        (**self).peek()
    }
}
