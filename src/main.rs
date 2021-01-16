mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;

fn color(r: ray::Ray, world: &dyn hitable::Hitable, depth: i32) -> vec3::Vec3 {
    let wrapped_rec = world.hit(r, 0.001, f32::INFINITY);
    if !wrapped_rec.is_none() {
        let rec = wrapped_rec.unwrap();
        let (attenuation, scattered, scatter_bool) = rec.material.scatter(r, rec);
        if depth < 50 && scatter_bool {
            return attenuation * color(scattered, world, depth + 1);
        } else {
            return vec3::Vec3::new(0, 0, 0);
        }
    } else {
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * vec3::Vec3::new(1, 1, 1)
            + t * vec3::Vec3::new(0.5, 0.7, 1.0);
    }
}

fn generate_world() -> Vec<Box<dyn hitable::Hitable>>{

    let mut objs = Vec::new();

    let mat = Box::new(material::Lambertian {
        albedo: vec3::Vec3::new(0.5, 0.5, 0.5),
    }) as Box<dyn material::Material>;

    objs.push(Box::new(sphere::Sphere {
        center: vec3::Vec3::new(0, -1000, 0),
        radius: 1000.0,
        material: mat,
    }) as Box<dyn hitable::Hitable>);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = vec3::Vec3::new(
                    (a as f32)+0.9*rand::random::<f32>(), 0.2,
                    (b as f32)+0.9*rand::random::<f32>()
            );

            let base = vec3::Vec3::new(4, 0.2, 0);

            if (center - base).length() > 0.9 {
                let mat: Box<dyn material::Material>;
                if choose_mat < 0.8 {
                    mat = Box::new(material::Lambertian {
                        albedo: vec3::Vec3::new(rand::random::<f32>()*rand::random::<f32>(), rand::random::<f32>()*rand::random::<f32>(), rand::random::<f32>()*rand::random::<f32>()),
                    }) as Box<dyn material::Material>;
                } else if choose_mat < 0.95 {
                    mat = Box::new(material::Metal {
                            albedo: vec3::Vec3::new(1.0 + rand::random::<f32>(), 0.5*(1.0+rand::random::<f32>()), 0.5*rand::random::<f32>()),
                            fuzz: 0.5*rand::random::<f32>(),
                        }) as Box<dyn material::Material>;
                } else {
                    mat = Box::new(material::Dielectric {
                            refraction_index: 1.7,
                        }) as Box<dyn material::Material>;
                }


                objs.push(Box::new(sphere::Sphere {
                    center: center,
                    radius: 0.2,
                    material: mat,
                }) as Box<dyn hitable::Hitable>);
            }
        }
    }

    let mat = Box::new(material::Lambertian {
        albedo: vec3::Vec3::new(0.4, 0.2, 0.1),
    }) as Box<dyn material::Material>;

    objs.push(Box::new(sphere::Sphere {
        center: vec3::Vec3::new(-4, 1, 0),
        radius: 1.0,
        material: mat,
    }) as Box<dyn hitable::Hitable>);

    let mat = Box::new(material::Metal {
        albedo: vec3::Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    }) as Box<dyn material::Material>;

    objs.push(Box::new(sphere::Sphere {
        center: vec3::Vec3::new(4, 1, 0),
        radius: 1.0,
        material: mat,
    }) as Box<dyn hitable::Hitable>);

    let mat = Box::new(material::Dielectric {
        refraction_index: 1.7,
    }) as Box<dyn material::Material>;

    objs.push(Box::new(sphere::Sphere {
        center: vec3::Vec3::new(0, 1, 0),
        radius: 1.0,
        material: mat,
    }) as Box<dyn hitable::Hitable>);

    return objs;
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 10;

    print!("P3\n{} {} \n255\n", nx, ny);

    let objs = generate_world();


    // let objs = vec![
    //     Box::new(sphere::Sphere {
    //         center: vec3::Vec3 {
    //             e: [0.0, 0.0, -1.0],
    //         },
    //         radius: 0.5,
    //         material: &material::Lambertian {
    //             albedo: vec3::Vec3 { e: [0.1, 0.2, 0.5] },
    //         },
    //     }) as Box<dyn hitable::Hitable>,
    //     Box::new(sphere::Sphere {
    //         center: vec3::Vec3 {
    //             e: [0.0, -100.5, -1.0],
    //         },
    //         radius: 100.0,
    //         material: &material::Lambertian {
    //             albedo: vec3::Vec3 { e: [0.8, 0.8, 0.0] },
    //         },
    //     }) as Box<dyn hitable::Hitable>,
    //     Box::new(sphere::Sphere {
    //         center: vec3::Vec3 {
    //             e: [1.0, 0.0, -1.0],
    //         },
    //         radius: 0.5,
    //         material: &material::Metal {
    //             albedo: vec3::Vec3 { e: [0.8, 0.6, 0.2] },
    //             fuzz: 0.01,
    //         },
    //     }) as Box<dyn hitable::Hitable>,
    //     Box::new(sphere::Sphere {
    //         center: vec3::Vec3 {
    //             e: [-1.0, 0.0, -1.0],
    //         },
    //         radius: 0.5,
    //         material: &material::Dielectric {
    //             refraction_index: 1.7,
    //         },
    //     }) as Box<dyn hitable::Hitable>,
    //     Box::new(sphere::Sphere {
    //         center: vec3::Vec3 {
    //             e: [-1.0, 0.0, -1.0],
    //         },
    //         radius: -0.45,
    //         material: &material::Dielectric {
    //             refraction_index: 1.7,
    //         },
    //     }) as Box<dyn hitable::Hitable>,
    // ];


    let world = hitable::HitableList { list: objs };


    let lookfrom = vec3::Vec3::new(5, 2, 4);
    let lookat = vec3::Vec3::new(0, 0, -1);
    let cam = Box::new(camera::NoBlurCamera::new(
        lookfrom,
        lookat,
        vec3::Vec3 { e: [0.0, 1.0, 0.0] },
        50.0,
        nx as f32 / ny as f32,
        // 2.0,
        // (lookfrom-lookat).length()
    )) as Box<dyn camera::Camera>;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = vec3::Vec3::new(0, 0, 0);
            for _s in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;

                let r = cam.get_ray(u, v);
                col += color(r, &world, 0);
            }
            col /= ns as f32;
            col = vec3::Vec3 {
                e: [col.r().sqrt(), col.g().sqrt(), col.b().sqrt()],
            };
            let icol = col * 255.99;
            println!(
                "{} {} {}",
                icol.r() as i32,
                icol.g() as i32,
                icol.b() as i32
            );
        }
    }
}
