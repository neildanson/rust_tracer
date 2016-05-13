use raytracer::vec3::*;
use raytracer::ray::Ray;
use raytracer::light::Light;
use raytracer::sphere::Sphere;
use raytracer::intersection::Intersection;
use raytracer::texture::Texture;

fn foo(normal: Vec3, texture: &Texture) -> Vec3 {
    let x = (normal.x / 2.0) + 0.5;
    let y = (normal.y / 2.0) + 0.5;
    texture.lookup(x, y)
}

fn object_intersects<'a>(ray: &Ray, object: &'a Sphere) -> Option<Intersection<'a>> {
    let diff = object.position - ray.position;
    let v = diff.dot(&ray.direction);
    if v < 0.0 {
        None
    } else {
        let distance_squared = object.radius.powi(2) - (diff.dot(&diff) - v.powi(2));
        if distance_squared < 0.0 {
            None
        } else {
            let distance = v - distance_squared.sqrt();
            Some(Intersection {
                ray: *ray,
                distance: distance,
                object: object,
            })
            // Normal = Vector3.Normalize(ray.Position + (ray.Direction * distance) - position); Object = s })
        }
    }

}

fn any_intersection(ray: &Ray, objects: &Vec<&Sphere>) -> bool {
    objects.iter().any(|object| object_intersects(ray, &object).is_some())
}

fn nearest_intersection<'a>(ray: &Ray, objects: &'a Vec<&Sphere>) -> Option<Intersection<'a>> {
    objects.iter().fold(None, |intersection, object| {
        let i = object_intersects(ray, object);
        match (intersection, i) {
            (Some(intersection), None) => Some(intersection),
            (None, Some(i)) => Some(i),
            (Some(a), Some(b)) => {
                if a.distance < b.distance {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            (_, _) => None,

        }
    })
}

fn apply_light(position: Vec3,
               normal: Vec3,
               objects: &Vec<&Sphere>,
               light: &Light,
               ray_direction: Vec3,
               base_color: Vec3)
               -> Vec3 {

    let light_dir = (light.position - position).normalize();
    let ray = Ray {
        position: position,
        direction: light_dir,
    };
    let is_in_shadow = any_intersection(&ray, objects);
    if is_in_shadow {
        ZERO
    } else {
        let illum = light_dir.dot(&normal);
        let lcolor = if illum > 0.0 {
            light.color * illum
        } else {
            ZERO
        };
        let diffuse_color = lcolor * base_color;
        let dot = normal.dot(&ray_direction);
        let ray_direction = (ray_direction - (normal * (2.0 * dot))).normalize();
        let specular = light_dir.dot(&ray_direction);
        let specular_result = if specular > 0.0 {
            light.color * (specular.powi(50))
        } else {
            ZERO
        };
        diffuse_color + specular_result
    }
}


fn apply_lighting(position: Vec3,
                  normal: Vec3,
                  objects: &Vec<&Sphere>,
                  lights: &Vec<Light>,
                  ray_direction: Vec3,
                  base_color: Vec3)
                  -> Vec3 {
    lights.iter().fold(ZERO, |color, light| {
        color + apply_light(position, normal, objects, &light, ray_direction, base_color)
    })
}

pub fn trace(ray: &Ray, objects: &Vec<&Sphere>, lights: &Vec<Light>, depth: i32) -> Vec3 {
    let intersection = nearest_intersection(ray, objects);
    match intersection {
        Some(intersection) => {
            let hit_point = intersection.ray.position +
                            (intersection.ray.direction * intersection.distance);

            let normal = intersection.object.normal(&intersection);

            let color = foo(normal, intersection.object.texture);
            // let color = Foo intersection.object intersection
            let color = apply_lighting(hit_point,
                                       normal, // intersection.normal,
                                       objects,
                                       lights,
                                       intersection.ray.direction,
                                       color);
            if depth < 3 {
                let ray = Ray {
                    position: hit_point,
                    direction: normal,
                };
                let newcolor = trace(&ray, objects, lights, depth + 1);
                color + newcolor
            } else {
                color
            }
        }
        None => ZERO,
    }
}
