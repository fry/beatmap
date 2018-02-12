extern crate image;
use image::{GenericImage, Rgb, Pixel, RgbImage};

use std::{thread, time};

type Animation = Vec<Vec<Rgb<u8>>>;
struct Animator {
    animation: Animation,
    bpm: u32,
    animThread: thread::JoinHandle<()>,
}

impl Animator {
    fn playback(&self, delay: time::Duration) {
        for row in self.animation {
            println!("playing");
            thread::sleep(delay);
        }
    }

    fn start(&self) {
        self.animThread =
            thread::spawn(move || { self.playback(time::Duration::from_millis(500)); });
    }
}

fn get_row(image: &RgbImage, y: u32) -> Vec<Rgb<u8>> {
    // Preallocate array
    let px = image.get_pixel(0, y).to_rgb();
    let mut row: Vec<Rgb<u8>> = vec![px; image.width() as usize];

    for x in 0..image.height() {
        row[x as usize] = image.get_pixel(x, y).to_rgb();
    }

    return row;
}

fn main() {
    println!("Hello, world!");

    let img = image::open("sprites/10_8.png").unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    println!("dimensions: {:?}", img.dimensions());

    let img_rgb = img.to_rgb();

    let height = img_rgb.height();

    let mut anim: Animation = Vec::new();
    for y in 0..height {
        let row = get_row(&img_rgb, y);
        anim.push(row);
    }
}