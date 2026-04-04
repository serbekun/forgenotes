pub trait InitInterface {

    fn init(&self) -> Result<(), String>;
}