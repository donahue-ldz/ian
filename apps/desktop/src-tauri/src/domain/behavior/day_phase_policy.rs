use chrono::{Local, TimeZone, Timelike};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayPhase {
    Morning,
    Day,
    Evening,
    Night,
}

impl DayPhase {
    pub fn from_local_hour(hour: u32) -> Self {
        match hour {
            5..=10 => Self::Morning,
            11..=16 => Self::Day,
            17..=21 => Self::Evening,
            _ => Self::Night,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Morning => "morning",
            Self::Day => "day",
            Self::Evening => "evening",
            Self::Night => "night",
        }
    }
}

pub fn minute_of_day_from_epoch_ms(now_ms: i64) -> u16 {
    let Some(datetime) = Local.timestamp_millis_opt(now_ms).single() else {
        return 0;
    };
    (datetime.hour() * 60 + datetime.minute()) as u16
}

pub fn phase_from_epoch_ms(now_ms: i64) -> DayPhase {
    let minute = minute_of_day_from_epoch_ms(now_ms);
    DayPhase::from_local_hour((minute / 60) as u32)
}

#[cfg(test)]
mod tests {
    use super::DayPhase;

    #[test]
    fn day_phase_has_predictable_boundaries() {
        assert!(matches!(DayPhase::from_local_hour(5), DayPhase::Morning));
        assert!(matches!(DayPhase::from_local_hour(11), DayPhase::Day));
        assert!(matches!(DayPhase::from_local_hour(17), DayPhase::Evening));
        assert!(matches!(DayPhase::from_local_hour(22), DayPhase::Night));
        assert!(matches!(DayPhase::from_local_hour(3), DayPhase::Night));
    }

    #[test]
    fn epoch_minute_uses_a_valid_local_day_minute() {
        assert!(super::minute_of_day_from_epoch_ms(23 * 60 * 60 * 1000) < 24 * 60);
    }
}
