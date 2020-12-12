use image::{ImageBuffer, DynamicImage, GrayImage};
use rand::prelude::*;

fn reverse(v: u8) -> u8 {
    if v == 255{
        return 0u8
    }
    return 255u8
}

fn main() {
    let img_rgb = image::open("res/input.jpeg").unwrap().into_rgb8();
    let img_luma = DynamicImage::ImageRgb8(img_rgb).into_luma8();
    let img : GrayImage = ImageBuffer::from_fn(img_luma.dimensions().0, img_luma.dimensions().1, |x, y| {
        if img_luma.get_pixel(x, y).0[0] < 127 {
            image::Luma([0u8])
        }else{
            image::Luma([255u8])
        }
    });
 
    img.save("input_1bit.png").unwrap();
    println!("dimensions {:?}", img.dimensions());

    let mut share1 : GrayImage = ImageBuffer::new(img.dimensions().0 * 2, img.dimensions().1 * 2);
    let mut share2 : GrayImage = ImageBuffer::new(img.dimensions().0 * 2, img.dimensions().1 * 2);

    let patterns = [(255, 255, 0, 0),
                    (255, 0, 255, 0),
                    (255, 0, 0, 255),
                    (0, 255, 255, 0),
                    (0, 255, 0, 255),
                    (0, 0, 255, 255)];

    let mut rng = rand::thread_rng();

    for x in 0..img.dimensions().0 {
        for y in 0..img.dimensions().1 {
            let pixel = img.get_pixel(x, y);
            let pat = patterns[rng.gen_range(0, patterns.len())];

            share1.put_pixel(x*2, y*2, image::Luma([pat.0]));
            share1.put_pixel(x*2+1, y*2, image::Luma([pat.1]));
            share1.put_pixel(x*2, y*2+1, image::Luma([pat.2]));
            share1.put_pixel(x*2+1, y*2+1, image::Luma([pat.3]));
            //println!("{:?}", pat);
            if pixel.0[0] == 0{
                share2.put_pixel(x*2, y*2, image::Luma([reverse(pat.0)]));
                share2.put_pixel(x*2+1, y*2, image::Luma([reverse(pat.1)]));
                share2.put_pixel(x*2, y*2+1, image::Luma([reverse(pat.2)]));
                share2.put_pixel(x*2+1, y*2+1, image::Luma([reverse(pat.3)]));
            }else{
                share2.put_pixel(x*2, y*2, image::Luma([pat.0]));
                share2.put_pixel(x*2+1, y*2, image::Luma([pat.1]));
                share2.put_pixel(x*2, y*2+1, image::Luma([pat.2]));
                share2.put_pixel(x*2+1, y*2+1, image::Luma([pat.3]));
            }
    
        }

    }
    share1.save("share1.png").unwrap();
    share2.save("share2.png").unwrap();
}