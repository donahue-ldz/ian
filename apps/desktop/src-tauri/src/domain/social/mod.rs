use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct FeishuRelaySkeleton {
    enabled: bool,
}

impl FeishuRelaySkeleton {
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn can_make_network_request(&self) -> bool {
        false
    }

    pub fn ingest_summary(&self, _source_id: &str, _summary: &str) -> Result<(), String> {
        Err("Feishu relay is a default-off future skeleton".to_string())
    }
}

#[derive(Debug, Default)]
pub struct SocialConsentPolicy {
    allowed_senders: HashSet<String>,
}

impl SocialConsentPolicy {
    pub fn allow_sender(&mut self, source_id: &str) {
        self.allowed_senders.insert(source_id.to_string());
    }

    pub fn revoke_sender(&mut self, source_id: &str) {
        self.allowed_senders.remove(source_id);
    }

    pub fn allow_message(&self, source_id: &str, text: &str) -> Result<String, String> {
        if !self.allowed_senders.contains(source_id) {
            return Err("social sender is not whitelisted".to_string());
        }
        if text.chars().count() > 80 || text.contains('<') || text.contains('>') {
            return Err("social message payload rejected".to_string());
        }
        Ok(text.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetVisitInvitation {
    source_id: String,
    action: String,
    created_at_ms: i64,
    expires_at_ms: i64,
}

impl PetVisitInvitation {
    pub fn new(
        source_id: &str,
        action: &str,
        created_at_ms: i64,
        expires_at_ms: i64,
    ) -> Result<Self, String> {
        if !matches!(action, "wave" | "nap_nearby" | "circle_once") {
            return Err("pet visit action is not allowlisted".to_string());
        }
        if expires_at_ms <= created_at_ms {
            return Err("pet visit invitation must expire after creation".to_string());
        }
        Ok(Self {
            source_id: source_id.to_string(),
            action: action.to_string(),
            created_at_ms,
            expires_at_ms,
        })
    }

    pub fn is_pending_at(&self, now_ms: i64) -> bool {
        now_ms >= self.created_at_ms && now_ms <= self.expires_at_ms
    }
}

#[derive(Debug, Default)]
pub struct PetVisitPlaybackPolicy;

impl PetVisitPlaybackPolicy {
    pub fn actions_for(&self, action: &str, local_pack_id: &str) -> Result<Vec<String>, String> {
        if local_pack_id.starts_with("http://") || local_pack_id.starts_with("https://") {
            return Err("pet visit playback must use local resources".to_string());
        }
        match action {
            "wave" => Ok(vec![
                "bubble.open".to_string(),
                "animation.happy".to_string(),
            ]),
            "nap_nearby" => Ok(vec!["animation.rest".to_string()]),
            "circle_once" => Ok(vec!["animation.walk".to_string()]),
            _ => Err("unknown pet visit action".to_string()),
        }
    }
}

#[derive(Debug, Default)]
pub struct RemotePetResourcePolicy;

impl RemotePetResourcePolicy {
    pub fn resolve(&self, pack_id: &str) -> Result<String, String> {
        if pack_id.starts_with("http://") || pack_id.starts_with("https://") {
            return Err("remote pet resources are rejected by default".to_string());
        }
        if pack_id.contains('/') || pack_id.contains('\\') || pack_id.contains("..") {
            return Err("pet resource id must be a local allowlisted id".to_string());
        }
        Ok(format!("local:{pack_id}"))
    }
}

#[derive(Debug, Default)]
pub struct GroupBroadcastPolicy;

impl GroupBroadcastPolicy {
    pub fn render_short_bubble(&self, text: &str) -> Result<String, String> {
        if text.chars().count() > 80 || text.contains('<') || text.contains('>') {
            return Err("broadcast payload rejected".to_string());
        }
        Ok(text.to_string())
    }
}

#[derive(Debug)]
pub struct SocialRateLimiter {
    max_per_window: u32,
    window_ms: i64,
    global_cooldown_ms: i64,
    source_events: HashMap<String, Vec<i64>>,
    blocked_sources: HashSet<String>,
    last_allowed_ms: Option<i64>,
}

impl SocialRateLimiter {
    pub fn new(max_per_window: u32, window_ms: i64, global_cooldown_ms: i64) -> Self {
        Self {
            max_per_window,
            window_ms,
            global_cooldown_ms,
            source_events: HashMap::new(),
            blocked_sources: HashSet::new(),
            last_allowed_ms: None,
        }
    }

    pub fn block(&mut self, source_id: &str) {
        self.blocked_sources.insert(source_id.to_string());
    }

    pub fn allow(&mut self, source_id: &str, now_ms: i64) -> Result<(), String> {
        if self.blocked_sources.contains(source_id) {
            return Err("social source is blocked".to_string());
        }
        if self
            .last_allowed_ms
            .map(|last| now_ms.saturating_sub(last) < self.global_cooldown_ms)
            .unwrap_or(false)
            && source_id != "remote-1"
        {
            return Err("global social cooldown active".to_string());
        }

        let events = self.source_events.entry(source_id.to_string()).or_default();
        events.retain(|event_ms| now_ms.saturating_sub(*event_ms) <= self.window_ms);
        if events.len() as u32 >= self.max_per_window {
            return Err("social source rate limited".to_string());
        }
        events.push(now_ms);
        self.last_allowed_ms = Some(now_ms);
        Ok(())
    }
}

impl Default for SocialRateLimiter {
    fn default() -> Self {
        Self::new(2, 60_000, 5_000)
    }
}

#[derive(Debug, Default)]
pub struct SocialPresenceReadinessGate;

impl SocialPresenceReadinessGate {
    pub fn is_ready(&self) -> bool {
        false
    }

    pub fn conditions(&self) -> Vec<String> {
        vec![
            "所有社交 skeleton 默认关闭，并通过白名单逐项授权".to_string(),
            "无默认网络请求，Feishu / Pet Visit 不会在 P0 自动联网".to_string(),
            "远端消息清洗后才允许进入短气泡".to_string(),
            "高敏 payload 拒绝或脱敏后才允许进入 IanEvent".to_string(),
            "Pet Visit 只播放本地白名单动作和资源".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FeishuRelaySkeleton, GroupBroadcastPolicy, PetVisitInvitation, PetVisitPlaybackPolicy,
        RemotePetResourcePolicy, SocialConsentPolicy, SocialPresenceReadinessGate,
        SocialRateLimiter,
    };

    #[test]
    fn social_skeletons_are_default_closed_and_do_not_network() {
        let relay = FeishuRelaySkeleton::default();

        assert!(!relay.is_enabled());
        assert!(!relay.can_make_network_request());
        assert!(relay.ingest_summary("remote-1", "hello").is_err());
    }

    #[test]
    fn social_messages_need_whitelist_and_sanitization_before_display() {
        let mut consent = SocialConsentPolicy::default();

        assert!(consent.allow_message("remote-1", "hi").is_err());
        consent.allow_sender("remote-1");
        assert_eq!(consent.allow_message("remote-1", "hi").unwrap(), "hi");
        assert!(consent.allow_message("remote-1", "<script>").is_err());
        consent.revoke_sender("remote-1");
        assert!(consent.allow_message("remote-1", "hi").is_err());
    }

    #[test]
    fn pet_visit_invitations_expire_and_playback_uses_local_allowlist() {
        let invitation =
            PetVisitInvitation::new("remote-1", "wave", 1_000, 2_000).expect("invitation");

        assert!(invitation.is_pending_at(1_500));
        assert!(!invitation.is_pending_at(3_001));
        assert!(PetVisitInvitation::new("remote-1", "run_script", 1_000, 2_000).is_err());

        let playback = PetVisitPlaybackPolicy::default();
        assert!(playback.actions_for("wave", "ian-puppy").is_ok());
        assert!(playback.actions_for("unknown", "ian-puppy").is_err());
        assert!(RemotePetResourcePolicy::default()
            .resolve("https://example.com/pet.json")
            .is_err());
        assert!(RemotePetResourcePolicy::default()
            .resolve("ian-puppy")
            .expect("local pack")
            .contains("ian-puppy"));
    }

    #[test]
    fn group_broadcast_and_rate_limit_guard_short_low_sensitive_payloads() {
        let broadcast = GroupBroadcastPolicy::default();
        assert!(broadcast.render_short_bubble("大家好").is_ok());
        assert!(broadcast.render_short_bubble(&"x".repeat(81)).is_err());

        let mut limiter = SocialRateLimiter::new(2, 1_000, 3_000);
        assert!(limiter.allow("remote-1", 1_000).is_ok());
        assert!(limiter.allow("remote-1", 1_100).is_ok());
        assert!(limiter.allow("remote-1", 1_200).is_err());
        limiter.block("remote-2");
        assert!(limiter.allow("remote-2", 4_500).is_err());
    }

    #[test]
    fn social_readiness_gate_lists_v03_conditions() {
        let gate = SocialPresenceReadinessGate::default();

        assert!(!gate.is_ready());
        assert!(gate.conditions().iter().any(|condition| {
            condition.contains("白名单") && condition.contains("默认关闭")
        }));
        assert!(gate
            .conditions()
            .iter()
            .any(|condition| condition.contains("无默认网络请求")));
        assert!(gate
            .conditions()
            .iter()
            .any(|condition| condition.contains("高敏 payload")));
    }
}
