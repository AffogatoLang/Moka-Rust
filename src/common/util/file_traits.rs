use std::io;
pub trait Loader<T> {
    fn load (&self) -> Result<T, io::Error>;
}
