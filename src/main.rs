use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::thread;

use clap::Parser;
use time::OffsetDateTime;

use web_audio_api::context::{AudioContext, BaseAudioContext, AudioContextOptions, AudioContextLatencyCategory};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};

#[derive(Parser, Debug)]
enum Mode {
    /// Read the current time.
    Read,
    /// Start a timer that counts up (unbounded).
    CountUp,
    /// Start a timer that counts down (with an alarm).
    CountDown { duration: String },
}

fn format_duration(t: Duration) -> String {
    let mut t = t.as_secs_f64().round() as u64;
    let (s, m);
    (s, t) = (t % 60, t / 60);
    (m, t) = (t % 60, t / 60);
    let h = t;

    format!("{h:>02}:{m:>02}:{s:>02}")
}

fn main() {
    match Mode::parse() {
        Mode::Read => {
            println!("{}", OffsetDateTime::now_local().unwrap());
        }
        Mode::CountUp => {
            let start = Instant::now();
            let sleep_time = Duration::from_millis(250);
            loop {
                print!("\r{}    ", format_duration(start.elapsed()));
                io::stdout().lock().flush().unwrap();
                thread::sleep(sleep_time);
            }
        }
        Mode::CountDown { duration } => {
            let duration = match duration.split(':').map(|x| x.parse::<u64>()).collect::<Result<Vec<_>,_>>() {
                Ok(tokens) if (1..=3).contains(&tokens.len()) => {
                    let mut res = 0;
                    for x in tokens.iter() {
                        res = 60 * res + x;
                    }
                    Duration::from_secs(res)
                }
                _ => panic!("duration should be of form ((h:)m:)s, got \"{duration}\""),
            };

            let start = Instant::now();
            let sleep_time = Duration::from_millis(250);
            while let Some(t) = duration.checked_sub(start.elapsed()) {
                print!("\r+{}    ", format_duration(t));
                io::stdout().lock().flush().unwrap();
                thread::sleep(sleep_time);
            }

            let context = AudioContext::new(AudioContextOptions {
                latency_hint: AudioContextLatencyCategory::Playback,
                ..AudioContextOptions::default()
            });
            let mut src = context.create_buffer_source();
            src.set_buffer(context.decode_audio_data_sync(include_bytes!("alarm.mp3").as_slice()).unwrap());
            src.set_loop(true);
            src.connect(&context.destination());
            src.start();

            loop {
                print!("\r-{}    ", format_duration(start.elapsed().saturating_sub(duration)));
                io::stdout().lock().flush().unwrap();
                thread::sleep(sleep_time);
            }
        }
    }
}
