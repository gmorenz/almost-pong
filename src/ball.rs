use super::IsCopy;

use std::num::abs;

use graphics::*;

use player::Player;
use XYWH::XYWH;

static BALL_WIDTH: f64 = 0.1;
static BALL_SPEED: f64 = 0.01;

pub struct Ball {
    loc: XYWH,
    velocity: [f64, .. 2]
}
impl IsCopy for Ball {}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            loc: XYWH {
                x: -BALL_WIDTH / 2.0,
                y: -BALL_WIDTH / 2.0,
                w: BALL_WIDTH,
                h: BALL_WIDTH,
            },
            velocity: [-1.0, 0.0],
        }
    }

    pub fn start( &mut self ) {
        self.velocity = [1.0, 0.0]
    }

    pub fn render( &self, c: &Context, gl: &mut Gl ) {
        c.rect(self.loc.x,
               self.loc.y,
               self.loc.w,
               self.loc.h)
            .rgb(0.0, 1.0, 0.0).fill( gl );
    }

    pub fn move( &mut self ) { 
        self.loc.x += self.velocity[0] * BALL_SPEED;
        if self.loc.y > 1.0 - BALL_WIDTH || self.loc.y < -1.0{
            self.velocity[1] = self.velocity[1] * -1.0;
        }
        self.loc.y += self.velocity[1] * BALL_SPEED;
    }
    
    pub fn update( &mut self, player1: &Player, player2: &Player ) {
        self.move();
        if self.loc.is_touching( player1.loc ) {
            let d_centers = (self.loc.center_y() - player1.loc.center_y()) /
                player1.loc.h;
            self.velocity = [ 1.0 - abs(d_centers).sqrt(),
                              d_centers ]
        }
        if self.loc.is_touching( player2.loc ) {
            let d_centers = (self.loc.center_y() - player2.loc.center_y()) /
                player2.loc.h;
            self.velocity = [ -1.0 + abs(d_centers).sqrt(),
                               d_centers ]
        }
    }
}
