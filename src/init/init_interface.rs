//! Standard interface for all init structures


/// Standard interface for all init structures
pub trait InitInterface {
    /// Init
    fn init(&self) -> Result<(), String>;
}