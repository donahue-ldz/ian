use crate::protocol::{BehaviorMode, Position};

pub struct MovementBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Default for MovementBounds {
    fn default() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 1600.0,
            max_y: 1000.0,
        }
    }
}

pub struct MovementBoundaryPolicy {
    bounds: MovementBounds,
}

impl Default for MovementBoundaryPolicy {
    fn default() -> Self {
        Self::new(MovementBounds::default())
    }
}

impl MovementBoundaryPolicy {
    pub fn new(bounds: MovementBounds) -> Self {
        Self { bounds }
    }

    pub fn constrain_target(
        &self,
        from: Position,
        target: Position,
        mode: &BehaviorMode,
    ) -> Position {
        let max_step = match mode {
            BehaviorMode::Quiet => 40.0,
            BehaviorMode::Normal => 90.0,
            BehaviorMode::Lively => 140.0,
        };

        let limited = Position {
            x: limit_delta(from.x, target.x, max_step),
            y: limit_delta(from.y, target.y, max_step),
        };

        Position {
            x: limited.x.clamp(self.bounds.min_x, self.bounds.max_x),
            y: limited.y.clamp(self.bounds.min_y, self.bounds.max_y),
        }
    }
}

fn limit_delta(from: f64, target: f64, max_step: f64) -> f64 {
    let delta = target - from;
    if delta.abs() <= max_step {
        target
    } else {
        from + delta.signum() * max_step
    }
}

#[cfg(test)]
mod tests {
    use super::{MovementBoundaryPolicy, MovementBounds};
    use crate::protocol::{BehaviorMode, Position};

    #[test]
    fn clamps_targets_to_screen_safe_bounds() {
        let policy = MovementBoundaryPolicy::new(MovementBounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 320.0,
            max_y: 240.0,
        });

        let target = policy.constrain_target(
            Position { x: 100.0, y: 100.0 },
            Position { x: 999.0, y: -10.0 },
            &BehaviorMode::Lively,
        );

        assert!(target.x <= 320.0);
        assert!(target.y >= 0.0);
    }

    #[test]
    fn behavior_mode_limits_single_step_displacement() {
        let policy = MovementBoundaryPolicy::default();

        let quiet = policy.constrain_target(
            Position { x: 100.0, y: 100.0 },
            Position { x: 260.0, y: 100.0 },
            &BehaviorMode::Quiet,
        );
        let lively = policy.constrain_target(
            Position { x: 100.0, y: 100.0 },
            Position { x: 260.0, y: 100.0 },
            &BehaviorMode::Lively,
        );

        assert_eq!(quiet.x, 140.0);
        assert_eq!(lively.x, 240.0);
    }
}
