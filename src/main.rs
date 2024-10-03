mod api;
use api::CarbonIntensityAPI;
use dirs::config_dir;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;

mod data;

mod config;
use config::Config;

fn main() {

    // Get the input args
    let args: Vec<String> = std::env::args().collect();
    let config = match args.len() < 2 {
        true => {
            // If input args is less than 2, the read the settings from the default file
            config_dir()
                .expect("Could not find the system's config directory")
                .join("cats.yaml")
        }
        false => {
            // If input args is greater than 2, then read the settings filename from the input args
            PathBuf::from(&args[1])
        }
    };
    println!("Loading default config file from {:?}", config);

    // Load the config file
    let config_file = std::fs::read_to_string(config).expect("Could not read the config file");
    let config: Config =
        serde_yaml::from_str(&config_file).expect("Could not parse the config file");

    // check if the data directory exists and create it if it does not
    if !config.data_dir().exists() {
        std::fs::create_dir_all(config.data_dir()).expect("Could not create the data directory");
    }

    // show the config settings
    println!("{}", config);

    let api = CarbonIntensityAPI::new(&config);

    match fetch_data(&api, &config) {
        Ok(_) => {
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            exit(1);
        }
    }
}

fn fetch_data(api: &CarbonIntensityAPI, config: &Config) -> Result<(), reqwest::Error> {
    loop {
        let results = api.fetch()?;

        let data_file = config.data_dir().join("data.json");

        // check if the data file exists
        if data_file.exists() {
            // if the file does exists, then delete it
            std::fs::remove_file(&data_file).expect("Could not delete the data file");
        }

        // write the data to the file
        serde_json::to_writer(
            &File::create(data_file).expect("Could not create the data file"),
            &results,
        )
        .expect("Could not write the data to the file");

        std::thread::sleep(std::time::Duration::from_secs(config.refresh_time()));
    }
}
