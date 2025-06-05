#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::{string::{String, ToString}, vec::Vec};

#[cfg(test)]
pub mod tests;

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

pub struct VirtualEditor {
    text: String,
    cursor: Position
}

impl Default for VirtualEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualEditor {
    pub fn new() -> Self {
        Self { text: String::new(), cursor: Position::default() }
    }

    pub fn with_text(text: String) -> Self {
        Self { text, cursor: Position::default() }
    }

    pub fn lines(&self) -> Vec<&str> {
        self.text.split('\n').collect()
    }

    pub fn line_count(&self) -> usize {
        self.text.split('\n').count()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn cursor(&self) -> &Position {
        &self.cursor
    }

    pub fn text_position(&self) -> usize {
        let mut position = 0;
        for (i, line) in self.text.lines().enumerate() {
            if i == self.cursor.y {
                // Add the byte length of the characters up to cursor.x in this line
                position += line.chars().take(self.cursor.x).map(|c| c.len_utf8()).sum::<usize>();
                break;
            }
            // Add the byte length of the whole line plus one for the newline character
            position += line.len() + 1;
        }
        position
    }

    pub fn move_up(&mut self) {
        if self.cursor.y == 0 {
            return;
        }
        self.cursor.y -= 1;
    }

    pub fn move_down(&mut self) {
        let num_lines = self.line_count();
        if self.cursor.y == num_lines - 1 {
            return;
        }
        self.cursor.y += 1;
    }

    pub fn move_right(&mut self) {
        if self.cursor.x == self.get_line_at_cursor().len() {
            return;
        }
        self.cursor.x += 1;
    }

    pub fn move_left(&mut self) {
        if self.cursor.x == 0 {
            return;
        }
        self.cursor.x -= 1;
    }

    
    pub fn move_to_position(&mut self, y: usize, x: usize) -> Option<()> {
        if y == 0 || x == 0 {
            return None
        }

        let (y, x) = (y - 1, x - 1);
        
        if y > self.line_count() {
            return None;
        }
        
        if x > self.lines()[y].len() {
            return None;
        }

        self.cursor.x = x;
        self.cursor.y = y;

        Some(())
    }

    pub fn move_to_end(&mut self) {
        self.cursor.y = self.line_count() - 1;
        self.cursor.x = self.lines()[self.line_count() - 1].len() - 1;
    }

    pub fn move_to_begin(&mut self) {
        self.cursor.y = 0;
        self.cursor.x = 0;
    }

    pub fn move_to_line_begin(&mut self) {
        self.cursor.x = 0;
    }

    pub fn move_to_line_end(&mut self) {
        self.cursor.x = self.get_line_at_cursor().len();
    }

    pub fn delete_char_left(&mut self) {
        if self.cursor.x == 0 {
            return;
        }
        self.text.remove(self.text_position() - 1);
    }

    pub fn delete_char_left_nocheck(&mut self) {
        let pos = self.text_position();

        if pos == 0 {
            return;
        }

        self.text.remove(pos - 1);
    }

    pub fn delete_char_right(&mut self) {
        if self.cursor.x == self.lines()[self.cursor.y].len() {
            return;
        }
        self.text.remove(self.text_position());
    }

    pub fn delete_char_right_nocheck(&mut self) {
        let pos = self.text_position();

        if pos >= self.text.len() {
            return;
        }

        self.text.remove(self.text_position());
    }

    pub fn insert_char(&mut self, ch: char) {
        let position = self.text_position();
        //println!("[{} | {:?} | {:?}] Inserting char: {}", position, self.cursor(), &self.text[..position], ch);
        self.text.insert(position, ch);
    }
    
    pub fn insert_str(&mut self, st: &str) {
        self.text.insert_str(self.text_position(), st);
    }

    pub fn insert_char_move(&mut self, ch: char) {
        self.insert_char(ch);

        if ch == '\n' {
            self.move_to_line_begin();
            self.move_down();
        } else {
            self.move_right();
        }
    }

    pub fn insert_str_move(&mut self, st: &str) {
        for ch in st.chars() {
            self.insert_char_move(ch);
        }
    }

    pub fn get_character_at_cursor(&self) -> char {
        self.get_line_at_cursor().chars().nth(self.cursor.x).unwrap()
    }

    pub fn get_line_at_cursor(&self) -> &str {
        self.lines()[self.cursor.y]
    }

    pub fn delete_line(&mut self) -> String {
        let line = self.get_line_at_cursor().to_string();

        self.cursor.x = 0;

        let start = self.text_position();
        let end = self.text_position() + line.len();

        for _ in start..=end {
            if self.text.chars().nth(start).is_none() {
                continue;
            }

            self.text.remove(start);
        }

        line
    }
}
