/// Cast an Object into another form
pub trait Upcast<T> {
    fn upcast(&self) -> Result<T, ()>;
}
