use super::bond_state::BondStateView;

#[derive(Default)]
pub struct BondEngine;

impl BondEngine {
    pub fn current(&self) -> BondStateView {
        BondStateView::New
    }
}
