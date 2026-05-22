use crate::protocol::IanAction;
pub use crate::protocol::MotionProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MomentBeatKind {
    Discover,
    Pause,
    Act,
    Settle,
}

#[derive(Debug, Clone)]
pub struct MomentBeat {
    pub kind: MomentBeatKind,
    action: Option<IanAction>,
    duration_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct MomentStory {
    _key: &'static str,
    beats: Vec<MomentBeat>,
}

impl MomentStory {
    pub fn new(key: &'static str) -> Self {
        Self {
            _key: key,
            beats: Vec::new(),
        }
    }

    pub fn discover(mut self, action: IanAction) -> Self {
        self.beats.push(MomentBeat {
            kind: MomentBeatKind::Discover,
            action: Some(action),
            duration_ms: None,
        });
        self
    }

    pub fn pause(mut self, duration_ms: u64) -> Self {
        self.beats.push(MomentBeat {
            kind: MomentBeatKind::Pause,
            action: None,
            duration_ms: Some(duration_ms),
        });
        self
    }

    pub fn act(mut self, action: IanAction) -> Self {
        self.beats.push(MomentBeat {
            kind: MomentBeatKind::Act,
            action: Some(action),
            duration_ms: None,
        });
        self
    }

    pub fn settle(mut self, action: IanAction) -> Self {
        self.beats.push(MomentBeat {
            kind: MomentBeatKind::Settle,
            action: Some(action),
            duration_ms: None,
        });
        self
    }

    pub fn beats(&self) -> &[MomentBeat] {
        &self.beats
    }

    pub fn into_actions(self, reduced_motion: bool) -> Vec<IanAction> {
        self.beats
            .into_iter()
            .filter_map(|beat| beat.into_action(reduced_motion))
            .collect()
    }
}

impl MomentBeat {
    fn into_action(self, reduced_motion: bool) -> Option<IanAction> {
        match self.kind {
            MomentBeatKind::Pause => {
                let _ = self.duration_ms;
                None
            }
            MomentBeatKind::Act if reduced_motion => match self.action {
                Some(IanAction::MovementMoveTo {
                    profile: MotionProfile::Playful,
                    ..
                }) => None,
                action => action,
            },
            _ => self.action,
        }
    }
}
