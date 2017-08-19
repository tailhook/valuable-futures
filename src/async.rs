/// A variant of `futures::Async` that carries state in `NotReady`
pub enum Async<A, F> {
    /// Represents that a value is already ready
    Ready(A),
    /// Represents that a value is not ready yet, and carries next state
    /// of the future
    NotReady(F),
}
