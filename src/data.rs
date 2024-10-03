
use chrono::DateTime;
use chrono::Utc;

use serde::ser::SerializeStruct;
use serde::Serialize;
#[derive(Default, Debug, Clone)]
pub struct CarbonIntensity {
   pub from: DateTime<Utc>,
   pub to: DateTime<Utc>,
   pub intensity: i64,
}

// implement serliaze for CarbonIntensity
impl Serialize for CarbonIntensity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CarbonIntensity", 3)?;
        state.serialize_field("from", &self.from.to_string())?;
        state.serialize_field("to", &self.to.to_string())?;
        state.serialize_field("intensity", &self.intensity)?;
        state.end()
    }
}
