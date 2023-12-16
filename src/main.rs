use image::{Rgb, RgbImage, imageops::flip_vertical_in_place};

const WHITE: Rgb<u8> = Rgb([255,0,0]);
const RED: Rgb<u8> = Rgb([255,0,0]);

fn main() {
    let mut img = RgbImage::new(100, 100);

    img.put_pixel(52, 41, RED);
    flip_vertical_in_place(&mut img);
    
    match img.save("img.bmp") {
        Ok(_) => (),
        Err(why) => panic!("{:?}", why)
    }
}
