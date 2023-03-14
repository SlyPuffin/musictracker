use cursive::traits::Nameable;
use cursive::views::Checkbox;
use cursive::views::{Button, LinearLayout};
use cursive::Cursive;
use rodio::{OutputStream, PlayError, Source};
use std::io::BufReader;
use std::time::Duration;

fn main() {
    let mut siv = cursive::default();

    let mut grid = LinearLayout::vertical();

    for y in 0..16 {
        let mut row = LinearLayout::horizontal();

        for x in 0..3 {
            row.add_child(Checkbox::new().with_name(format!("{}{}", x, y).as_str()));
        }

        grid.add_child(row);
    }

    let invert_button = Button::new("Invert", invert);
    grid.add_child(invert_button);
    let play_button = Button::new("Play", play);
    grid.add_child(play_button);

    siv.add_layer(grid);

    siv.run();
}

fn invert(s: &mut Cursive) {
    for y in 0..16 {
        for x in 0..3 {
            s.call_on_name(format!("{}{}", x, y).as_str(), |view: &mut Checkbox| {
                view.set_checked(!view.is_checked());
            });
        }
    }
}

fn play(s: &mut Cursive) {
    let mut cymbal_input_vec: Vec<bool> = Vec::new();
    let mut kick_input_vec: Vec<bool> = Vec::new();
    let mut snare_input_vec: Vec<bool> = Vec::new();
    let mut cymbal_output_vec: Vec<bool> = Vec::new();
    let mut kick_output_vec: Vec<bool> = Vec::new();
    let mut snare_output_vec: Vec<bool> = Vec::new();
    let delay_time = 300;

    for y in 0..16 {
        for x in 0..3 {
            s.call_on_name(format!("{}{}", x, y).as_str(), |view: &mut Checkbox| {
                if x == 0 {
                    cymbal_input_vec.push(view.is_checked());
                }
                if x == 1 {
                    kick_input_vec.push(view.is_checked());
                }
                if x == 2 {
                    snare_input_vec.push(view.is_checked());
                }
            });
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
            cymbal_vec.push(
                stream_handle.play_raw(
                    cymbal_source
                        .clone()
                        .delay(Duration::from_millis(delay as u64))
                        .convert_samples(),
                ),
            );
            cymbal_output_vec.push(true);
        } else {
            cymbal_output_vec.push(false);
        }
        if kick_input_vec.remove(0) {
            kick_vec.push(
                stream_handle.play_raw(
                    kick_source
                        .clone()
                        .delay(Duration::from_millis(delay as u64))
                        .convert_samples(),
                ),
            );
            kick_output_vec.push(true);
        } else {
            kick_output_vec.push(false);
        }
        if snare_input_vec.remove(0) {
            snare_vec.push(
                stream_handle.play_raw(
                    snare_source
                        .clone()
                        .delay(Duration::from_millis(delay as u64))
                        .convert_samples(),
                ),
            );
            snare_output_vec.push(true);
        } else {
            snare_output_vec.push(false);
        }
    }
    std::thread::sleep(Duration::from_millis(
        (cymbal_output_vec.len() * delay_time) as u64,
    ));
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
