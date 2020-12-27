mod vec3;

fn main() {
    let nx = 200;
    let ny = 100;



    print!("P3\n{} {} \n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let pixel = vec3::Vec3{ e: [i as f32 / nx as f32,
                                        j as f32 / ny as f32,
                                        0.2] };
            let icol = pixel * 255.99;
            println!("{} {} {}", icol.r() as i32, icol.g() as i32, icol.b() as i32);
        }
    }
}
