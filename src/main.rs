mod geometry;

use geometry::{Vec3};

struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
}

fn main() {
    let buffer: [u8; 30000] = [100; 30000];

    let mut count = 0;

    let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    println!("{}", v.x);

    for i in 0..100 {
        for j in 0..100 {
            count += 1;
        }
    }

    println!("{}", count);

    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, 100, 100, image::ColorType::Rgb8)
        .unwrap(); 
}
