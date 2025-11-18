pub(crate) trait Handler<M> {
    type Response;

    async fn handle(&mut self, msg: M) -> Self::Response;
}
