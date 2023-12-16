use std::{io::BufReader, fs::File};

use image::{Rgb, RgbImage, imageops::flip_vertical_in_place, ImageBuffer};
use wavefront::Obj;

const WHITE: Rgb<u8> = Rgb([255,255,255]);
const RED: Rgb<u8> = Rgb([255,0,0]);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn draw_line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: Rgb<u8>) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 {1} else {-1};
    let sy = if y0 < y1 {1} else {-1};
    let mut err = dx - dy;

    while x0 != x1 || y0 != y1 {
        let e2 = 2 * err;
        if e2 >= -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
        img.put_pixel(x0.try_into().unwrap(), y0.try_into().unwrap(), colour)
    }
}

fn main() {
    let file = File::open("african_head.obj").unwrap();
    let reader = BufReader::new(file);
    let obj = Obj::from_reader(reader).unwrap();

    let mut img = RgbImage::new(WIDTH, HEIGHT);
    

    obj.polygons().for_each(|polygon| {
        for i in 0..polygon.vertices().count() {
            let v0 = polygon.vertex(i).unwrap();
            let v1 = polygon.vertex((i+1) % 3).unwrap();
            let x0 = (v0.position()[0] + 1.) * (WIDTH - 1) as f32 / 2.;
            let y0 = (v0.position()[1] + 1.) * (HEIGHT - 1) as f32 / 2.;
            let x1 = (v1.position()[0] + 1.) * (WIDTH - 1) as f32 / 2.;
            let y1 = (v1.position()[1] + 1.) * (HEIGHT - 1) as f32 / 2.;
            draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, &mut img, WHITE)
        }
    });
    
    flip_vertical_in_place(&mut img);
    
    match img.save("img.bmp") {
        Ok(_) => (),
        Err(why) => panic!("{:?}", why)
    }
}
