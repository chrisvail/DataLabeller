use std::{fs::File, io::{BufRead, BufReader}};
use nalgebra::{Vector2, Vector3};

use super::{Mesh};

#[allow(unused)]
pub fn parse_obj(file: File) -> Result<Mesh, String> {

    let file = BufReader::new(file);

    let mut mesh = Mesh::new();

    for line in file.lines() {
        let line = line.expect("Why would this fail?");

        match line {
            _ if line.starts_with("v ") => {
                let (vert_data, colour_data) = parse_vertex(line)?;
                mesh.add_vertex(vert_data);
                if let Some(colour_data) = colour_data {
                    mesh.add_colour(colour_data);
                }
            },
            _ if line.starts_with("vn ") => {
                let normal_data = parse_vertex_normal(line)?;
                mesh.add_normal(normal_data);
            },
            _ if line.starts_with("vt ") => {
                let texture_coord = parse_texture_coordinate(line)?;
                mesh.add_uv(texture_coord);
            },
            _ if line.starts_with("f ") => {
                let face = parse_face(line)?;
                mesh.add_face(face)?;
            },
            _ if line.starts_with('#') => {}, // Ignore comments
            _ => { } // Ignore other lines
            // _ => {panic!("This line is not supported: {line}")}
        }
    }

    Ok(mesh)

}

fn parse_vertex(line: String) -> Result<(Vector3<f32>, Option<Vector3<u8>>), String> {
    let split: Result<Vec<f32>, _> = line.split(' ').skip(1).map(|val| val.parse::<f32>()).collect();
    let split = split.map_err(|_| "Failed to split line".to_string())?;

    match split {
        _ if split.len() == 3 => {
            Ok((Vector3::new(split[0], split[1], split[2]), None))
        },
        _ if split.len() == 6 => {
            let colours = [split[3], split[4], split[5]].map(|val| (val*255.0).round());
            if colours.iter().all(|val| *val >= 0.0 && *val < 256.0) {
                Ok((
                    Vector3::new(split[0], split[1], split[2]), 
                    Some(Vector3::new(colours[0] as u8, colours[1] as u8, colours[2] as u8))
                ))
            } else {
                Err("Invalid colours found".to_string())
            }
            
        }
        _ => {Err("Unable to parse line: Invalid number of entries".to_string())}
    }
}

fn parse_vertex_normal(line: String) -> Result<Vector3<f32>, String> {
    let split: Result<Vec<f32>, _> = line.split(' ').skip(1).map(|val| val.parse::<f32>()).collect();
    let split =  split.map_err(|_| "Failed to split line".to_string())?;

    if split.len() == 3 {
        Ok(Vector3::new(split[0], split[1], split[2]))
    } else {
        Err("Invalid number of normals".to_string())
    }
}

fn parse_texture_coordinate(line: String) -> Result<Vector2<f32>, String> {
    let split: Result<Vec<f32>, _> = line.split(' ').skip(1).map(|val| val.parse::<f32>()).collect();
    let split =  split.map_err(|_| "Failed to split line".to_string())?;

    match split {
        _ if split.len() == 1 => {
            Err("Only one texture coord found - expected 2".to_string())
        },
        _ if split.len() == 2 => {
            Ok(Vector2::new(split[0], split[1]))
        },
        // _ if split.len() == 3 => {
        //     Ok(TextureCoord::Three(Vector3::new(split[0], split[1], split[2])))
        // },
        _ => Err("Invalid number of texture coordinates".to_string())
        
    }
}

fn parse_face(line: String) -> Result<Vector3<usize>, String> {
    if line.contains('/') {
        todo!("This is not implemented yet");
    } else {
        let split: Result<Vec<usize>, _> = line.split(' ').skip(1).map(|val| val.parse::<usize>()).collect();
        let split = split.map_err(|_| "Failed to split line".to_string())?;

        if split.len() != 3 {
            Err("Only triangle faces are currently accepted".to_string())
        } else {
            Ok(Vector3::new(split[0] - 1, split[1] - 1, split[2] - 1))
        }

    }
}