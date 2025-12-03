pub mod error {
    use thiserror::Error;

    #[derive(Error, Clone, Debug, PartialEq)]
    pub enum AocError {
        #[error("Couldn't get rule from str, {0}")]
        FromStrError(String),
        #[error("Couldn't parse direction from char, {0}")]
        ParseDirectionError(char),
        #[error("Couldn't parse point type from char, {0}")]
        ParsePointTypeError(char),
        #[error("Couldn't parse year from env var, {0}")]
        ParseYearFromEnvVarError(String),
        #[error("Couldn't get remote data, {0}")]
        GetRemoteDataError(String),
        #[error("Couldn't write local data, {0}")]
        WriteLocalDataError(String),
        #[error("Invalid part number: {0}")]
        InvalidPartNumberError(u8),
    }
}

pub mod data {
    use std::fs;

    pub fn load_data(day: u8) -> Result<String, super::error::AocError> {
        let local_path = get_local_path(day);
        if let Some(data) = try_loading_cached_data(local_path.clone()) {
            return Ok(data);
        }
        let data = get_remote_data(day)?;
        fs::write(local_path, &data)
            .map_err(|e| super::error::AocError::WriteLocalDataError(e.to_string()))?;
        Ok(data)
    }

    fn try_loading_cached_data(local_path: std::path::PathBuf) -> Option<String> {
        if local_path.exists() {
            println!("Reading from local file");
            let cached_data = fs::read_to_string(local_path.clone()).expect("Error reading file");
            if cached_data.contains("Puzzle inputs differ by user") {
                println!("Puzzle inputs differ by user, deleting file");
                fs::remove_file(local_path).expect("Error deleting file");
                return None;
            }
            return Some(cached_data);
        }
        None
    }

    /// Finds an environment variable matching AOC_YYYY_SESSION_ID pattern
    /// and returns (year, session_id)
    fn find_aoc_session() -> Result<String, super::error::AocError> {
        for (key, _) in std::env::vars() {
            if let Some(year) = key
                .strip_prefix("AOC_")
                .and_then(|s| s.strip_suffix("_SESSION_ID"))
            {
                if year.len() == 4 && year.chars().all(|c| c.is_ascii_digit()) {
                    return Ok(year.to_string());
                }
            }
        }
        Err(super::error::AocError::ParseYearFromEnvVarError(
            "No AOC_YYYY_SESSION_ID environment variable found (e.g. AOC_2025_SESSION_ID)"
                .to_string(),
        ))
    }

    fn get_remote_data(day: u8) -> Result<String, super::error::AocError> {
        println!("Reading from adventofcode.com");
        let year = find_aoc_session()?;
        let session_var_name = format!("AOC_{}_SESSION_ID", year);
        let session_id = std::env::var(&session_var_name).map_err(|_| {
            super::error::AocError::ParseYearFromEnvVarError(format!(
                "{} environment variable not found",
                session_var_name
            ))
        })?;
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let mut request_headers = reqwest::header::HeaderMap::new();
        let header_string = format!("session={}", session_id);
        request_headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&header_string).unwrap(),
        );

        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers(request_headers)
            .cookie_store(true)
            .build()
            .expect("couldn't build blocking client");
        let response = client.get(&url).send().expect("couldn't send request");
        response
            .text()
            .map_err(|e| super::error::AocError::GetRemoteDataError(e.to_string()))
    }

    fn get_local_path(day: u8) -> std::path::PathBuf {
        std::path::PathBuf::from(format!("cached_data/day{:02}.txt", day))
    }

    pub fn get_day_number(package_name: &str) -> u8 {
        let (_, day_number) = package_name.split_once("day").expect("'day' and number");
        day_number
            .parse::<u8>()
            .expect("Day should have a number with it")
    }
}
