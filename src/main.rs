use std::io::BufReader;
use rodio::{OutputStream, PlayError, Source};
use std::time::Duration;
use csv::Reader;
use std::fs::File;

fn main() {
    let delay_time = 300;

    let csv_file = File::open("drums_2.csv").unwrap();
    let reader = BufReader::new(csv_file);
    let mut csv_reader = Reader::from_reader(reader);

    let mut cymbal_input_vec: Vec<bool> = Vec::new();
    let mut kick_input_vec: Vec<bool> = Vec::new();
    let mut snare_input_vec: Vec<bool> = Vec::new();
    let mut cymbal_output_vec: Vec<bool> = Vec::new();
    let mut kick_output_vec: Vec<bool> = Vec::new();
    let mut snare_output_vec: Vec<bool> = Vec::new();

    for result in csv_reader.records() {
        let record = result.unwrap();
        let mut iterator = 0;
        for field in record.iter() {
            if iterator == 0 {
                if field == String::from("x") {
                    cymbal_input_vec.push(true);
                } else {
                    cymbal_input_vec.push(false);
                }
            }
            if iterator == 1 {
                if field == String::from("x") {
                    kick_input_vec.push(true);
                } else {
                    kick_input_vec.push(false);
                }
            }
            if iterator == 2 {
                if field == String::from("x") {
                    snare_input_vec.push(true);
                } else {
                    snare_input_vec.push(false);
                }
            }
            iterator = iterator + 1;
        }
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = std::fs::File::open("custom_assets/ANMNGCHI_kick_01.wav").unwrap();
    let kick_source = rodio::Decoder::new(BufReader::new(file))
        .unwrap()
        .amplify(0.30)
        .buffered();
    let file = std::fs::File::open("custom_assets/ANMNGCHI_cymbal_03.wav").unwrap();
    let cymbal_source = rodio::Decoder::new(BufReader::new(file))
        .unwrap()
        .amplify(0.10)
        .buffered();
    let file = std::fs::File::open("custom_assets/ANMNGCHI_snare_01.wav").unwrap();
    let snare_source = rodio::Decoder::new(BufReader::new(file))
        .unwrap()
        .amplify(0.20)
        .buffered();

    let mut cymbal_vec: Vec<Result<(), PlayError>> = Vec::new();
    let mut kick_vec: Vec<Result<(), PlayError>> = Vec::new();
    let mut snare_vec: Vec<Result<(), PlayError>> = Vec::new();

    for i in 0..cymbal_input_vec.len() {
        let delay = delay_time * i;
        if cymbal_input_vec.remove(0) {
            cymbal_vec.push(stream_handle.play_raw(cymbal_source.clone().delay(Duration::from_millis(delay as u64)).convert_samples()));
            cymbal_output_vec.push(true);
        } else {
            cymbal_output_vec.push(false);
        }
        if kick_input_vec.remove(0) {
            kick_vec.push(stream_handle.play_raw(kick_source.clone().delay(Duration::from_millis(delay as u64)).convert_samples()));
            kick_output_vec.push(true);
        } else {
            kick_output_vec.push(false);
        }
        if snare_input_vec.remove(0) {
            snare_vec.push(stream_handle.play_raw(snare_source.clone().delay(Duration::from_millis(delay as u64)).convert_samples()));
            snare_output_vec.push(true);
        } else {
            snare_output_vec.push(false);
        }
    }
    std::thread::sleep(Duration::from_millis((cymbal_output_vec.len() * delay_time) as u64));
    for _i in 0..cymbal_output_vec.len() {
        if cymbal_output_vec.remove(0) {
            drop(cymbal_vec.pop());
        }
        if kick_output_vec.remove(0) {
            drop(kick_vec.pop());
        }
        if snare_output_vec.remove(0) {
            drop(snare_vec.pop());
        }
    }
}
