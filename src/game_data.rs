//! The data for each game session. 
//! 
//! You could also store data associated with each human player here.
//! We could also store the player's gamepad_id here.

//use raylib::ffi::{Texture2D, Vector2};
use raylib::texture::Texture2D;

pub struct GameData {
    pub points: u32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub player_walk_tex: Vec<Texture2D>,
   // pub player_run_tex: Vec<Texture2D>,
    // pub player_idle_tex: Vec<Texture2D>
    pub p1_facing_left: bool,
    pub p1_current_state: f32,
    pub background_tex_vec: Vec<Texture2D>,
    pub obstacle_tex_vec: Vec<Texture2D>
}

impl GameData {
    pub fn new(
        width: i32,
         height: i32,
         player_walk: Vec<Texture2D>,
        // player_run: Vec<Texture2D>,
        // player_idle: Vec<Texture2D>
        background_tex: Vec<Texture2D>,
        obstacle_tex: Vec<Texture2D>
        )-> Self  {
        Self {
            points: 0,
            screen_width: width,
            screen_height: height,
           // player_idle_tex: player_idle,
     //       player_run_tex: player_run,
            player_walk_tex: player_walk,
            p1_facing_left: false,
            p1_current_state: 0.0,
            background_tex_vec: background_tex,
            obstacle_tex_vec: obstacle_tex
        }
    }

    /// add one to the player's total points.
    pub fn score(&mut self) {
        self.points += 1;
    }
}