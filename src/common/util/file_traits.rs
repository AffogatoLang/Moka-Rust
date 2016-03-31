use std::io;
pub trait Loader<T> {
    fn load (&self) -> io::Result<T>;
}
