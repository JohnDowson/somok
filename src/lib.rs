trait ToResult: Sized {
    fn okay<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    fn error<T>(self) -> Result<T, Self> {
        Err(self)
    }
}
impl<T: Sized> ToResult for T {}
trait ToOption: Sized {
    fn some(self) -> Option<Self> {
        Some(self)
    }
}
impl<T: Sized> ToOption for T {}
