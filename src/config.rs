use csv::ReaderBuilder;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub visualization_program: String,
    pub repository_path: String,
    pub image_name: String,
    pub file_path: String,
}

impl Config {
    pub fn new_from_file(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;
        if let Some(result) = rdr.records().next() {
            let record = result?;
            return Ok(Config {
                visualization_program: record[0].trim().to_string(),
                repository_path: record[1].trim().to_string(),
                image_name: record[2].trim().to_string(),
                file_path: record[3].trim().to_string(),
            });
        }
        Err("Не удалось прочитать конфигурацию".into())
    }
}
