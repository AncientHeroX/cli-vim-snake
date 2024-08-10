use std::{error::Error, io::{stdout, Write}, thread::sleep, time::Duration};

pub struct Screen {
    background_char: char,
    height: usize,
    width: usize,
    canvas: Vec<Vec<char>>
}
impl Screen {
    pub fn new (w: usize, h: usize, b_char: char) -> Screen {
        Screen { background_char: b_char, width: w, height: h, canvas: vec![vec![b_char; w]; h]}
    }
    pub fn draw(&mut self, x: i32, y: i32, c: char) -> Result<(), String> {
        if (x as usize) < self.width && (y as usize) < self.height {
            self.canvas[y as usize][x as usize] = c;
            Ok(())
        } else {
            Err("Out of bounds".to_string())
        }
    }
    pub fn render(&mut self) {
        for r in &self.canvas {
            for c in r {
                print!("{}", c);
            }
            print!("\n");
            stdout().flush().unwrap();
        }

        self.canvas = vec![vec![self.background_char; self.width]; self.height];
        sleep(Duration::from_millis(100));
    }
}
