use super::{BondSignal, BondStateView};

#[derive(Default)]
pub struct BondEngine {
    interaction_count: u32,
    state: BondStateView,
}

impl BondEngine {
    pub fn current(&self) -> BondStateView {
        self.state
    }

    pub fn apply(&mut self, signal: BondSignal) -> BondStateView {
        match signal {
            BondSignal::UserInteraction => {
                self.interaction_count += 1;
                self.state = if self.interaction_count >= 3 {
                    BondStateView::Familiar
                } else if self.interaction_count >= 1 {
                    BondStateView::GettingCloser
                } else {
                    BondStateView::New
                };
            }
        }

        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::BondEngine;
    use crate::domain::bond::{BondSignal, BondStateView};

    #[test]
    fn local_interactions_update_bond_view_without_numeric_ui_state() {
        let mut engine = BondEngine::default();

        assert_eq!(engine.current(), BondStateView::New);
        assert_eq!(
            engine.apply(BondSignal::UserInteraction),
            BondStateView::GettingCloser
        );
        engine.apply(BondSignal::UserInteraction);
        assert_eq!(
            engine.apply(BondSignal::UserInteraction),
            BondStateView::Familiar
        );
    }
}
