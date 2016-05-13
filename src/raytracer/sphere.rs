use raytracer::vec3::Vec3;
use raytracer::intersection::Intersection;
use raytracer::texture::Texture;

pub struct Sphere<'a> {
    pub position: Vec3,
    pub radius: f64,
    pub texture: &'a Texture,
}

impl<'a> Sphere<'a> {
    pub fn normal(&self, intersection: &Intersection) -> Vec3 {
        (intersection.ray.position + (intersection.ray.direction * intersection.distance) -
         self.position)
            .normalize()
    }
}