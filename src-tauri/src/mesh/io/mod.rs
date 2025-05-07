use std::{ffi::OsStr, fs::File, path::Path, time::Instant};

use nalgebra::{Vector2, Vector3};
use obj::parse_obj;

pub mod obj;
pub mod ply;
pub mod stl;

#[derive(Debug, serde::Serialize)]
pub struct ThreeMesh {
    faces: Vec<usize>,
    vertices: Vec<f32>,
    vertices_size: usize,
    colours: Option<Vec<f32>>,
    colour_size: Option<usize>,
    normals: Option<Vec<f32>>,
    normal_size: Option<usize>,
    uvs: Option<Vec<f32>>,
    uv_size: Option<usize>,
}

impl ThreeMesh {
    pub fn new(
        faces: Vec<usize>,
        vertices: Vec<f32>,
        vert_count: usize,
        colours: Option<Vec<f32>>,
        normals: Option<Vec<f32>>,
        uvs: Option<Vec<f32>>,
    ) -> Self {
        let colour_size = colours.as_ref().map(|colour_data| colour_data.len() / vert_count);

        let normal_size = normals.as_ref().map(|normal_data| normal_data.len() / vert_count);

        let uv_size = uvs.as_ref().map(|uv_data| uv_data.len() / vert_count);

        let vertices_size = vertices.len() / vert_count;

        Self {
            faces,
            vertices,
            vertices_size,
            colours,
            colour_size,
            normals,
            normal_size,
            uvs,
            uv_size,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vector3<f32>>,
    colours: Option<Vec<Vector3<u8>>>,
    normals: Option<Vec<Vector3<f32>>>,
    uvs: Option<Vec<Vector2<f32>>>,
    faces: Vec<Vector3<usize>>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            colours: None,
            normals: None,
            uvs: None,
            faces: vec![],
        }
    }

    pub fn add_vertex(&mut self, vertex: Vector3<f32>) {
        self.vertices.push(vertex);
    }

    pub fn add_face(&mut self, face: Vector3<usize>) -> Result<(), String> {
        if face.max() <= self.vertices.len() {
            self.faces.push(face);
            Ok(())
        } else {
            Err("Failed to add face - References uncreated vertices".to_string())
        }
    }

    pub fn add_colour(&mut self, colour: Vector3<u8>) {
        match self.colours.as_mut() {
            Some(colours) => colours.push(colour),
            None => self.colours = Some(vec![colour]),
        }
    }

    pub fn add_normal(&mut self, normal: Vector3<f32>) {
        match self.normals.as_mut() {
            Some(normals) => normals.push(normal),
            None => self.normals = Some(vec![normal]),
        }
    }

    pub fn add_uv(&mut self, uv: Vector2<f32>) {
        match self.uvs.as_mut() {
            Some(uvs) => uvs.push(uv),
            None => self.uvs = Some(vec![uv]),
        }
    }
}

impl From<Mesh> for ThreeMesh {
    fn from(value: Mesh) -> Self {
        let vert_count = value.vertices.len();
        ThreeMesh::new(
            value
                .faces
                .into_iter()
                .flat_map(|vec| vec![vec.x, vec.y, vec.z])
                .collect(),
            value
                .vertices
                .into_iter()
                .flat_map(|vec| vec![vec.x, vec.y, vec.z])
                .collect(),
            vert_count,
            value.colours.map(|val| {
                val.iter()
                    .flat_map(|vec| {
                        vec![
                            vec.x as f32 / 255.0,
                            vec.y as f32 / 255.0,
                            vec.z as f32 / 255.0,
                        ]
                    })
                    .collect()
            }),
            value.normals.map(|val| {
                val.iter()
                    .flat_map(|vec| vec![vec.x, vec.y, vec.z])
                    .collect()
            }),
            value
                .uvs
                .map(|val| val.iter().flat_map(|vec| vec![vec.x, vec.y]).collect()),
        )
    }
}

pub fn parse_mesh_file(path: String) -> Result<Mesh, String> {
    let path = Path::new(&path);
    let file = File::open(path).map_err(|_| "Failed to open file".to_string())?;

    let t0 = Instant::now();
    let mesh = match path.extension().and_then(|ext| ext.to_str()) {
        Some("obj") => parse_obj(file),
        _ => Err("Invalid file type".to_string()),
    }?;

    println!("{:?}", Instant::now() - t0);

    Ok(mesh)
}
