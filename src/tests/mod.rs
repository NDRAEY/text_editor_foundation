use alloc::borrow::ToOwned;

pub mod movable;

use crate::VirtualEditor;

const UNIVERSAL_MULTILINE_STRING: &str = "Lucario\nRiolu\nZoroark\nPikachu\nZeraora\nLuxio";

#[test]
pub fn simple() {
    let mut ed = VirtualEditor::new();

    ed.insert_str("Hello world!");
    ed.move_right();
    ed.move_right();
    ed.move_right();
    ed.move_right();
    ed.move_right();

    ed.insert_str(" new");

    assert_eq!(ed.text(), "Hello new world!");
}

#[test]
pub fn moving() {
    let mut ed = VirtualEditor::new();

    ed.insert_str("Hello world!");
    ed.move_right();
    ed.move_right();
    ed.move_right();
    ed.move_right();
    ed.move_right();

    ed.insert_str(" new");


    let result = ed.move_to_position(1, 1);

    assert!(result.is_some());

    let result = ed.move_to_position(1, 0);

    assert!(result.is_none());

    let result = ed.move_to_position(3, 2);

    assert!(result.is_none());

}

#[test]
pub fn moving_begin_end() {
    let mut ed = VirtualEditor::with_text(UNIVERSAL_MULTILINE_STRING.to_owned());

    
    ed.move_to_begin();

    let character = ed.get_character_at_cursor();

    assert_eq!(character, 'L');


    ed.move_to_end();

    let character = ed.get_character_at_cursor();

    assert_eq!(character, 'o');
}

#[test]
pub fn lines() {
    let mut ed = VirtualEditor::new();

    ed.insert_str("Hello\nworld!\n");
    ed.move_down();

    ed.insert_str("boring\n");

    assert_eq!(ed.text(), "Hello\nboring\nworld!\n");
}

#[test]
pub fn line_deletion() {
    let mut ed = VirtualEditor::with_text(UNIVERSAL_MULTILINE_STRING.to_owned());

    ed.move_down();  // Riolu
    ed.move_down();  // Zoroark
    ed.move_down();  // Pikachu

    let result = ed.delete_line();

    assert_eq!(result, "Pikachu");

    ed.move_to_end();

    let result = ed.delete_line();

    assert_eq!(result, "Luxio");

    assert_eq!(ed.text(), "Lucario\nRiolu\nZoroark\nZeraora\n");
}

#[test]
pub fn empty_input() {
    let mut ed = VirtualEditor::new();

    let result = ed.delete_line();

    assert_eq!(result, "");

    ed.insert_str("");

    assert_eq!(ed.text(), "");
}

#[test]
pub fn insert_at_begin() {
    let mut ed = VirtualEditor::new();

    ed.insert_str("Hello, world!");
    ed.insert_str("That's cool! ");

    assert_eq!(ed.text(), "That's cool! Hello, world!");
}
