
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p: vec3::Vec3;
    loop {
        p = 2.0 * vec3::Vec3{ e: [rand::random::<f32>(), rand::random::<f32>(),
                                      rand::random::<f32>()]}
                - vec3::Vec3{ e: [1.0, 1.0, 1.0] };
        if !(p.squared_length() >= 1.0) { break; }
    }
    p
}

fn color(r: ray::Ray, world: &dyn hitable::Hitable) -> vec3::Vec3 {
    let wrapped_rec = world.hit(r, 0.001, f32::INFINITY);
    if !wrapped_rec.is_none() {
        let rec = wrapped_rec.unwrap();
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(ray::Ray{ a: rec.p, b: target-rec.p }, world);
    } else {
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * vec3::Vec3{ e: [1.0, 1.0, 1.0]} + t * vec3::Vec3{ e: [0.2, 0.5, 0.7]}
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
                let u = (i as f32 + rand::random::<f32>())
                        / nx as f32;
                let v = (j as f32 + rand::random::<f32>())
                        / ny as f32;

                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(r, &world);
            }
            col /= ns as f32;
            col = vec3::Vec3{ e: [col.r().sqrt(), col.g().sqrt(), col.b().sqrt()] };
            let icol = col * 255.99;
            println!("{} {} {}", icol.r() as i32, icol.g() as i32, icol.b() as i32);
        }
    }
}
