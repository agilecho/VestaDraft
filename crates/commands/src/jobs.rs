//! Jobs are admitted by the NS2 owner and advanced between frames. Domain kernels publish only at commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    Queued,
    Running,
    Committed,
    Failed,
    Cancelled,
}
