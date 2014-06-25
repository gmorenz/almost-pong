#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate debug;

use app::App;

use piston::{
    Game,
    GameWindowSDL2,
    GameIteratorSettings,
    GameWindowSettings,
};

mod app;
mod ball;
mod constants;
mod player;
mod XYWH;

pub trait IsCopy: Copy {}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    
    let mut app = App::new();

    app.run( &mut window, &iter_settings );
}
