use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawObservation {
    #[serde(rename = "STATION_ID")]
    pub station_id: String,
    #[serde(rename = "DURATION")]
    pub duration: String,
    #[serde(rename = "SENSOR_NUMBER")]
    pub sensor_number: String,
    #[serde(rename = "SENSOR_TYPE")]
    pub sensor_type: String,
    #[serde(rename = "DATE TIME")]
    pub date_recording: String,
    #[serde(rename = "OBS DATE")]
    pub date_observation: String,
    #[serde(rename = "VALUE")]
    pub value: String,
    #[serde(rename = "DATA_FLAG")]
    pub data_flag: String,
    #[serde(rename = "UNITS")]
    pub units: String,
}

#[cfg(test)]
mod test {
    use super::RawObservation;
    use csv::ReaderBuilder;
    use serde::Deserialize;
    const STR_RESULT: &str = r#"STATION_ID,DURATION,SENSOR_NUMBER,SENSOR_TYPE,DATE TIME,OBS DATE,VALUE,DATA_FLAG,UNITS
VIL,D,15,STORAGE,20220215 0000,20220215 0000,9593, ,AF
VIL,D,15,STORAGE,20220216 0000,20220216 0000,9589, ,AF
VIL,D,15,STORAGE,20220217 0000,20220217 0000,9589, ,AF
VIL,D,15,STORAGE,20220218 0000,20220218 0000,9585, ,AF
VIL,D,15,STORAGE,20220219 0000,20220219 0000,9585, ,AF
VIL,D,15,STORAGE,20220220 0000,20220220 0000,9585, ,AF
VIL,D,15,STORAGE,20220221 0000,20220221 0000,9581, ,AF
VIL,D,15,STORAGE,20220222 0000,20220222 0000,9593, ,AF
VIL,D,15,STORAGE,20220223 0000,20220223 0000,9601, ,AF
VIL,D,15,STORAGE,20220224 0000,20220224 0000,9601, ,AF
VIL,D,15,STORAGE,20220225 0000,20220225 0000,9601, ,AF
VIL,D,15,STORAGE,20220226 0000,20220226 0000,9597, ,AF
VIL,D,15,STORAGE,20220227 0000,20220227 0000,9597, ,AF
VIL,D,15,STORAGE,20220228 0000,20220228 0000,9597, ,AF
"#;
    #[test]
    fn test_daily_observation_serde_deserialization() {
        let expected = String::from("VIL");
        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(STR_RESULT.as_bytes());
        for result in rdr.deserialize() {
            let record: RawObservation = result.unwrap();
            assert_eq!(expected, record.station_id);
        }
    }
}
