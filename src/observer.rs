pub trait Observer {
    type Item;
    type Err;
    fn next(&mut self, value: Self::Item) {}
    fn error(&mut self, err: Self::Err) {}
    fn complete(&mut self) {}
}

impl<Item, Err, T> Observer for Box<T>
where
    T: Observer<Item = Item, Err = Err> + ?Sized,
{
    type Item = Item;
    type Err = Err;
    fn next(&mut self, value: Self::Item) {
        todo!()
    }
    fn error(&mut self, err: Self::Err) {
        todo!()
    }
    fn complete(&mut self) {
        todo!()
    }
}
