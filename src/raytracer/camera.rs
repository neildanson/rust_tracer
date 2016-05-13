use raytracer::vec3::Vec3;
use raytracer::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

pub fn create_camera(position: Vec3, look_at: Vec3, inverse_height: f64) -> Camera {
    let forward = (look_at - position).normalize();
    let down = Vec3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    let right = forward.cross(&down).normalize() * 1.5 * inverse_height;
    let up = forward.cross(&right).normalize() * 1.5 * inverse_height;

    Camera {
        position: position,
        forward: forward,
        right: right,
        up: up,
    }
}

fn recenter_x(x: f64, half_width: f64) -> f64 {
    x - half_width
}

fn recenter_y(y: f64, half_height: f64) -> f64 {
    -(y - half_height)
}

pub fn get_ray(position: Vec3,
               x: f64,
               y: f64,
               half_width: f64,
               half_height: f64,
               camera: &Camera)
               -> Ray {
    let right = camera.right * recenter_x(x, half_width);
    let up = camera.up * recenter_y(y, half_height);
    Ray {
        position: position,
        direction: (right + up + camera.forward).normalize(),
    }
}