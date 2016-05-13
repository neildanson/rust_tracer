use raytracer::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
}