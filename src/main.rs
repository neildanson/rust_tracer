
mod raytracer;
extern crate image;
extern crate rand;

use raytracer::texture::*;
use raytracer::camera::*;
use raytracer::vec3::*;
use raytracer::light::*;
use raytracer::sphere::*;
use raytracer::raytracer::*;
use std::fs::File;
use std::path::Path;
use std::time::*;
use rand::Rng;

fn main() {
    let lava = load_texture("lava.jpg");
    let grass = load_texture("grass.png");

    let width = 1280;
    let height = 720;
    let half_width = width as f64 / 2.0;
    let half_height = height as f64 / 2.0;
    let inverse_height = 1.0 / height as f64;
    let blur = 100;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let position = Vec3 {
        x: 3.0,
        y: 3.0,
        z: -3.0,
    };
    let look_at = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    let camera = create_camera(position, look_at, inverse_height);
    let sphere1 = Sphere {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
        texture: &lava,
    };
    let sphere2 = Sphere {
        position: Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
        texture: &grass,
    };
    let sphere3 = Sphere {
        position: Vec3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
        texture: &lava,
    };
    let sphere4 = Sphere {
        position: Vec3 {
            x: -2.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
        texture: &grass,
    };

    let sphere5 = Sphere {
        position: Vec3 {
            x: -4.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
        texture: &lava,
    };
    let ground = Sphere {
        position: Vec3 {
            x: 0.0,
            y: -100.0,
            z: 0.0,
        },
        radius: 99.0,
        texture: &lava,
    };

    let objects = vec![&sphere1, &sphere2, &sphere3, &sphere4, &sphere5, &ground];

    let lights = vec![Light {
                          position: Vec3 {
                              x: -3.0,
                              y: 3.0,
                              z: -1.0,
                          },
                          color: Vec3 {
                              x: 0.5,
                              y: 0.0,
                              z: 0.0,
                          },
                      },
                      Light {
                          position: Vec3 {
                              x: 3.0,
                              y: 3.0,
                              z: 1.0,
                          },
                          color: Vec3 {
                              x: 0.0,
                              y: 0.0,
                              z: 0.5,
                          },
                      },
                      Light {
                          position: Vec3 {
                              x: 0.0,
                              y: 1.0,
                              z: -10.0,
                          },
                          color: Vec3 {
                              x: 0.5,
                              y: 0.5,
                              z: 0.5,
                          },
                      }];

    let mut rng = rand::thread_rng();
    let start = SystemTime::now();
    for x in 0..width {
        for y in 0..height {
            let mut base_color = ZERO;
            for _ in 1..blur - 1 {
                let position = Vec3 {
                    x: position.x + ((rng.gen::<f64>() - 0.5) * 0.2),
                    y: position.y + ((rng.gen::<f64>() - 0.5) * 0.2),
                    z: position.z,
                };
                let camera = create_camera(position, look_at, inverse_height);
                let ray = get_ray(position,
                                  x as f64,
                                  y as f64,
                                  half_width,
                                  half_height,
                                  &camera);
                base_color = base_color + trace(&ray, &objects, &lights, 0);
            }
            let ray = get_ray(position,
                              x as f64,
                              y as f64,
                              half_width,
                              half_height,
                              &camera);

            let color = trace(&ray, &objects, &lights, 0);
            let color = (color + base_color) / blur as f64;

            let px = image::Rgba(color.to_color());
            imgbuf.put_pixel(x as u32, y as u32, px);
        }
    }

    let end = SystemTime::now();
    let render_time = end.duration_since(start).unwrap();
    println!("{:?}", render_time);
    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG);
}
