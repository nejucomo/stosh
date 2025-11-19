pub(crate) trait Handler<M> {
    type Response;

    fn handle(&mut self, msg: M) -> std::io::Result<Self::Response>;
}
