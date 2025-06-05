use crate::VirtualEditor;

#[test]
pub fn insert_str_move() {
    let mut ed = VirtualEditor::new();

    ed.insert_str_move("Hello world! ");
    ed.insert_str_move("And new world greets you!");

    assert_eq!(ed.text(), "Hello world! And new world greets you!");
}

#[test]
pub fn insert_str_move_new_line() {
    let mut ed = VirtualEditor::new();

    ed.insert_str_move("Hello world!\n");
    ed.insert_str_move("And new world greets you!");

    assert_eq!(ed.text(), "Hello world!\nAnd new world greets you!");
}

#[test]
pub fn insert_str_move_new_line_many() {
    let mut ed = VirtualEditor::new();

    ed.insert_str_move("Yksi\nKaksi\nKolme\n");
    ed.insert_str_move("Neljä\nViisi\nKuusi");

    assert_eq!(ed.text(), "Yksi\nKaksi\nKolme\nNeljä\nViisi\nKuusi");

    let position = ed.cursor();
    assert_eq!(position.x, 5);
    assert_eq!(position.y, 5);
}