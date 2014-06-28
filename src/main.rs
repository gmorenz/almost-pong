#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate debug;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use app::{
    App,
    RenderResources
};

use piston::{
    ConcurrentGame,
    GameIteratorSettings,
    GameWindowSettings,
};

use sdl2_game_window::{
    ConcurrentWindowSDL2,
};

use opengl_graphics::Gl;

mod app;
mod ball;
mod constants;
mod player;
mod XYWH;

pub trait IsCopy: Copy {}

fn main() {
    let (game_window, render_window) = ConcurrentWindowSDL2::new(
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

    let render_resources = RenderResources{ gl: Gl::new() };

    app.run( game_window, render_window, iter_settings, render_resources );
}
