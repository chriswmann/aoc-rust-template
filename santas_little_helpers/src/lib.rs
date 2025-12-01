use std::fmt;
pub mod error {
    use std::fmt;

    #[derive(Clone, Debug, PartialEq)]
    pub enum AocError {
        FromStrError(String),
        ParseDirectionError(char),
        ParsePointTypeError(char),
        ParseYearFromEnvVarError(String),
        GetRemoteDataError(String),
        WriteLocalDataError(String),
    }

    impl fmt::Display for AocError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::FromStrError(e) => writeln!(f, "Couldn't get rule from str, {}", e),
                Self::ParseDirectionError(e) => {
                    writeln!(f, "Couldn't parse direction from char, {}", e)
                }
                Self::ParsePointTypeError(e) => {
                    writeln!(f, "Couldn't parse point type from char, {}", e)
                }
                Self::ParseYearFromEnvVarError(e) => {
                    writeln!(f, "Couldn't parse year from env var, {}", e)
                }
                Self::GetRemoteDataError(e) => {
                    writeln!(f, "Couldn't get remote data, {}", e)
                }
                Self::WriteLocalDataError(e) => {
                    writeln!(f, "Couldn't write local data, {}", e)
                }
            }
        }
    }
    impl std::error::Error for AocError {}
}

#[derive(Clone, Debug)]
pub enum Part {
    One,
    Two,
}

impl std::convert::TryFrom<u8> for Part {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Part::One as u8 => Ok(Part::One),
            x if x == Part::Two as u8 => Ok(Part::Two),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub mod data {
    use std::fs;
    use std::path;

    pub fn load_data(day: u8) -> Result<String, super::error::AocError> {
        let local_path = get_local_path(day);
        if let Some(data) = try_loading_cached_data(&local_path) {
            return Ok(data);
        }
        let data = get_remote_data(day)?;
        let local_path = get_local_path(day);
        fs::write(local_path, &data)
            .map_err(|e| super::error::AocError::WriteLocalDataError(e.to_string()))?;
        Ok(data)
    }

    fn try_loading_cached_data(local_path: &str) -> Option<String> {
        if path::Path::new(local_path).exists() {
            println!("Reading from local file");
            let cached_data = fs::read_to_string(local_path).expect("Error reading file");
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
        let session_id = std::env::var(&session_var_name).expect(&format!(
            "{} environment variable not found",
            session_var_name
        ));
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

    fn get_local_path(day: u8) -> String {
        format!("cached_data/day{:02}.txt", day)
    }

    pub fn get_day_number(package_name: &str) -> u8 {
        let (_, day_number) = package_name.split_once("day").expect("'day' and number");
        day_number
            .parse::<u8>()
            .expect("Day should have a number with it")
    }
}
