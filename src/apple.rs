use std::error::Error;

use rand::prelude;
use super::snake::SnakeBody;
use super::screen::Screen;

pub struct Apple {
    bounds: (usize, usize),
    pub m_x: i32,
    pub m_y: i32
}

impl Apple {
    pub fn new(bound_height: usize, bound_width: usize) -> Apple {
        Apple { 
            bounds: (bound_height, bound_width),
            m_x: { rand::random::<usize>() % bound_width} as i32,
            m_y: { rand::random::<usize>() % bound_height} as i32
        }
    }
    pub fn re_pos(&mut self) {
        let (bound_height, bound_width) = self.bounds;
        self.m_x = { rand::random::<usize>() % bound_width } as i32;
        self.m_y = { rand::random::<usize>() % bound_height } as i32
    }
    pub fn check_collision(&self, snake_head: &SnakeBody) -> bool {
        if self.m_y == snake_head.m_y && self.m_x == snake_head.m_x {
            true
        } else {
            false
        }
    }
    pub fn draw(&self, screen: &mut Screen) -> Result<(), String> {
        match screen.draw(self.m_x, self.m_y, 'a') {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Apple out of bounds: {e}")),
        }
    }
}
