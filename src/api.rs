use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::data::CarbonIntensity;
use crate::config::Config;

use reqwest::blocking::Client;

pub struct CarbonIntensityAPI {
    url: String,
    endpoint: String,
    postcode: String,
    timestamp: Option<DateTime<Utc>>,
    client: Client,
}

impl CarbonIntensityAPI {
    ///
    /// Creates a new [`CarbonIntensityAPI`].
    pub fn new(config: &Config) -> CarbonIntensityAPI {
        let postcode = config.postcode();
        let postcode = match postcode.chars().collect::<Vec<char>>().len() > 4 {
            true => {
                eprint!("Warning: truncating postcode {} to", postcode);
                let postcode = postcode.chars().take(4).collect::<String>();
                eprintln!(" {}\n", postcode);
                postcode
            }
            false => postcode.to_string(),
        };

        let client = Client::new();
        CarbonIntensityAPI {
            url: config.endpoint().to_string(),
            endpoint: "/fw48h/postcode/".to_string(),
            postcode,
            timestamp: None,
            client,
        }
    }

    /* /// Sets the time for which the carbon intensity data should be fetched.
    /// If no time is set, the current time is used.
    pub fn set_time(mut self, time: DateTime<Utc>) {
        self.timestamp = Some(time)
    } */

    /// Fetches the carbon intensity data from the API.
    pub fn fetch(&self) -> Result<Vec<CarbonIntensity>, reqwest::Error> {
        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub data: Data,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub regionid: i64,
            pub dnoregion: String,
            pub shortname: String,
            pub postcode: String,
            pub data: Vec<Daum>,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Daum {
            pub from: String,
            pub to: String,
            pub intensity: Intensity,
            pub generationmix: Vec<Generationmix>,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Intensity {
            pub forecast: i64,
            pub index: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Generationmix {
            pub fuel: String,
            pub perc: f64,
        }

        let timestamp = match self.timestamp {
            Some(timestamp) => timestamp.format("%Y-%m-%dT%H:%MZ"),
            None => Utc::now().format("%Y-%m-%dT%H:%MZ"),
        };

        let url = format!(
            "{}{}{}{}",
            self.url, timestamp, self.endpoint, self.postcode
        );

        let response = self.client.get(&url).send()?;
        let body = response.text()?;

        let data: Root = serde_json::from_str(&body).unwrap();
        let intensity = data
            .data
            .data
            .iter()
            .map(|daum| CarbonIntensity {
                from: NaiveDateTime::parse_from_str(&daum.from, "%Y-%m-%dT%H:%MZ")
                    .unwrap()
                    .and_utc(),
                to: NaiveDateTime::parse_from_str(&daum.to, "%Y-%m-%dT%H:%MZ")
                    .unwrap()
                    .and_utc(),
                intensity: daum.intensity.forecast,
            })
            .collect::<Vec<CarbonIntensity>>();

        Ok(intensity)
    }
}
