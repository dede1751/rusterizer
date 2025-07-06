use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::primitives::{FaceData3D, Float2, Float3, Tri, VectorOps};

#[derive(Default, Debug, Clone)]
pub struct Mesh {
    pub data: Vec<FaceData3D>,
}

impl Mesh {
    pub fn from_obj_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut mesh = Mesh::default();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some((prefix, rest)) = line.split_once(char::is_whitespace) {
                match prefix {
                    "v" => {
                        let nums: Vec<f32> = rest.split_whitespace().flat_map(str::parse).collect();
                        vertices.push(Float3::new(nums[0], nums[1], nums[2]));
                    }
                    "vn" => {
                        let nums: Vec<f32> = rest.split_whitespace().flat_map(str::parse).collect();
                        normals.push(Float3::new(nums[0], nums[1], nums[2]).normalized());
                    }
                    "vt" => {
                        let nums: Vec<f32> = rest.split_whitespace().flat_map(str::parse).collect();
                        uvs.push(Float2::new(nums[0], nums[1]));
                    }
                    "f" => {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        let mut vertex_vals = Vec::new();
                        for i in parts.iter() {
                            let indices: Vec<Option<usize>> = i
                                .split('/')
                                .map(|s| s.parse::<usize>().ok().map(|n| n - 1))
                                .collect();
                            vertex_vals.push((
                                indices[0].map_or(Float3::ZERO, |i| vertices[i]),
                                indices[2].map_or(Float3::ZERO, |i| normals[i]),
                                indices[1].map_or(Float2::ZERO, |i| uvs[i]),
                            ));
                        }

                        // Handle non-triangular faces via triangle fan
                        for i in 2..vertex_vals.len() {
                            let v1 = vertex_vals[0];
                            let v2 = vertex_vals[i - 1];
                            let v3 = vertex_vals[i];

                            mesh.data.push(FaceData3D {
                                vertices: Tri::new(v1.0, v2.0, v3.0),
                                normals: Tri::new(v1.1, v2.1, v3.1),
                                uvs: Tri::new(v1.2, v2.2, v3.2),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(mesh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_from_obj() {
        let mesh = Mesh::from_obj_file("../resources/models/cube.obj").unwrap();
        assert_eq!(mesh.data.len(), 12);
    }
}
