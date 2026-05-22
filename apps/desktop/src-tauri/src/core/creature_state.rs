use crate::protocol::{
    BehaviorMode, CurrentBehavior, DeveloperSnooze, DeveloperWorkspace, IanAction, IanState,
    PlayfulDiagnostic, PlayfulEnergy, PlayfulState, Position, QuietHours, ScreenBounds,
};

pub struct CreatureState {
    state: IanState,
}

impl CreatureState {
    pub fn new(state: IanState) -> Self {
        Self { state }
    }

    pub fn snapshot(&self) -> &IanState {
        &self.state
    }

    pub fn set_position(&mut self, position: Position) {
        self.state.position = position.clone();
        self.state.home_anchor = position;
        self.state.is_dragging = false;
    }

    pub fn set_behavior_mode(&mut self, mode: BehaviorMode) {
        self.state.behavior_mode = mode;
    }

    pub fn set_active_pet(&mut self, active_pet_id: String) {
        self.state.active_pet_id = active_pet_id.clone();
        self.state.active_resource_pack = active_pet_id;
    }

    pub fn set_quiet_hours(&mut self, quiet_hours: QuietHours) {
        self.state.quiet_hours = quiet_hours;
    }

    pub fn set_day_phase(&mut self, day_phase: String) {
        self.state.day_phase = day_phase;
    }

    pub fn set_screen_bounds(&mut self, bounds: ScreenBounds) {
        self.state.screen_bounds = Some(bounds);
    }

    pub fn record_user_interaction(&mut self, now_ms: i64) {
        self.state.last_user_interaction_ms = now_ms;
    }

    pub fn set_creature_settings(
        &mut self,
        movement_intensity: String,
        bubble_frequency: String,
        rest_behavior: String,
        playful_energy: PlayfulEnergy,
        playful_snoozed_until_ms: Option<i64>,
        surface_scale: f64,
        diagnostics_enabled: bool,
    ) {
        self.state.movement_intensity = movement_intensity;
        self.state.bubble_frequency = bubble_frequency;
        self.state.rest_behavior = rest_behavior;
        self.state.playful_energy = playful_energy;
        self.state.playful_snoozed_until_ms = playful_snoozed_until_ms;
        self.state.surface_scale = surface_scale.clamp(0.8, 1.4);
        self.state.diagnostics_enabled = diagnostics_enabled;
    }

    pub fn set_reminders_enabled(&mut self, enabled: bool) {
        self.state.reminders_enabled = enabled;
    }

    pub fn set_reminder_settings(&mut self, enabled: bool, interval_minutes: u16) {
        self.state.reminders_enabled = enabled;
        self.state.reminder_interval_minutes = interval_minutes.clamp(15, 240);
    }

    pub fn set_do_not_disturb(&mut self, enabled: bool) {
        self.state.do_not_disturb = enabled;
    }

    pub fn set_privacy_onboarding_seen(&mut self, seen: bool) {
        self.state.privacy_onboarding_seen = seen;
    }

    pub fn set_find_ian_shortcut_enabled(&mut self, enabled: bool) {
        self.state.find_ian_shortcut_enabled = enabled;
    }

    pub fn set_byom_key_configured(&mut self, configured: bool) {
        self.state.byom_key_configured = configured;
        if !configured {
            self.state.byom_enabled = false;
        }
    }

    pub fn set_dragging(&mut self, is_dragging: bool) {
        self.state.is_dragging = is_dragging;
    }

    pub fn set_bubble_input_active(&mut self, is_active: bool) {
        self.state.is_bubble_input_active = is_active;
    }

    pub fn set_developer_workspace(&mut self, workspace: DeveloperWorkspace) {
        self.state.developer_workspace = workspace;
    }

    pub fn set_developer_snooze(&mut self, snooze: DeveloperSnooze) {
        self.state.developer_snooze = snooze;
    }

    pub fn set_active_app_category(&mut self, category: Option<String>) {
        self.state.active_app_category = category;
    }

    pub fn set_capability_enabled(&mut self, capability: &str, enabled: bool) -> bool {
        match capability {
            "byom" => self.state.byom_enabled = enabled,
            "git_metadata" => self.state.git_metadata_enabled = enabled,
            "build_test_events" => self.state.build_test_events_enabled = enabled,
            "keyboard_rhythm" => self.state.keyboard_rhythm_enabled = enabled,
            "active_app_presence" => self.state.active_app_presence_enabled = enabled,
            _ => return false,
        }
        true
    }

    pub fn apply_actions(&mut self, actions: &[IanAction]) {
        for action in actions {
            match action {
                IanAction::AnimationPlay { name, .. } => {
                    self.state.current_animation = name.clone();
                    self.state.current_behavior = match name.as_str() {
                        "rest" => CurrentBehavior::Resting,
                        "walk" => CurrentBehavior::Walking,
                        "happy" => CurrentBehavior::Happy,
                        "run" => CurrentBehavior::Running,
                        "zoomies" => CurrentBehavior::Zooming,
                        "sleep" => CurrentBehavior::Sleeping,
                        _ => CurrentBehavior::Idle,
                    };
                }
                IanAction::BehaviorRunAround { .. } => {
                    self.state.current_animation = "run".to_string();
                    self.state.current_behavior = CurrentBehavior::Running;
                }
                IanAction::BehaviorZoomies { duration_ms, .. } => {
                    self.state.current_animation = "zoomies".to_string();
                    self.state.current_behavior = CurrentBehavior::Zooming;
                    self.state.playful_state = PlayfulState::Zooming;
                    self.state.playful_state_until_ms = Some(*duration_ms as i64);
                }
                IanAction::PlayfulStateSet { state, until_ms } => {
                    self.state.playful_state = state.clone();
                    self.state.playful_state_until_ms = *until_ms;
                }
                IanAction::PlayfulDiagnostic {
                    timestamp_ms,
                    reason,
                    result,
                    cooldown_key,
                    chosen_reaction_key,
                } => {
                    self.state.last_playful_diagnostic = Some(PlayfulDiagnostic {
                        timestamp_ms: *timestamp_ms,
                        reason: reason.clone(),
                        result: result.clone(),
                        cooldown_key: cooldown_key.clone(),
                        chosen_reaction_key: chosen_reaction_key.clone(),
                    });
                }
                IanAction::MovementMoveTo { x, y, .. } => {
                    self.state.position = Position { x: *x, y: *y };
                }
                IanAction::AppearanceScaleTo { .. } => {}
                IanAction::StateSync { state } => {
                    self.state = state.clone();
                }
                IanAction::SpeechShow { .. }
                | IanAction::BubbleOpen
                | IanAction::BubbleClose
                | IanAction::EffectPlay { .. } => {}
            }
        }
    }
}
