use super::MoodState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoodAnimationCandidate {
    pub animation: &'static str,
    pub weight: u8,
}

#[derive(Debug, Default)]
pub struct MoodAnimationPolicy;

impl MoodAnimationPolicy {
    pub fn candidates_for(&self, mood: MoodState) -> Vec<MoodAnimationCandidate> {
        match mood {
            MoodState::Happy => vec![
                MoodAnimationCandidate {
                    animation: "happy",
                    weight: 5,
                },
                MoodAnimationCandidate {
                    animation: "walk",
                    weight: 2,
                },
            ],
            MoodState::Sleepy => vec![
                MoodAnimationCandidate {
                    animation: "sleep",
                    weight: 5,
                },
                MoodAnimationCandidate {
                    animation: "rest",
                    weight: 3,
                },
            ],
            MoodState::Bored => vec![
                MoodAnimationCandidate {
                    animation: "walk",
                    weight: 3,
                },
                MoodAnimationCandidate {
                    animation: "idle",
                    weight: 2,
                },
            ],
            MoodState::Calm => vec![
                MoodAnimationCandidate {
                    animation: "idle",
                    weight: 4,
                },
                MoodAnimationCandidate {
                    animation: "rest",
                    weight: 1,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MoodAnimationPolicy;
    use crate::domain::mood::MoodState;

    #[test]
    fn mood_animation_weights_are_deterministic_and_distinct() {
        let policy = MoodAnimationPolicy::default();
        let happy = policy.candidates_for(MoodState::Happy);
        let sleepy = policy.candidates_for(MoodState::Sleepy);

        assert!(happy
            .iter()
            .any(|candidate| { candidate.animation == "happy" && candidate.weight > 1 }));
        assert!(sleepy
            .iter()
            .any(|candidate| { candidate.animation == "sleep" && candidate.weight > 1 }));
        assert_ne!(happy, sleepy);
    }
}
