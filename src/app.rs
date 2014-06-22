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

struct GameState {
    player1: Player,
    player2: Player,
    ball: Ball,
}
impl IsCopy for GameState {}

struct Resources {
    gl: Gl,
}

pub struct App {
    resources: Resources,
    state: GameState,
}

impl App {
    pub fn new() -> App {
        App {
            state: GameState {
                player1: Player::new( -0.9 , -0.25 ),
                player2: Player::new( 0.8, -0.25 ),
                ball: Ball::new(),
            },
            resources: Resources {
                gl: Gl::new(),
            },
        }
    }
}

impl Game for App {
    fn render( &mut self, _args: &mut RenderArgs ) {
        let c = &Context::new();
        let gl = &mut self.resources.gl;
        
        c.rgb(0.0, 0.0, 0.0).clear(gl);
        
        self.state.player1.render( c, gl );
        self.state.player2.render( c, gl );
        self.state.ball.render( c, gl );
    }

    fn update( &mut self, args: &mut UpdateArgs ) {
        self.state.player1.update( args.dt );
        self.state.player2.update( args.dt );
        self.state.ball.update( &self.state.player1, 
                                &self.state.player2 );
    }

    fn key_press( &mut self, args: &KeyPressArgs ) {
        match args.key {
            keyboard::W => self.state.player1.move_up(),
            keyboard::S => self.state.player1.move_down(),
            keyboard::Up => self.state.player2.move_up(),
            keyboard::Down => self.state.player2.move_down(),
            _ => ()
        }
    }

    fn key_release( &mut self, args: &KeyReleaseArgs ) {
        match args.key {
            keyboard::W => self.state.player1.stop(),
            keyboard::S => self.state.player1.stop(),
            keyboard::Up => self.state.player2.stop(),
            keyboard::Down => self.state.player2.stop(),
            _ => ()
        }
    }
}
