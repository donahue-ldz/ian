use crate::protocol::IanEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensitivityLevel {
    Low,
    Medium,
    High,
}

pub trait PerceptionAdapter {
    fn id(&self) -> &'static str;
    fn sensitivity(&self) -> SensitivityLevel;
    fn poll(&mut self) -> Vec<IanEvent>;
}
