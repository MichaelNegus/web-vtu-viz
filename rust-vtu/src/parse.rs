// This is some terrible code to extract vertices from a vtu file, and makes a lot of assumptions on the file format.


/* e.g.
fn main() {
    let file_path = std::path::PathBuf::from("box.vtu");
    let vtk_file = Vtk::import(file_path).expect("Failed to load file");

    let points = vtu_vertices(vtk_file);
    println!("{:?}", points);
}
*/

// pub fn parse_to_vtu(data: Vec<u8>) -> Vtk {}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Parses a CSV-like file into Vec<Vec<f32>>
pub fn parse_csv_to_vec_f32<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<f32>>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?; // Read the line
        let values = line
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        result.push(values);
    }

    Ok(result)
}

/// Parses a CSV-like file into Vec<Vec<f32>>
pub fn parse_csv_to_vec_i32<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<usize>>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?; // Read the line
        let values = line
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        result.push(values);
    }

    Ok(result)
}
