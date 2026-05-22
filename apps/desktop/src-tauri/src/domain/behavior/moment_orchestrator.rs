use std::collections::HashMap;

use crate::protocol::{IanAction, IanState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IanMomentKind {
    FindIanEntrance,
    PointerCuriosity,
    DragCarry,
    DropSettle,
    RareIdleSurprise,
    MemoryEcho,
}

impl IanMomentKind {
    pub fn key(self) -> &'static str {
        match self {
            Self::FindIanEntrance => "find_ian_entrance",
            Self::PointerCuriosity => "pointer_curiosity",
            Self::DragCarry => "drag_carry",
            Self::DropSettle => "drop_settle",
            Self::RareIdleSurprise => "rare_idle_surprise",
            Self::MemoryEcho => "memory_echo",
        }
    }

    pub fn from_key(value: &str) -> Option<Self> {
        match value {
            "find_ian_entrance" => Some(Self::FindIanEntrance),
            "pointer_curiosity" => Some(Self::PointerCuriosity),
            "drag_carry" => Some(Self::DragCarry),
            "drop_settle" => Some(Self::DropSettle),
            "rare_idle_surprise" => Some(Self::RareIdleSurprise),
            "memory_echo" => Some(Self::MemoryEcho),
            _ => None,
        }
    }

    fn cooldown_ms(self) -> i64 {
        match self {
            Self::FindIanEntrance => 4_000,
            Self::PointerCuriosity => 5_000,
            Self::DragCarry => 1_000,
            Self::DropSettle => 5_000,
            Self::RareIdleSurprise => 60 * 60 * 1000,
            Self::MemoryEcho => 12 * 60 * 60 * 1000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MomentDecision {
    pub kind: IanMomentKind,
    pub result: &'static str,
}

impl MomentDecision {
    pub fn triggered(&self) -> bool {
        self.result == "triggered"
    }

    pub fn diagnostic(&self, now_ms: i64) -> IanAction {
        IanAction::PlayfulDiagnostic {
            timestamp_ms: now_ms,
            reason: self.kind.key().to_string(),
            result: self.result.to_string(),
            cooldown_key: Some(self.kind.key().to_string()),
            chosen_reaction_key: self
                .triggered()
                .then(|| format!("{}_sequence", self.kind.key())),
        }
    }
}

#[derive(Debug)]
pub struct MomentOrchestrator {
    last_started_ms: HashMap<IanMomentKind, i64>,
    global_window_start_ms: i64,
    global_count: u32,
    global_budget: u32,
}

impl Default for MomentOrchestrator {
    fn default() -> Self {
        Self {
            last_started_ms: HashMap::new(),
            global_window_start_ms: 0,
            global_count: 0,
            global_budget: 4,
        }
    }
}

impl MomentOrchestrator {
    pub fn decide(&mut self, kind: IanMomentKind, now_ms: i64, state: &IanState) -> MomentDecision {
        let result = if blocks_context(kind, state) {
            "blocked_context"
        } else if blocks_user_control(kind, state) {
            "blocked_user_control"
        } else if state.do_not_disturb {
            "blocked_dnd"
        } else if state.movement_intensity == "reduced" && reduces_motion(kind) {
            "blocked_reduced_motion"
        } else if self.is_cooling_down(kind, now_ms) {
            "blocked_cooldown"
        } else if self.global_budget_exhausted(now_ms) && consumes_global_budget(kind) {
            "blocked_budget"
        } else {
            self.record_start(kind, now_ms);
            "triggered"
        };

        MomentDecision { kind, result }
    }

    fn is_cooling_down(&self, kind: IanMomentKind, now_ms: i64) -> bool {
        self.last_started_ms
            .get(&kind)
            .map(|last| now_ms.saturating_sub(*last) < kind.cooldown_ms())
            .unwrap_or(false)
    }

    fn global_budget_exhausted(&mut self, now_ms: i64) -> bool {
        const WINDOW_MS: i64 = 10 * 60 * 1000;
        if self.global_window_start_ms == 0
            || now_ms.saturating_sub(self.global_window_start_ms) > WINDOW_MS
        {
            self.global_window_start_ms = now_ms;
            self.global_count = 0;
        }

        self.global_count >= self.global_budget
    }

    fn record_start(&mut self, kind: IanMomentKind, now_ms: i64) {
        self.last_started_ms.insert(kind, now_ms);
        if consumes_global_budget(kind) {
            self.global_count = self.global_count.saturating_add(1);
        }
    }
}

pub fn memory_echo_text(tags: &[String]) -> Option<&'static str> {
    if tags.iter().any(|tag| tag == "pref:quiet") {
        return Some("我记得你喜欢安静一点。");
    }
    if tags.iter().any(|tag| tag == "topic:water") {
        return Some("我记得要慢慢喝水。");
    }
    None
}

fn blocks_context(kind: IanMomentKind, state: &IanState) -> bool {
    match kind {
        IanMomentKind::DragCarry => state.is_bubble_input_active,
        IanMomentKind::DropSettle => state.is_bubble_input_active,
        _ => state.is_dragging || state.is_bubble_input_active || state.current_animation == "run",
    }
}

fn blocks_user_control(kind: IanMomentKind, state: &IanState) -> bool {
    if matches!(
        kind,
        IanMomentKind::FindIanEntrance | IanMomentKind::DragCarry | IanMomentKind::DropSettle
    ) {
        return false;
    }

    matches!(state.playful_energy, crate::protocol::PlayfulEnergy::Off)
        || matches!(state.behavior_mode, crate::protocol::BehaviorMode::Quiet)
}

fn reduces_motion(kind: IanMomentKind) -> bool {
    !matches!(
        kind,
        IanMomentKind::FindIanEntrance | IanMomentKind::MemoryEcho
    )
}

fn consumes_global_budget(kind: IanMomentKind) -> bool {
    !matches!(kind, IanMomentKind::DragCarry | IanMomentKind::DropSettle)
}

#[cfg(test)]
mod tests {
    use crate::protocol::IanState;

    use super::{memory_echo_text, IanMomentKind, MomentOrchestrator};

    #[test]
    fn moment_orchestrator_applies_per_kind_cooldown() {
        let mut orchestrator = MomentOrchestrator::default();
        let state = IanState::default();

        let first = orchestrator.decide(IanMomentKind::PointerCuriosity, 1_000, &state);
        let repeated = orchestrator.decide(IanMomentKind::PointerCuriosity, 2_000, &state);
        let recovered = orchestrator.decide(IanMomentKind::PointerCuriosity, 12_000, &state);

        assert_eq!(first.result, "triggered");
        assert_eq!(repeated.result, "blocked_cooldown");
        assert_eq!(recovered.result, "triggered");
    }

    #[test]
    fn moment_orchestrator_limits_global_surprise_budget() {
        let mut orchestrator = MomentOrchestrator::default();
        let state = IanState::default();

        assert_eq!(
            orchestrator
                .decide(IanMomentKind::FindIanEntrance, 1_000, &state)
                .result,
            "triggered"
        );
        assert_eq!(
            orchestrator
                .decide(IanMomentKind::PointerCuriosity, 12_000, &state)
                .result,
            "triggered"
        );
        assert_eq!(
            orchestrator
                .decide(IanMomentKind::RareIdleSurprise, 30_000, &state)
                .result,
            "triggered"
        );
        assert_eq!(
            orchestrator
                .decide(IanMomentKind::MemoryEcho, 40_000, &state)
                .result,
            "triggered"
        );
        assert_eq!(
            orchestrator
                .decide(IanMomentKind::FindIanEntrance, 590_000, &state)
                .result,
            "blocked_budget"
        );
    }

    #[test]
    fn moment_orchestrator_blocks_reduced_motion_surprises() {
        let mut orchestrator = MomentOrchestrator::default();
        let mut state = IanState::default();
        state.movement_intensity = "reduced".to_string();

        assert_eq!(
            orchestrator
                .decide(IanMomentKind::RareIdleSurprise, 1_000, &state)
                .result,
            "blocked_reduced_motion"
        );
    }

    #[test]
    fn memory_echo_uses_only_confirmed_low_sensitive_tags() {
        assert_eq!(
            memory_echo_text(&["pref:quiet".to_string()]),
            Some("我记得你喜欢安静一点。")
        );
        assert_eq!(memory_echo_text(&["candidate:quiet".to_string()]), None);
    }
}
