extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Checkboxes};

fn main() {
    let checkboxes = &[
        ("Ice Cream", false),
        ("Vanilla Cupcake", true),
        ("Chocolate Muffin", false),
        ("A Pile of sweet, sweet mustard", false)
    ];
    let states = &[
        false,
        true,
        false,
        false
    ];
    let selections = Checkboxes::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your food")
        .items_with_states(&checkboxes[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", checkboxes[selection].0);
        }
    }
}
