use crate::core::*;

pub enum MeshRecipe<'a> {
    Basic {
        data: &'a BasicMesh,
    },
    Simple {
        data: &'a SimpleMesh,
    }
}

pub type Mesh = Box<dyn MeshProto>;

pub trait MeshProto {
    fn cook(&self) -> MeshRecipe;
}

/// MeshBasic only draw lines 
#[derive(Clone)]
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
#[derive(Clone)]
pub struct SimpleMesh {
    pub vertices: Vec<Point3<f32>>,
    // Point index + Color
    pub polygons: Vec<(usize, usize, usize, String)>, 
}

impl SimpleMesh {

    pub fn new(vertices: Vec<Point3<f32>>, polygons: Vec<(usize, usize, usize, String)>) -> Mesh {
        Box::new(Self {
            vertices,
            polygons
        })
    }
}

impl MeshProto for SimpleMesh {
    fn cook(&self) -> MeshRecipe {
        MeshRecipe::Simple { data: &self }
    }
}

