use rand::Rng;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

fn color(r: ray::Ray, world: &dyn hitable::Hitable) -> vec3::Vec3 {
    let rec = world.hit(r, 0.0, f32::INFINITY);
    if !rec.is_none() {
        return 0.5 * vec3::Vec3{ e: [rec.unwrap().normal.x()+1.0,
                                     rec.unwrap().normal.y()+1.0,
                                     rec.unwrap().normal.z()+1.0] };
    } else {
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * vec3::Vec3{ e: [1.0, 1.0, 1.0]} + t * vec3::Vec3{ e: [0.0, 0.0, 0.0]}
    }
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 10;

    print!("P3\n{} {} \n255\n", nx, ny);

    let objs = vec![
        Box::new(sphere::Sphere { center: vec3::Vec3{ e: [0.0, 0.0, -1.0] },
                                  radius: 0.5}) as Box<dyn hitable::Hitable>,
        Box::new(sphere::Sphere { center: vec3::Vec3{ e: [0.0, -100.5, -1.0] },
                                  radius: 100.0}) as Box<dyn hitable::Hitable>,
    ];
    let world = hitable::HitableList{ list: objs };
    let cam = camera::Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = vec3::Vec3{ e: [0.0, 0.0, 0.0]};
            for s in 0..ns {
                let u = (i as f32 + rand::thread_rng().gen_range(0.0..1.0))
                        / nx as f32;
                let v = (j as f32 + rand::thread_rng().gen_range(0.0..1.0))
                        / ny as f32;

                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(r, &world);
            }
            col /= ns as f32;
            let icol = col * 255.99;
            println!("{} {} {}", icol.r() as i32, icol.g() as i32, icol.b() as i32);
        }
    }
}
