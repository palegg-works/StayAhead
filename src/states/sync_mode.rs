#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SyncMode {
    NotSynced,
    Pushing,
    Pulling,
    InSync,
    Failed,
}
