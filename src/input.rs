use thiserror::Error;

const SESSION_TOKEN: &str = env!("AOC_TOKEN");

#[derive(Debug, Error)]
pub enum InputError {
    #[error("building request client")]
    ClientBuilder(#[source] reqwest::Error),
    #[error("requesting input file")]
    RequestingInput(#[source] reqwest::Error),
    #[error("response status unsuccessful")]
    ResponseStatus(#[source] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("downloading to local file")]
    Downloading(#[source] reqwest::Error),
}

pub fn url_for_day(year: u32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

pub fn path_for_day(year: u32, day: u32) -> String {
    format!("inputs/{}/day-{:0>2}.txt", year, day)
}

pub fn get_input(year: u32, day: u32) -> Result<String, InputError> {
    let input_path = std::path::PathBuf::from(path_for_day(year, day));
    if !input_path.exists() {
        let client = reqwest::blocking::Client::builder()
            .gzip(true)
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(InputError::ClientBuilder)?;

        let mut response = client
            .get(&url_for_day(year, day))
            .header(
                reqwest::header::COOKIE,
                format!("session={}", SESSION_TOKEN),
            )
            .send()
            .map_err(InputError::RequestingInput)?
            .error_for_status()
            .map_err(InputError::ResponseStatus)?;

        if let Some(parent) = input_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let mut file = std::fs::File::create(input_path)?;
        response
            .copy_to(&mut file)
            .map_err(InputError::Downloading)?;
    }
    match std::fs::read_to_string(path_for_day(year, day)) {
        Ok(input) => return Ok(input),
        Err(err) => return Err(InputError::Io(err)),
    }
}
