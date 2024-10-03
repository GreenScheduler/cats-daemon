use core::fmt;
use std::{fmt::{Display, Formatter}, path::PathBuf};
use dirs::data_dir;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    refresh_time: RefreshTime,
    endpoint: String,
    postcode: String,
    #[serde(default)]
    data_dir: DataDir,
}

#[derive(Debug, Deserialize)]
pub struct DataDir(PathBuf);
impl Default for DataDir {
    fn default() -> Self {
        DataDir(data_dir().expect("Could not find system Data Dir").join("cats"))
    }
}

#[derive(Debug, Deserialize)]
pub struct RefreshTime(u64);
impl Default for RefreshTime {
    fn default() -> Self {
        RefreshTime(1)
    }
}

impl Config {
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir.0
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn postcode(&self) -> &str {
        &self.postcode
    }

    pub fn refresh_time(&self) -> u64 {
        self.refresh_time.0 
    }
    /* pub fn refresh_time_secs(&self) -> u64 {
        self.refresh_time.0 * 60 * 60
    } */
}   

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Running CATS daemon with the following settings:")?;
        writeln!(f, "Refresh time: {} hour", self.refresh_time())?;
        writeln!(f, "API endpoint: {}", self.endpoint())?;
        writeln!(f, "Server Postcode: {}", self.postcode())?;
        writeln!(f, "Data directory: {}", self.data_dir().as_os_str().to_str().expect("Could not parse Data Dir"))?;
        Ok(())
    }
}
