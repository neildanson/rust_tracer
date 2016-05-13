use raytracer::ray::Ray;
use raytracer::sphere::Sphere;

pub struct Intersection<'a> {
    pub ray: Ray,
    pub distance: f64,
    pub object: &'a Sphere<'a>,
}