use super::IsCopy;

use graphics::*;
use piston::keyboard;
use piston::{
    Game,
    KeyPressArgs,
    KeyReleaseArgs,
    RenderArgs,
    UpdateArgs,
};

use player::Player;
use ball::Ball;

use constants::PLAYER_WIDTH;

pub struct App {
    player1: Player,
    player2: Player,
    ball: Ball,
}
impl IsCopy for App {}

pub struct RenderResources {
    pub gl: Gl,
}

impl App {
    pub fn new() -> App {
        App {
            player1: Player::new( -0.9 , -0.25 ),
            player2: Player::new( 0.9 - PLAYER_WIDTH, -0.25 ),
            ball: Ball::new(),
        }
    }
}

impl Game<RenderResources> for App {
    fn render( &mut self, resources: &mut RenderResources, _args: &mut RenderArgs ) {
        let c = &Context::new();
        let gl = &mut resources.gl;
        
        c.rgb(0.0, 0.0, 0.0).draw(gl);
        
        self.player1.render( c, gl );
        self.player2.render( c, gl );
        self.ball.render( c, gl );
    }

    fn update( &mut self, args: &mut UpdateArgs ) {
        self.player1.update( args.dt );
        self.player2.update( args.dt );
        self.ball.update( &self.player1, 
                                &self.player2 );
    }

    fn key_press( &mut self, args: &KeyPressArgs ) {
        match args.key {
            keyboard::W => self.player1.move_up(),
            keyboard::S => self.player1.move_down(),
            keyboard::Up => self.player2.move_up(),
            keyboard::Down => self.player2.move_down(),
            _ => ()
        }
    }

    fn key_release( &mut self, args: &KeyReleaseArgs ) {
        match args.key {
            keyboard::W => self.player1.stop(),
            keyboard::S => self.player1.stop(),
            keyboard::Up => self.player2.stop(),
            keyboard::Down => self.player2.stop(),
            _ => ()
        }
    }
}
