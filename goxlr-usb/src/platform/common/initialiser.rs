pub(crate) trait InitialisableGoXLR {
    async fn initialise(&mut self) -> anyhow::Result<()>;
}
