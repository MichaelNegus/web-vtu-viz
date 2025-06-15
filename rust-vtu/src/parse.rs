// This is some terrible code to extract vertices from a vtu file, and makes a lot of assumptions on the file format.

use vtkio::{
    model::{DataSet, Piece, Vtk},
    IOBuffer,
};

/* e.g.
fn main() {
    let file_path = std::path::PathBuf::from("box.vtu");
    let vtk_file = Vtk::import(file_path).expect("Failed to load file");

    let points = vtu_vertices(vtk_file);
    println!("{:?}", points);
}
*/

// pub fn parse_to_vtu(data: Vec<u8>) -> Vtk {}

pub fn vtu_vertices(vtk: Vtk) -> Result<Vec<[f64; 3]>, String> {
    let data = match vtk.data {
        DataSet::UnstructuredGrid { pieces, .. } => pieces.into_iter().map(|p| match p {
            Piece::Inline(d) => Ok(d),
            _ => Err("Only inline data supported".to_string()),
        }),
        _ => return Err("We only support UnstructuredGrid".to_string()),
    };

    // TODO: must be a better way to do this with a flat_map or similar?
    let x: Result<Vec<Vec<_>>, _> = data.map(|p| get_points(p?.points)).collect();
    Ok(x?.concat())
}

// takes a `points` field of an UnstructuredGridPiece
fn get_points(points: IOBuffer) -> Result<Vec<[f64; 3]>, String> {
    // The IOBuffer is a flat list of x,y,z coordinates
    let flat_points = points.cast_into().expect("Failed to convert points");
    let mut flat_points = flat_points.iter();
    let mut point_triples = vec![];
    loop {
        let x = flat_points.next();
        if x.is_none() {
            return Ok(point_triples);
        }
        let y = flat_points.next();
        let z = flat_points.next();
        if y.is_none() || z.is_none() {
            return Err("points buffer not multiple of three in length".to_string());
        }
        point_triples.push([*x.unwrap(), *y.unwrap(), *z.unwrap()]);
    }
}

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
