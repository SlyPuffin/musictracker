use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::Checkbox;
use cursive::views::{LinearLayout, Button};

fn main() {
    let mut siv = cursive::default();

    let mut grid = LinearLayout::vertical();

    for y in 0..16 {
        let mut row = LinearLayout::horizontal();

        for x in 0..3 {
            row.add_child(Checkbox::new()
                .with_name(format!("{}{}", x, y).as_str()));
        }

        grid.add_child(row);
    }

    let mut button = Button::new("Evaluate", evaluate);
    grid.add_child(button);

    siv.add_layer(grid);

    siv.run();
}

fn evaluate(s: &mut Cursive) {
    for y in 0..16 {
        for x in 0..3 {
            s.call_on_name(format!("{}{}", x, y).as_str(), |view: &mut Checkbox| {
                view.set_checked(!view.is_checked());
            });
        }
    }
}

fn populateVectors(s: &mut Cursive) {
    // TODO: Return the vectors (and borrow them)
    let mut cymbal_input_vec: Vec<bool> = Vec::new();
    let mut kick_input_vec: vec<bool> = vec::new();
    let mut snare_input_vec: Vec<bool> = Vec::new();
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
            }
         }
    }
}