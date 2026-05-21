#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondStateView {
    New,
    GettingCloser,
    Familiar,
}

impl Default for BondStateView {
    fn default() -> Self {
        Self::New
    }
}
