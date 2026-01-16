//! Global state for the current game session.

// Removed unused import (Vector2) to stop compiler warnings

pub struct GameData {
    pub points: u32,
    pub screen_width: i32,
    pub screen_height: i32,
}

impl GameData {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            points: 0,
            screen_width: width,
            screen_height: height,
        }
    }

    // simple score increment
    pub fn score(&mut self) {
        self.points += 1;
    }
}