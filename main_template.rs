use santas_little_helpers::data::{get_day_number, load_data};
use santas_little_helpers::error::AocError;
fn main() -> Result<(), AocError> {
    let package_name = env!("CARGO_PKG_NAME");
    let day_number = get_day_number(package_name);
    let data = load_data(day_number)?;
    run(&data, day_number)?;
    Ok(())
}

fn run(data: &str, day: u32) -> Result<(), AocError> {
    let part01_answer = part01(data)?;
    println!("Day {}, Part 1 answer is: {}", day, part01_answer);
    let part02_answer = part02(data)?;
    println!("Day {}, Part 2 answer is: {}", day, part02_answer);
    Ok(())
}

pub fn part01(data: &str) -> Result<u32, AocError> {
    Ok(42)
}

pub fn part02(data: &str) -> Result<u32, AocError> {
    Ok(42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {}

    #[test]
    fn test_part02() {}
}
