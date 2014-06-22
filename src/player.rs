use super::IsCopy;

use graphics::*;
//use piston::Gl;

use XYWH::XYWH;

static SPEED_MULT: f64 = 1.0;

enum Dir {
    Down = 1,
    None = 0,
    Up = -1,
}

pub struct Player {
    pub loc: XYWH,
    dir: Dir,
}
impl IsCopy for Player {}

impl Player {
    pub fn new( x: f64, y: f64 ) -> Player {
        Player {
            loc: XYWH {
                x: x,
                y: y,
                w: 0.1,
                h: 0.5,
            },
            dir: None,
        }
    }

    fn translate_up( &mut self, d: f64 ) {
        self.loc.y  += d * SPEED_MULT;
        if self.loc.y + self.loc.h > 1.0 { 
            self.loc.y = 1.0 - self.loc.h
        }
    }

    fn translate_down( &mut self, d: f64 ) {
        self.loc.y -= d * SPEED_MULT;
        if self.loc.y  < -1.0 {
            self.loc.y = -1.0 ;
        }
    }

    pub fn move_up( &mut self ) {
        self.translate_up(0.01);
        self.dir = Up;
    }

    pub fn move_down( &mut self ) {
        self.translate_down(0.01);
        self.dir = Down;
    }

    pub fn stop( &mut self ) {
        self.dir = None;
    }

    pub fn render( &self, c: &Context, gl: &mut Gl ) {
        c.rect(self.loc.x, 
               self.loc.y,
               self.loc.w,
               self.loc.h)
            .rgb(0.0, 1.0, 0.0).fill( gl );
        
    }

    pub fn update( &mut self,  dt: f64 ) {
        match self.dir {
            Up => self.translate_up( dt ),
            Down => self.translate_down( dt ),
            None => (),
        }
    }
}
