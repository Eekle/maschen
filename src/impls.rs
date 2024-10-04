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
