//! The core game play scene
//! 
//! This represents the chase game. Here we store information about the game world and the player's "character".

use raylib::prelude::*;

use crate::menu_scene::WinScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::game_data::GameData;
use crate::utils::*;

pub struct GameScene {
    points: Vec<Vector2>,
    player_position: Vector2,
    player_direction: Vector2,
    player_1_speed: f32,
    gravity: f32,
    player_1_grounded: bool,
    player_1_velo: Vector2,
    walk_frame: usize,
    walk_timing: f32,
    frame_time: f32
}

impl GameScene {
    pub fn new(n: usize, width: i32, height: i32) -> Self {
        let mut points = Vec::new();
        for _ in 0..n {
            points.push(random_point(width, height));
        }
        Self { 
            points: points,
            player_position: Vector2::new((450) as f32, (width - 15) as f32),
            player_direction: Vector2::zero(),
            player_1_speed: 300.0,
            gravity: 0.8,
            player_1_grounded: true,
            player_1_velo: Vector2::zero(),
            walk_frame: 0,
            walk_timing: 0.0,
            frame_time: 0.1
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {
        self.walk_frame = 0;
        self.walk_timing = 0.0;
        }



    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        
        // set the intention to move in the given direction.
        let mut direction = Vector2::zero();
        let gravity: f32 =  0.8;
        if (_rl.is_key_down(KeyboardKey::KEY_A) || 
            _rl.is_key_down(KeyboardKey::KEY_LEFT)) && self.player_position.x > 15.0
        {
            direction += Vector2::new(-1.0, 0.0);
            _data.p1_facing_left = true;
        }
        
        if (_rl.is_key_down(KeyboardKey::KEY_D) || 
            _rl.is_key_down(KeyboardKey::KEY_RIGHT))  && self.player_position.x < _data.screen_width as f32
        {
            direction += Vector2::new(1.0, 0.0);
            _data.p1_facing_left = false;
        }

        if self.player_1_grounded && (_rl.is_key_pressed(KeyboardKey::KEY_W) || _rl.is_key_pressed(KeyboardKey::KEY_UP)){
            self.player_1_velo.y = -1000.0;
            self.player_1_grounded = false;
        }
        
        self.player_direction = direction;
        

        SceneSwitch::None
    }

    fn update(&mut self, _dt: f32, data: &mut GameData) -> SceneSwitch {
        // update position of player, deal with collisions (later ...)

        let player_1_moving = self.player_direction.x != 0.0;
        if player_1_moving {
            self.walk_timing += _dt;

            self.frame_time = 0.1;
            if self.walk_timing >= self.frame_time {
                self.walk_timing = 0.0;
                self.walk_frame = (self.walk_frame + 1) % data.player_walk_tex.len();
            }
        } 
        else {
            self.walk_frame = 0;
            self.walk_timing = 0.0;
        }

        if !self.player_1_grounded {
            self.player_1_velo.y += self.gravity;
        }

        self.player_position.x += self.player_direction.x * self.player_1_speed * _dt;
        self.player_position.y += self.player_1_velo.y * _dt;

        if self.player_position.y >= (data.screen_height -15) as f32 {
            self.player_position.y = (data.screen_height - 15) as f32;
            self.player_1_velo.y = 0.0;
            self.player_1_grounded = true;

        }
        if let Some(last) = self.points.last() {
            // remove the last point.
            if last.distance_to(self.player_position) < 25.0 {
                self.points.pop();
                data.score();
            } 
        } else {
            println!("Deal with win condition, send new scene");
            return SceneSwitch::Push(Box::new(WinScene));
        }


        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITE);
        // // Draw last point in the vector
        // if let Some(last) = self.points.last() {
        //     d.draw_circle(last.x as i32,
        //      last.y as i32, 
        //     20.0, 
        //      Color::BLUE);
        // }


        let back_source = Rectangle::new(0.0, 0.0, 576.0, 324.0);
        let back_draw = Rectangle::new(0.0,0.0, data.screen_width as f32,data.screen_height as f32);
        let back_origin = Vector2::new(0.0, 0.0);
        d.draw_texture_pro(&data.background_tex_vec[0], back_source, back_draw,back_origin, 0.0, Color::WHITE);



        let r1; 
        if data.p1_facing_left {
             r1 = Rectangle::new(0.0, 0.0, -419.0, 380.0);
        }
        else {
            r1 = Rectangle::new(0.0,0.0,419.0,380.0);
        }
        let r2 = Rectangle::new(self.player_position.x as f32, self.player_position.y as f32, 128.0, 128.0);
        let origin = Vector2::new(64.0, 110.0);
        d.draw_texture_pro(&data.player_walk_tex[self.walk_frame], r1, r2, origin, 0.0, Color::WHITE);
        
        let ob_1_source = Rectangle::new(0.0, 0.0, 64.0,64.0);
        let ob_1_destination = Rectangle::new(500.0, 800.0, 128.0, 128.0);
        let ob_origin = Vector2::new(0.0,0.0);

        let ob_2source = Rectangle::new(0.0,0.0,64.0,64.0);
        let ob_2_destination = Rectangle::new(628.0,800.0, 128.0,128.0);
        let ob_2_origin = Vector2::new(0.0,0.0);

        let ob_3source = Rectangle::new(0.0,0.0,64.0,64.0);
        let ob_3_destination = Rectangle::new(756.0,800.0, 128.0,128.0);
        let ob_3_origin = Vector2::new(0.0,0.0);
    
        d.draw_texture_pro(&data.obstacle_tex_vec[0],ob_1_source, ob_1_destination, ob_origin, 0.0, Color::WHITE);
        d.draw_texture_pro(&data.obstacle_tex_vec[1], ob_2source, ob_2_destination, ob_2_origin, 0.0, Color::WHITE);
        d.draw_texture_pro(&data.obstacle_tex_vec[2], ob_3source, ob_3_destination, ob_3_origin, 0.0, Color::WHITE);
        
        
        // Draw score based on game data
        let message = format!("Score: {}", data.points);

        d.draw_text(message.as_str(), 10, data.screen_height - 25, 20, Color::BLACK);
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}