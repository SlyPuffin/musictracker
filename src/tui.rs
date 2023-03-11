use cursive::Cursive;
use cursive::views::{Checkbox, LinearLayout};

fn main() {
    let mut siv = cursive::default();

    // Create a linear layout with horizontal orientation
    let mut grid = LinearLayout::horizontal();

    // Add 3 vertical linear layouts to the horizontal layout
    for _ in 0..3 {
        let mut row = LinearLayout::vertical();

        // Add 16 checkboxes to each vertical layout
        for _ in 0..16 {
            row.add_child(Checkbox::new());
        }

        // Add the vertical layout to the horizontal layout
        grid.add_child(row);
    }

    // Add the grid layout to the TUI
    siv.add_layer(grid);

    // Run the TUI event loop
    siv.run();
}

