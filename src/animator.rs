extern crate image;
use std::cell::Cell;
use std::sync::Mutex;
use self::image::Rgb;

extern crate floating_duration;
use self::floating_duration::{TimeAsFloat};

use std::time;
/*
every beat, sync playback
playback loops around at speed determined by bpm
- get current progress %
each image has "beatpoints"
- every 1 / bpi %
*/

pub type Animation = Vec<Vec<Rgb<u8>>>;
pub struct Animator {
    animation: Animation,
    /// beats per minute
    bpm: u32,
    /// beats per image
    bpi: u32,
    /// current playback progress
    progress: Mutex<f64>,
    last_tick: Mutex<time::Instant>
}

impl Animator {
    pub fn new(animation: Animation) -> Animator {
        return Animator {
            animation,
            bpm: 160,
            bpi: 8,

            progress: Mutex::new(0.0),
            last_tick: Mutex::new(time::Instant::now())
        }
    }

    pub fn progress(&self) -> f64 {
        return *self.progress.lock().unwrap();
    }

    pub fn tick(&self) {
        let mut last_tick = self.last_tick.lock().unwrap();
        let beats_passed = (self.bpm as f64) / 60.0 * last_tick.elapsed().as_fractional_secs();
        let mut progress = self.progress.lock().unwrap();
        *progress += beats_passed / (self.bpi as f64);
        *progress %= 1.0;
        //println!("playing, bpm={}, {:?}", self.bpm, *progress);

        *last_tick = time::Instant::now();
    }

    /// Synchronizes the progress to the beat
    pub fn beat(&self) {
        let granulity = 1.0 / (self.bpi as f64);
        let progress = self.progress.lock().unwrap();
        let new_progress = (*progress / granulity).floor() * granulity;
        let skew = new_progress - *progress;
        println!("beat {}, {}, {}", granulity, *progress, skew);
    }
}