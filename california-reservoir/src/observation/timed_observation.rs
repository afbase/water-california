use super::observation::DataRecording;
use chrono::NaiveDate;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    str,
};
#[derive(Debug, Clone)]
pub struct CoreObservation {
    pub station_id: String,
    pub date_observation: NaiveDate,
    pub date_recording: NaiveDate,
    pub value: DataRecording,
}
#[derive(Debug, Clone)]
pub enum TimedObservation {
    Monthly(CoreObservation),
    Daily(CoreObservation),
}

impl PartialEq for CoreObservation {
    fn eq(&self, other: &Self) -> bool {
        self.station_id == other.station_id
            && self.date_observation == other.date_observation
            && self.date_recording == other.date_recording
            && self.value == other.value
    }
}

impl Eq for CoreObservation {}

impl PartialEq for TimedObservation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Monthly(l0), Self::Monthly(r0)) => l0 == r0,
            (Self::Daily(l0), Self::Daily(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for TimedObservation {}

// observations a and b can be compared iff the following is satisfied:
// 1. a.station_id == b.station_id,
// 2. a.duration == b.duration,
// an observation is partially ordered by date_observation
impl PartialOrd for TimedObservation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (TimedObservation::Daily(a), TimedObservation::Daily(b)) => {
                if a.station_id != b.station_id {
                    return None;
                }
                Some(a.date_observation.cmp(&b.date_observation))
            }
            (TimedObservation::Monthly(a), TimedObservation::Monthly(b)) => {
                if a.station_id != b.station_id {
                    return None;
                }
                Some(a.date_observation.cmp(&b.date_observation))
            }
            _ => None,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        // Pattern `Some(Less | Eq)` optimizes worse than negating `None | Some(Greater)`.
        // FIXME: The root cause was fixed upstream in LLVM with:
        // https://github.com/llvm/llvm-project/commit/9bad7de9a3fb844f1ca2965f35d0c2a3d1e11775
        // Revert this workaround once support for LLVM 12 gets dropped.
        !matches!(self.partial_cmp(other), None | Some(Ordering::Greater))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}
