pub trait Loader<T> {
    fn load (&self, path:String) -> Result<T, String>;
}
