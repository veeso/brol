use rodio::source::{Amplify, SineWave, Source, TakeDuration};
use rodio::{OutputStream, Sink};
use std::time::Duration;

fn wave(freq: f32, millis: u64, amplify: f32) -> Amplify<TakeDuration<SineWave>> {
    SineWave::new(freq)
        .take_duration(Duration::from_millis(millis))
        .amplify(amplify)
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(wave(40.0, 750, 1.5));
    sink.append(wave(30.0, 500, 1.5));
    sink.append(wave(20.0, 750, 2.0));
    sink.append(wave(1200.0, 60, 0.1));
    sink.sleep_until_end();
}
