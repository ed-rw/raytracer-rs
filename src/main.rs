mod vec3;
mod ray;

fn hit_sphere(center: vec3::Vec3, radius: f32, r: ray::Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: ray::Ray) -> vec3::Vec3 {
    if hit_sphere(vec3::Vec3{ e: [0.0, 0.0, -1.0]}, 0.5, r) {
        return vec3::Vec3{ e: [1.0, 0.0, 0.0]};
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Vec3{ e: [1.0, 1.0, 1.0]} + t * vec3::Vec3{ e: [0.0, 0.0, 0.0]}
}

fn main() {
    let nx = 4000;
    let ny = 2000;

    print!("P3\n{} {} \n255\n", nx, ny);

    let lower_left_corner = vec3::Vec3{ e: [-2.0, -1.0, -1.0]};
    let horizontal = vec3::Vec3{ e: [4.0, 0.0, 0.0]};
    let vertical = vec3::Vec3{ e: [0.0, 2.0, 0.0]};
    let origin = vec3::Vec3{ e: [0.0, 0.0, 0.0]};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = ray::Ray{ a: origin, b: lower_left_corner + u*horizontal + v*vertical};

            let icol = color(r) * 255.99;
            println!("{} {} {}", icol.r() as i32, icol.g() as i32, icol.b() as i32);
        }
    }
}
