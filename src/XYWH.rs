use super::IsCopy;

pub struct XYWH {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl IsCopy for XYWH {}

impl XYWH {
    pub fn top( &self ) -> f64 { 
        self.y
    }
    pub fn bottom( &self ) -> f64 { 
        self.y + self.h
    }
    pub fn left( &self ) -> f64 { 
        self.x 
    }
    pub fn right( &self ) -> f64 { 
        self.x + self.w 
    }
    pub fn center_y( &self ) -> f64 {
        self.y + (self.h / 2.0)
    }

    pub fn is_touching( &self, other: XYWH ) -> bool {
        return self.top() < other.bottom() &&
            other.top() < self.bottom() &&
            self.left() < other.right() &&
            other.left() < self.right()
    }
}
