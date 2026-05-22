use crate::protocol::IanEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifeDriveSnapshot {
    pub curiosity: u8,
    pub comfort: u8,
    pub boredom: u8,
    pub affection: u8,
    pub energy: u8,
}

impl LifeDriveSnapshot {
    pub fn as_public_score(&self) -> Option<u8> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct LifeDrive {
    snapshot: LifeDriveSnapshot,
}

impl Default for LifeDrive {
    fn default() -> Self {
        Self {
            snapshot: LifeDriveSnapshot {
                curiosity: 42,
                comfort: 56,
                boredom: 18,
                affection: 32,
                energy: 54,
            },
        }
    }
}

impl LifeDrive {
    pub fn snapshot(&self) -> LifeDriveSnapshot {
        self.snapshot
    }

    pub fn record_event(&mut self, event: &IanEvent) {
        match event {
            IanEvent::MouseClick { .. } => {
                self.bump_affection(7);
                self.bump_curiosity(4);
                self.lower_boredom(8);
            }
            IanEvent::MouseNear { .. } | IanEvent::MouseChaseCandidate { .. } => {
                self.bump_curiosity(6);
                self.lower_boredom(3);
            }
            IanEvent::MouseDragStart { .. } => {
                self.lower_comfort(8);
                self.bump_affection(3);
                self.bump_energy(2);
            }
            IanEvent::MouseDragEnd { .. } => {
                self.bump_comfort(12);
                self.lower_energy(2);
            }
            IanEvent::SystemShortcutTriggered { action, .. } if action == "find_ian" => {
                self.bump_affection(5);
                self.bump_curiosity(3);
            }
            IanEvent::TimeTick { .. } => {
                self.bump_boredom(12);
                self.lower_energy(1);
                self.lower_curiosity(1);
            }
            _ => {}
        }
    }

    fn bump_curiosity(&mut self, amount: u8) {
        self.snapshot.curiosity = saturating_add(self.snapshot.curiosity, amount);
    }

    fn lower_curiosity(&mut self, amount: u8) {
        self.snapshot.curiosity = self.snapshot.curiosity.saturating_sub(amount);
    }

    fn bump_comfort(&mut self, amount: u8) {
        self.snapshot.comfort = saturating_add(self.snapshot.comfort, amount);
    }

    fn lower_comfort(&mut self, amount: u8) {
        self.snapshot.comfort = self.snapshot.comfort.saturating_sub(amount);
    }

    fn bump_boredom(&mut self, amount: u8) {
        self.snapshot.boredom = saturating_add(self.snapshot.boredom, amount);
    }

    fn lower_boredom(&mut self, amount: u8) {
        self.snapshot.boredom = self.snapshot.boredom.saturating_sub(amount);
    }

    fn bump_affection(&mut self, amount: u8) {
        self.snapshot.affection = saturating_add(self.snapshot.affection, amount);
    }

    fn bump_energy(&mut self, amount: u8) {
        self.snapshot.energy = saturating_add(self.snapshot.energy, amount);
    }

    fn lower_energy(&mut self, amount: u8) {
        self.snapshot.energy = self.snapshot.energy.saturating_sub(amount);
    }
}

fn saturating_add(value: u8, amount: u8) -> u8 {
    value.saturating_add(amount).min(100)
}
