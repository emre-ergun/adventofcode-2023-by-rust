use std::fs;
fn main() {
    let file_path = "./puzzle-input.txt";
    match calculate_calibration(file_path) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn calculate_calibration(file_path: &str) -> Result<i32, String> {
    Ok(fs::read_to_string(&file_path)
        .map_err(|err| err.to_string())?
        .lines()
        .map(|s| {
            let mut calibration_val = 0;
            for c in s.chars() {
                if c.is_numeric() {
                    let value = c.to_string().parse::<i32>().unwrap();
                    calibration_val = value * 10;
                    break;
                }
            }

            for c in s.chars().rev() {
                if c.is_numeric() {
                    calibration_val += c.to_string().parse::<i32>().unwrap();
                    break;
                }
            }

            calibration_val
        })
        .sum::<i32>())
}
