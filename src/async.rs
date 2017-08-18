pub enum Async<A, F> {
    Ready(A),
    NotReady(F),
}
