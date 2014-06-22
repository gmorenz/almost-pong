#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate debug;

use app::App;

use piston::{
    AssetStore,
    Game,
    GameWindowSDL2,
    GameIteratorSettings,
    GameWindowSettings,
};

mod app;
mod ball;
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

    let mut asset_store = AssetStore::empty();

    app.run( &mut window, &mut asset_store, &iter_settings );
}
