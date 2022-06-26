use super::{
    observation::{DataRecording, Duration, Observation},
    raw_observation::RawObservation,
    timed_observation::{TimedObservation, CoreObservation},
};
use chrono::{Datelike, NaiveDate};
use csv::{ByteRecord, StringRecord};

//               ┌────────────────────────────────────┐
//               │                                    │
//               │  csv::{ByteRecord, StringRecord}   │
//               │                                    │
//               └─────────────────┬──────────────────┘
//                                 │
//                                 │
//                                 │
//                                 ▼
//                       ┌─────────────────────┐
//                       │ serde::Deserializer │
//                       └─────────┬───────────┘
//                                 │
//                                 │
//                                 │
// ┌───────────────────────►┌──────▼───────┐◄──────────────────────┐
// │                        │RawObservation│                       │
// │                ┌───────┴──────────────┴───────────┐           │
// │                │                                  │           │
// │                │                                  │           │
// │                │                                  │           │
// │                │                                  │           │
// │                │                                  ▼           │
// ├────────────────▼─────────────────────────────────►┌───────────┤
// │TimedObservation│                                  │Observation│
// └────────────────┘◄─────────────────────────────────┴───────────┘

const DATE_FORMAT: &str = "%Y%m%d %H%M";
const YEAR_FORMAT: &str = "%Y-%m-%d";
const CSV_ROW_LENGTH: usize = 9;

impl TryFrom<RawObservation> for Observation {
    type Error = ();

    fn try_from(value: RawObservation) -> Result<Self, Self::Error> {
        let station_id = value.station_id.trim().to_uppercase();
        let duration_result = match value.duration.to_uppercase().trim() {
            "D" => Ok(Duration::Daily),
            "M" => Ok(Duration::Monthly),
            _ => Err(()),
        };
        let date_observation =
            NaiveDate::parse_from_str(value.date_observation.trim(), DATE_FORMAT).unwrap();
        let date_recording =
            NaiveDate::parse_from_str(value.date_recording.trim(), DATE_FORMAT).unwrap();
        let data_value_result: Result<DataRecording, ()> = match value.value.trim() {
            "BRT" => Ok(DataRecording::Brt),
            "ART" => Ok(DataRecording::Art),
            "---" => Ok(DataRecording::Dash),
            s => match s.parse::<u32>() {
                Err(_p) => Ok(DataRecording::Recording(0u32)),
                Ok(u) => Ok(DataRecording::Recording(u)),
            },
            // _ => Err(()),
        };
        if let (Ok(duration), Ok(value)) = (duration_result, data_value_result) {
            Ok(Observation {
                station_id,
                date_recording,
                date_observation,
                value,
                duration,
            })
        } else {
            Err(())
        }
    }
}

impl TryFrom<RawObservation> for TimedObservation {
    type Error = ();

    fn try_from(value: RawObservation) -> Result<Self, Self::Error> {
        let obs: Result<Observation, ()> = Observation::try_from(value);
        match obs {
            Ok(o) => {
                let to: Result<TimedObservation, ()> = TimedObservation::try_from(o);
                to
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<Observation> for StringRecord {
    fn try_from(value: Observation) -> Result<Self, Self::Error> {
        //         r#"STATION_ID,DURATION,SENSOR_NUMBER,SENSOR_TYPE,DATE TIME,OBS DATE,VALUE,DATA_FLAG,UNITS
        // VIL,D,15,STORAGE,20220215 0000,20220215 0000,9593, ,AF";
        let station_id = value.station_id.to_uppercase();
        let station_id_str = station_id.as_str();
        let duration = match value.duration {
            Duration::Daily => "D",
            Duration::Monthly => "M",
        };
        let sensor_number = "15";
        let sensor_type = "STORAGE";
        let date_time = format!(
            "{}{:02}{:02} 0000",
            value.date_recording.year(),
            value.date_recording.month(),
            value.date_recording.day()
        );
        let date_time_str = date_time.as_str();
        let date_obs = format!(
            "{}{:02}{:02} 0000",
            value.date_observation.year(),
            value.date_observation.month(),
            value.date_observation.day()
        );
        let date_obs_str = date_obs.as_str();
        let val = match value.value {
            DataRecording::Recording(a) => a.to_string(),
            DataRecording::Art => String::from("ART"),
            DataRecording::Brt => String::from("BRT"),
            DataRecording::Dash => String::from("---"),
        };
        let val_str = val.as_str();
        let data_flag = "";
        let units = "AF";
        let b = ByteRecord::from(vec![
            station_id_str,
            duration,
            sensor_number,
            sensor_type,
            date_time_str,
            date_obs_str,
            val_str,
            data_flag,
            units,
        ]);
        match StringRecord::from_byte_record(b) {
            Ok(s) => Ok(s),
            Err(_) => Err(()),
        }
    }

    type Error = ();
}

impl TryFrom<Observation> for TimedObservation {
    type Error = ();

    fn try_from(value: Observation) -> Result<Self, Self::Error> {
        let core = CoreObservation {
            station_id: value.station_id,
            date_observation: value.date_observation,
            date_recording: value.date_recording,
            value: value.value,
        };
        match value.duration {
            Duration::Daily => Ok(TimedObservation::Daily(core)),
            Duration::Monthly => Ok(TimedObservation::Monthly(core)),
        }
    }
}

impl TryFrom<Observation> for RawObservation {
    type Error = ();

    fn try_from(value: Observation) -> Result<Self, Self::Error> {
        let station_id = value.station_id;
        let duration = {
            match value.duration {
                Duration::Daily => String::from("D"),
                Duration::Monthly => String::from("M"),
            }
        };
        let sensor_number = String::from("15");
        let sensor_type = String::from("STORAGE");
        let date_recording = format!(
            "{}{:02}{:02} 0000",
            value.date_recording.year(),
            value.date_recording.month(),
            value.date_recording.day()
        );
        let date_observation = format!(
            "{}{:02}{:02} 0000",
            value.date_observation.year(),
            value.date_observation.month(),
            value.date_observation.day()
        );
        let value = {
            match value.value {
                DataRecording::Art => String::from("ART"),
                DataRecording::Brt => String::from("BRT"),
                DataRecording::Dash => String::from("---"),
                DataRecording::Recording(a) => a.to_string(),
            }
        };
        let data_flag = String::from("");
        let units = String::from("AF");
        Ok(RawObservation {
            station_id,
            duration,
            sensor_number,
            sensor_type,
            date_recording,
            date_observation,
            value,
            data_flag,
            units,
        })
    }
}

impl TryFrom<TimedObservation> for Observation {
    type Error = ();

    fn try_from(value: TimedObservation) -> Result<Self, Self::Error> {
        match value {
            TimedObservation::Daily(core_obs) => Ok(Observation {
                station_id: core_obs.station_id,
                date_observation: core_obs.date_observation,
                date_recording: core_obs.date_recording,
                value: core_obs.value,
                duration: Duration::Daily,
            }),
            TimedObservation::Monthly(core_obs) => Ok(Observation {
                station_id: core_obs.station_id,
                date_observation: core_obs.date_observation,
                date_recording: core_obs.date_recording,
                value: core_obs.value,
                duration: Duration::Monthly,
            }),
        }
    }
}
