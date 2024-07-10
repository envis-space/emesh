use crate::Error;
use nalgebra::Point3;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexedPolygon {
    vertices: Vec<usize>,
}

impl IndexedPolygon {
    pub fn new(vertices: Vec<usize>) -> Result<Self, Error> {
        Ok(Self { vertices })
    }

    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    vertices: Vec<Point3<f64>>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point3<f64>>) -> Result<Self, Error> {
        Ok(Self { vertices })
    }

    pub fn vertices(&self) -> &Vec<Point3<f64>> {
        &self.vertices
    }
}
