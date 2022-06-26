use super::{
    raw_observation::RawObservation,
    timed_observation::{CoreObservation, TimedObservation},
};
use chrono::NaiveDate;
use csv::{ByteRecord, StringRecord};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    str,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObservationError {
    HttpRequestError,
    HttpResponseParseError,
    ObservationCollectionError,
    FunctionFail,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Duration {
    Daily,
    Monthly,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataRecording {
    Brt,
    Art,
    Dash,
    Recording(u32),
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub station_id: String,
    pub date_observation: NaiveDate,
    pub date_recording: NaiveDate,
    pub value: DataRecording,
    pub duration: Duration,
}

impl PartialEq for Observation {
    fn eq(&self, other: &Self) -> bool {
        self.date_observation == other.date_observation
            && self.date_recording == other.date_recording
            && self.station_id == other.station_id
            && self.value == other.value
            && self.duration == other.duration
    }
}

impl Eq for Observation {}

// observations a and b can be compared iff the following is satisfied:
// 1. a.station_id == b.station_id,
// 2. a.duration == b.duration,
// an observation is partially ordered by date_observation
impl PartialOrd for Observation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.station_id != other.station_id {
            return None;
        }
        if self.duration != other.duration {
            return None;
        }
        Some(self.date_observation.cmp(&other.date_observation))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(
            self.date_observation.partial_cmp(&other.date_observation),
            Some(Ordering::Less)
        )
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
