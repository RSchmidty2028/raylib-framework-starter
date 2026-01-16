use raylib::prelude::*;

use wizards_1v1::game_data::GameData;
use wizards_1v1::menu_scene::MenuScene;
use wizards_1v1::scenes::{Scene, SceneSwitch};

use std::time::Instant;

fn main() {
    let width: i32 = 1280;
    let height: i32 = 720;
    
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Wizard 1v1")
        .build();

    rl.set_target_fps(60);
    // rl.toggle_fullscreen(); // lags my pc so keeping it off for now

    // init game data
    let mut game_data = GameData::new(width, height);

    // scene stack, start with menu
    let mut scenes: Vec<Box<dyn Scene>> = vec![Box::new(MenuScene)];

    let mut last_time = Instant::now();
    let mut keep_playing = true;
     
    // main loop
    while !rl.window_should_close() && keep_playing {
        
        // calc delta time
        let temp = Instant::now();
        let delta = (temp - last_time).as_secs_f32();
        last_time = temp;

        // --- input ---
        {
            let the_scene = scenes.last_mut().unwrap();
            let result = the_scene.handle_input(&mut rl, &mut game_data);
            
            // switch scenes if needed
            match result {
                SceneSwitch::Push(new_scene) => {
                    scenes.push(new_scene);
                },
                SceneSwitch::Pop => {
                    scenes.pop();
                },
                SceneSwitch::Replace(new_scene) => {
                    scenes.pop();
                    scenes.push(new_scene);
                },
                SceneSwitch::Quit => keep_playing = false,
                _ => ()
            }
        }

        // --- update ---
        {
            let the_scene = scenes.last_mut().unwrap();
            let result = the_scene.update(delta, &mut game_data);
            
            match result {
                SceneSwitch::Push(new_scene) => {
                    scenes.push(new_scene);
                },
                SceneSwitch::Pop => {
                    scenes.pop();
                },
                SceneSwitch::Replace(new_scene) => {
                    scenes.pop();
                    scenes.push(new_scene);
                },
                SceneSwitch::Quit => keep_playing = false,
                _ => ()
            }
        }

        // --- draw ---
        let the_scene = scenes.last().unwrap();
        let mut d = rl.begin_drawing(&thread); 
        the_scene.draw(&mut d, &mut game_data);
    }
}