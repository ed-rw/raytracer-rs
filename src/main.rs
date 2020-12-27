mod vec3;
mod ray;
mod hitable;
mod sphere;

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

    print!("P3\n{} {} \n255\n", nx, ny);

    let lower_left_corner = vec3::Vec3{ e: [-2.0, -1.0, -1.0]};
    let horizontal = vec3::Vec3{ e: [4.0, 0.0, 0.0]};
    let vertical = vec3::Vec3{ e: [0.0, 2.0, 0.0]};
    let origin = vec3::Vec3{ e: [0.0, 0.0, 0.0]};
    let objs = vec![
        Box::new(sphere::Sphere { center: vec3::Vec3{ e: [0.0, 0.0, -1.0] },
                         radius: 0.5}) as Box<dyn hitable::Hitable>,
        Box::new(sphere::Sphere { center: vec3::Vec3{ e: [0.0, -100.5, -1.0] },
                         radius: 100.0}) as Box<dyn hitable::Hitable>,
    ];
    let world = hitable::HitableList{ list: objs };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = ray::Ray{ a: origin, b: lower_left_corner + u*horizontal + v*vertical};

            let icol = color(r, &world) * 255.99;
            println!("{} {} {}", icol.r() as i32, icol.g() as i32, icol.b() as i32);
        }
    }
}
