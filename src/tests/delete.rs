use crate::VirtualEditor;

#[test]
pub fn delete_at_end() {
    let mut ed = VirtualEditor::new();

    ed.insert_str_move("Hello world!");
    ed.delete_char_left();
    ed.delete_char_left();
    ed.delete_char_left();

    assert_eq!(ed.text(), "Hello wor");
}

#[test]
pub fn delete_at_begin() {
    let mut ed = VirtualEditor::new();

    ed.insert_str("Hello world!");
    ed.delete_char_right();
    ed.delete_char_right();
    ed.delete_char_right();
    ed.delete_char_right();

    assert_eq!(ed.text(), "o world!");
}

#[test]
pub fn delete_at_middle() {
    let mut ed = VirtualEditor::new();

    ed.insert_str_move("Hello world!");
    ed.move_to_position(1, 6).unwrap();
    ed.delete_char_right();

    assert_eq!(ed.text(), "Helloworld!");
}