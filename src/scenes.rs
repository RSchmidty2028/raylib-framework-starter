// traits for scenes and scene switching logic
use raylib::prelude::*;
use crate::game_data::GameData;

pub enum SceneSwitch {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
    Quit,
}

pub trait Scene {
    
    // called when scene starts
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}

    // handle user input
    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }

    // update logic
    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }

    // draw everything
    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData);

    // cleanup
    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}