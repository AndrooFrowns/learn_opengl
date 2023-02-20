/// Defines the properties of a given run.  Allows for identifiying the attempt in a few ways
pub trait Runner {
    /// The Chapter Number for this Runner
    fn chapter(&self) -> i32;

    /// The Section Number for this Runner
    fn section(&self) -> i32;

    /// The name for this Runner
    fn name(&self) -> &'static str;

    /// Run the example.
    fn run(&self);
}
