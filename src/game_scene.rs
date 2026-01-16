// the core gameplay scene
// this handles the 1v1 arena, inputs, physics, and rendering

use raylib::prelude::*;
use crate::scenes::{Scene, SceneSwitch};
use crate::game_data::GameData;
use crate::utils::*;
use crate::projectile::Projectile; 
use crate::player::Player; 

pub struct GameScene {
    points: Vec<Vector2>,
    players: Vec<Player>, 
    projectiles: Vec<Projectile>, 
    gravity: f32,
}

impl GameScene {
    pub fn new(_n: usize, width: i32, height: i32) -> Self {
        Self { 
            points: Vec::new(),
            // spawn 2 players: left (blue) and right (red)
            players: vec![
                Player::new(0, 200.0, 400.0, Color::BLUE), // player 1
                Player::new(1, 1000.0, 400.0, Color::RED)  // player 2
            ],
            projectiles: Vec::new(), 
            gravity: 900.0, 
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}

    fn handle_input(&mut self, rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        
        let mut new_shots = Vec::new(); 

        for player in &mut self.players {
            
            // firing cooldown
            if player.shoot_timer > 0.0 {
                player.shoot_timer -= rl.get_frame_time();
            }

            let mut direction = 0.0;
            let mut aimed_with_stick = false;

            // gamepad input
            if rl.is_gamepad_available(player.input_id) {
                // move
                let axis_x = rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
                if axis_x.abs() > 0.1 { direction = axis_x; }

                // jump
                if player.grounded && rl.is_gamepad_button_pressed(player.input_id, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    player.vel.y = -550.0;
                    player.grounded = false;
                }
                
                // aim
                let aim_x = rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
                let aim_y = rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_Y);
                let aim_input = Vector2::new(aim_x, aim_y);
                
                if aim_input.length() > 0.1 { 
                    player.aim = aim_input.normalized(); 
                    aimed_with_stick = true;
                }
            }

            // kbm input (player 1 only)
            if player.input_id == 0 {
                if rl.is_key_down(KeyboardKey::KEY_A) { direction = -1.0; }
                if rl.is_key_down(KeyboardKey::KEY_D) { direction = 1.0; }
                if player.grounded && (rl.is_key_pressed(KeyboardKey::KEY_W) || rl.is_key_pressed(KeyboardKey::KEY_SPACE)) {
                    player.vel.y = -550.0;
                    player.grounded = false;
                }
                
                // only use mouse if controller isnt aiming
                if !aimed_with_stick {
                    let mouse_pos = rl.get_mouse_position();
                    let center = Vector2::new(player.pos.x + 15.0, player.pos.y + 15.0);
                    let diff = mouse_pos - center;
                    if diff.length() > 0.0 { player.aim = diff.normalized(); }
                }
            }
            
            // keyboard input (player 2)
            if player.input_id == 1 {
                 if rl.is_key_down(KeyboardKey::KEY_LEFT) { direction = -1.0; }
                 if rl.is_key_down(KeyboardKey::KEY_RIGHT) { direction = 1.0; }
                 if player.grounded && (rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_RIGHT_CONTROL)) {
                     player.vel.y = -550.0;
                     player.grounded = false;
                 }
            }

            player.vel.x = direction * 300.0;

            // shooting logic
            let mut shoot = false;

            // gamepad trigger
            if rl.is_gamepad_available(player.input_id) {
                let trigger = rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER);
                if trigger > 0.5 { shoot = true; }
            }

            // mouse/key press
            if player.input_id == 0 && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) { shoot = true; }
            if player.input_id == 1 && rl.is_key_pressed(KeyboardKey::KEY_ENTER) { shoot = true; }

            // fire if button pressed AND cooldown is 0
            if shoot && player.shoot_timer <= 0.0 {
                let center = Vector2::new(player.pos.x + 15.0, player.pos.y + 15.0);
                let speed = 800.0;
                new_shots.push(Projectile::new(center, player.aim * speed, player.input_id, Color::BLACK));
                
                // reset timer (0.5s delay)
                player.shoot_timer = 0.5; 
            }
        }

        self.projectiles.extend(new_shots);
        SceneSwitch::None
    }

    fn update(&mut self, dt: f32, _data: &mut GameData) -> SceneSwitch {
        // move projectiles
        for p in &mut self.projectiles {
            p.update(dt);
        }

        // collision logic
        for p in &mut self.projectiles {
            if !p.active { continue; }

            for player in &mut self.players {
                // dont shoot urself
                if p.owner_id == player.input_id { continue; }

                // hit detection
                if check_collision_circle_rec(p.pos, 10.0, player.rect()) {
                    p.active = false; // destroy bullet
                    player.pos.y = 0.0; // respawn player
                    player.vel = Vector2::zero();
                }
            }
        }

        // clean up old bullets
        self.projectiles.retain(|p| p.active);
        
        // physics update
        let floor_y = 600.0; 
        for player in &mut self.players {
            player.vel.y += self.gravity * dt;
            player.pos.x += player.vel.x * dt;
            player.pos.y += player.vel.y * dt;

            if player.pos.y > floor_y {
                player.pos.y = floor_y;
                player.vel.y = 0.0;
                player.grounded = true;
            }
        }

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, _data: &mut GameData) {
        d.clear_background(Color::RAYWHITE);

        // draw floor
        d.draw_rectangle(0, 600 + 30, 1280, 50, Color::ORANGE); 

        for p in &self.projectiles {
            p.draw(d);
        }
        
        // draw players
        for player in &self.players {
            d.draw_rectangle_rec(player.rect(), player.color);

            // draw wand/aim line
            let center = Vector2::new(player.pos.x + 15.0, player.pos.y + 15.0);
            let end = center + (player.aim * 50.0);
            d.draw_line_ex(center, end, 3.0, Color::BLACK);
        }
    }
}