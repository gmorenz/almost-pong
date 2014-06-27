#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate debug;

use app::{
    App,
    RenderResources
};

use piston::{
    Game,
    GameWindowSDL2,
    GameIteratorSettings,
    GameWindowSettings,
};

use graphics::Gl;

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
    
    let app = App::new();

    let mut render_resources = RenderResources{ gl: Gl::new() };

    app.run( &mut window, iter_settings, render_resources );
}
