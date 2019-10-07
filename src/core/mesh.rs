use crate::core::*;

pub enum MeshRecipe<'a> {
    Basic {
        data: &'a BasicMesh,
    },
}

pub type Mesh = Box<dyn MeshProto>;

pub trait MeshProto {
    fn cook(&self) -> MeshRecipe;
}

/// MeshBasic only draw lines 
pub struct BasicMesh {
    pub vertices: Vec<Point3<f32>>,
    pub breaks: Vec<usize>,
}

impl BasicMesh {
    pub fn new(vertices: Vec<Point3<f32>>, breaks: Vec<usize>) -> Mesh {
        Box::new(Self {
            vertices,
            breaks,
        })
    }
}

impl MeshProto for BasicMesh {
    fn cook(&self) -> MeshRecipe {
        MeshRecipe::Basic { data: &self }
    }
}

/// MeshPolygon draw triangles 
pub struct SimpleMesh {
    pub vertices: Vec<Point3<f32>>,
    pub polygons: Vec<(usize, usize, usize, usize)>,
}


