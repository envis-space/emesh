use crate::polygon::{IndexedPolygon, Polygon};
use nalgebra::Point3;
use rayon::iter::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    vertices: Vec<Point3<f64>>,
    polygons: Vec<IndexedPolygon>,
}

impl Mesh {
    pub fn new(vertices: Vec<Point3<f64>>, polygons: Vec<IndexedPolygon>) -> Self {
        Self { vertices, polygons }
    }

    pub fn add_vertex(&mut self, vertex: Point3<f64>) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_indexed_polygon(&mut self, polygon: IndexedPolygon) {
        self.polygons.push(polygon);
    }

    pub fn add_polygon(&mut self, polygon: &Polygon, tolerance: Option<f64>) {
        if tolerance.is_some() {
            todo!("not supported yet");
        }
        let start_index = self.vertices.len();
        let indext_range: Vec<_> =
            (start_index..(start_index + polygon.vertices().len())).collect();
        let mut vertices = polygon.vertices().clone();
        self.vertices.append(&mut vertices);

        let indexed_polygon = IndexedPolygon::new(indext_range).unwrap();
        self.polygons.push(indexed_polygon);
    }

    pub fn get_polygon(&self, index: usize) -> Option<Polygon> {
        let indexed_polygon = self.polygons.get(index)?;

        let polygon = resolve_indexed_polygon(indexed_polygon, &self.vertices);
        Some(polygon)
    }

    pub fn get_polygons(&self) -> Vec<Polygon> {
        self.polygons
            .par_iter()
            .map(|p| resolve_indexed_polygon(p, &self.vertices))
            .collect()
    }
}

fn resolve_indexed_polygon(
    indexed_polygon: &IndexedPolygon,
    vertices: &Vec<Point3<f64>>,
) -> Polygon {
    let vertices: Vec<Point3<f64>> = indexed_polygon
        .vertices()
        .iter()
        .map(|i| *vertices.get(*i).expect("indices must be sane"))
        .collect();

    Polygon::new(vertices).expect("must be sane")
}
