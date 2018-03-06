extern crate image;
use image::{GenericImage, Rgb, Pixel, RgbImage};
use std::{time, thread};
use std::sync::Arc;
use std::str;
mod animator;

extern crate netopt;
extern crate mqttc;
extern crate mqtt3;
extern crate floating_duration;

use netopt::NetworkOptions;
use mqttc::{PubSub, ClientOptions, ReconnectMethod, PubOpt};

const TOPIC_BEAT: &str = "untzifier/output/beat";
const TOPIC_BPM: &str = "untzifier/output/bpm";

fn get_row(image: &RgbImage, y: u32) -> Vec<Rgb<u8>> {
    let mut row: Vec<Rgb<u8>> = Vec::with_capacity(image.width() as usize);

    for x in 0..image.height() {
        row.push(image.get_pixel(x, y).to_rgb());
    }

    return row;
}

fn load_anim(name: &str) -> animator::Animation {
    let img = image::open(name).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    println!("dimensions: {:?}", img.dimensions());

    let img_rgb = img.to_rgb();

    let height = img_rgb.height();

    let mut anim: animator::Animation = Vec::new();
    for y in 0..height {
        let row = get_row(&img_rgb, y);
        anim.push(row);
    }

    return anim;
}

fn main() {
    println!("Hello, world!");

    let netopt = NetworkOptions::new();
    let mut opts = ClientOptions::new();
    opts.set_reconnect(ReconnectMethod::ReconnectAfter(time::Duration::from_secs(1)));
    let mut client = opts.connect("127.0.0.1:1883", netopt).expect("Can't connect to server");
    client.subscribe(TOPIC_BEAT).unwrap();
    client.subscribe(TOPIC_BPM).unwrap();
    //client.publish("topic", "msg", PubOpt::at_most_once()).unwrap();


    
    let anim = load_anim("sprites/10_8.png");
    let animator = animator::Animator::new(anim);

    let anim_cell = Arc::new(animator);

    let anim_cell1 = anim_cell.clone();
    let t = thread::spawn(move || { 
        loop {
            anim_cell1.tick();
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    loop {
        match client.await() {
            Ok(result) => {
                match result {
                    Some(message) => {
                        println!("topic {}", message.topic.path());
                        match message.topic.path().as_ref() {
                            TOPIC_BEAT => {
                                anim_cell.beat();
                            },
                            TOPIC_BPM => {
                                let bpm_str = str::from_utf8(&message.payload).unwrap();
                                match bpm_str.parse::<u32>() {
                                    Ok(v) => println!("new bpm {}", v),
                                    Err(_) => ()
                                };
                            }
                            _ => ()
                        }
                    }
                    None => println!("."),
                }
            }
            Err(_) => continue
        }
    }

    t.join().unwrap();
}