use core::f32;

use bevy::math::{NormedVectorSpace, Vec3};
use vtkio::Vtk;

use crate::parse::{parse_csv_to_vec_f32, parse_csv_to_vec_i32, vtu_vertices};

pub const VTU_DEMO_FILE: &str = "assets/vtu/box.vtu";

pub fn parse_vtu_pts(path: &str) -> Result<Vec<[f64; 3]>, String> {
    let file_path = std::path::PathBuf::from(path);
    let vtk_file = Vtk::import(file_path).map_err(|e| e.to_string())?;
    vtu_vertices(vtk_file)
}

// Generate a dummmy grid of point
pub fn dummy_pts(size: usize) -> Vec<[f32; 3]> {
    let mut points = Vec::with_capacity(size * size * size);
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                points.push([i as f32, j as f32, k as f32]);
            }
        }
    }
    return points;
}

pub fn pts_from_csv(path: &str) -> Vec<[f32; 3]> {
    let pts_vec = parse_csv_to_vec_f32(path).unwrap();
    let mut result = Vec::with_capacity(pts_vec.len());
    for pt in pts_vec.iter() {
        result.push([pt[0], pt[1], pt[2]]);
    }
    result
}

pub fn cells_from_csv(path: &str) -> Vec<[usize; 8]> {
    let cell_vec = parse_csv_to_vec_i32(path).unwrap();
    let mut result = Vec::with_capacity(cell_vec.len());
    for cell in cell_vec.iter() {
        result.push([
            cell[0], cell[1], cell[2], cell[3], cell[4], cell[5], cell[6], cell[7],
        ]);
    }
    result
}

pub fn cell_centres(pts: &[[f32; 3]], cells: &[[usize; 8]]) -> Vec<[f32; 3]> {
    let mut cell_centres = Vec::new();
    for cell in cells.iter() {
        let mut cell_centre: [f32; 3] = [0., 0., 0.];
        for &i in cell.iter() {
            cell_centre[0] += pts[i][0];
            cell_centre[1] += pts[i][1];
            cell_centre[2] += pts[i][2];
        }

        cell_centre[0] /= 8.;
        cell_centre[1] /= 8.;
        cell_centre[2] /= 8.;
        cell_centres.push(cell_centre);
    }

    cell_centres
}

const COOL_WARM_LUT: &[[f32; 3]; 3] = &[
    [57. / 255., 82. / 255., 204. / 255.],
    [220. / 255., 221. / 255., 221. / 255.],
    [204. / 255., 0. / 255., 33. / 255.],
];

// Map some scalar data to a cool-warm gradient
pub fn cool_warm_gradient(data: &[f32], min: f32, max: f32) -> Vec<[f32; 3]> {
    let mut colours = Vec::with_capacity(data.len());
    for &val in data.iter() {
        colours.push(cool_warm_col_from_val(val, min, max));
    }
    colours
}

pub fn cool_warm_col_from_val(val: f32, min: f32, max: f32) -> [f32; 3] {
    let t = f32::min(1.0, f32::max(0.0, (val - min) / (max - min)));
    if t < 0.5 {
        [
            (t / 0.5) * COOL_WARM_LUT[1][0] + (1. - t / 0.5) * COOL_WARM_LUT[0][0],
            (t / 0.5) * COOL_WARM_LUT[1][1] + (1. - t / 0.5) * COOL_WARM_LUT[0][1],
            (t / 0.5) * COOL_WARM_LUT[1][2] + (1. - t / 0.5) * COOL_WARM_LUT[0][2],
        ]
    } else {
        [
            ((t - 0.5) / 0.5) * COOL_WARM_LUT[2][0] + (1. - (t - 0.5) / 0.5) * COOL_WARM_LUT[1][0],
            ((t - 0.5) / 0.5) * COOL_WARM_LUT[2][1] + (1. - (t - 0.5) / 0.5) * COOL_WARM_LUT[1][1],
            ((t - 0.5) / 0.5) * COOL_WARM_LUT[2][2] + (1. - (t - 0.5) / 0.5) * COOL_WARM_LUT[1][2],
        ]
    }
}

pub fn min_max_norm(vecs: &[[f32; 3]]) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = -f32::MAX;
    for vec in vecs {
        let mag = Vec3::new(vec[0], vec[1], vec[2]).norm();
        min = f32::min(mag, min);
        max = f32::max(mag, max)
    }
    (min, max)
}
