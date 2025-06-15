use core::f32;

use bevy::math::{NormedVectorSpace, Vec3};
use crate::parse::parse_csv_to_vec_f32;
use crate::parse::parse_csv_to_vec_i32;

pub const VTU_DEMO_FILE: &str = "assets/vtu/box.vtu";

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

/// Find the index of the closest cell centre to a given point
/// Take in a vector of cell centres and a sample point, then return the index of the closest cell centre.
pub fn closest_cell_index(point: [f32; 3], cell_centres: &[[f32; 3]]) -> usize {
    let mut closest_index = -1;
    let mut closest_distance = f32::MAX;

    for (i, &cell_centre) in cell_centres.iter().enumerate() {
        let distance = Vec3::new(
            point[0] - cell_centre[0],
            point[1] - cell_centre[1],
            point[2] - cell_centre[2],
        )
        .norm();
        if distance < closest_distance {
            closest_distance = distance;
            closest_index = i as isize;
        }
    }

    if closest_index < 0 {
        panic!("No cell centres provided");
    }

    closest_index as usize
}

// Create a list of streamline points
pub fn streamline_points(
    start: [f32; 3],
    num_streamline_points: usize,
    velocity: &[[f32; 3]],
    cell_centres: &[[f32; 3]],
    dx: f32,
) -> Vec<[f32; 3]> {
    let mut points = Vec::with_capacity(num_streamline_points);
    let mut current_point = start;

    for _ in 0..num_streamline_points {
        points.push(current_point);
        let index = closest_cell_index(current_point, cell_centres);
        let velocity_vector = velocity[index];

        // Normalize the velocity vector to get a direction
        let norm_velocity = Vec3::new(velocity_vector[0], velocity_vector[1], velocity_vector[2])
            .normalize_or_zero();

        // Update the current point based on the normalized velocity
        current_point[0] += norm_velocity.x * dx; // Adjust step size as needed
        current_point[1] += norm_velocity.y * dx;
        current_point[2] += norm_velocity.z * dx;
    }

    points
}

pub fn fan_streamline_points(cell_centres: &[[f32; 3]], velocity: &[[f32; 3]]) -> Vec<[f32; 3]> {
    // Get streamline points. Start with an empty list and call the streamline_points function repeatedly

    // Empty vector to hold all streamline points
    let mut all_streamline_points = Vec::new();

    let fan_center = [0.0, 0.075, 0.125];
    let fan_radius = 0.05;

    // Generate randomised starting positions somewhere in the fan area
    for i in 0..10 {
        // random angle in radians
        let angle = i as f32 * (2.0 * std::f32::consts::PI / 10.0) + rand::random::<f32>() * 0.1;
        let r = fan_radius * rand::random::<f32>(); // Random radius between 0 and fan_radius

        // random radius in the range [0, fan_radius]
        let x = fan_center[0];
        let y = fan_center[1] + r * angle.cos();
        let z = fan_center[2] + r * angle.sin();

        // Generate streamline points from this position
        let streamline_points = streamline_points([x, y, z], 100, &velocity, &cell_centres, 0.01);
        all_streamline_points.extend(streamline_points);

        all_streamline_points.push([x, y, z]);
    }
    all_streamline_points
}
