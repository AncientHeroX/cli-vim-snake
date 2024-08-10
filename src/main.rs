use std::{io::{stdout, Write}, ops::BitXor, thread::sleep, time::Duration, usize};


const WIDTH: usize = 100;
const HEIGHT: usize = 50;
const BACKGROUND_CHAR: char = ' ';

struct Screen {
    height: usize,
    width: usize,
    canvas: Vec<Vec<char>>
}
impl Screen {
    fn new (w: usize, h: usize) -> Screen {
        Screen { width: w, height: h, canvas: vec![vec![BACKGROUND_CHAR; w]; h]}
    }
    fn draw(&mut self, x: i32, y: i32) {
        if (x as usize) < self.width && (y as usize) < self.height {
            self.canvas[y as usize][x as usize] = '0';
        }
    }

    fn render(&mut self) {
        for r in &self.canvas {
            for c in r {
                print!("{}", c);
            }
            print!("\n");
            stdout().flush().unwrap();
        }
        print!("\x1B[{HEIGHT}A");
        self.canvas = vec![vec![BACKGROUND_CHAR; self.width]; self.height];

        sleep(Duration::from_millis(100));
        stdout().flush().unwrap();
    }
}

struct SnakeBody {
    m_x: i32,
    m_y: i32,
}
impl SnakeBody {
    fn new (x: i32, y: i32) -> SnakeBody {
        SnakeBody { m_x: x, m_y: y }
    }
}
struct Snake {
    body: Vec<SnakeBody>,
    xdir: i32,
    ydir: i32
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake{
        Snake { 
            body: vec![SnakeBody::new(x, y)],
            xdir: 1,
            ydir: 0
        }
    }
    fn add_tail(&mut self) {
        let head = &self.body[self.body.len() - 1];
        let n_head = SnakeBody::new(head.m_x + &self.xdir, head.m_y + self.ydir);

        self.body.push(n_head);
    }
    fn adjust(&mut self) {
        self.add_tail();

        self.body.remove(0);
    }

    fn draw(&mut self, s: &mut Screen) {
        self.adjust();
        for part in &self.body {
            s.draw(part.m_x, part.m_y);
        }
    }
}

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT);
    let mut snake = Snake::new(49, 24);
    snake.xdir = 1;
    snake.ydir = 0;
    
    snake.add_tail();
    snake.add_tail();
    snake.add_tail();

    loop {
        snake.xdir = snake.xdir ^ 1;
        snake.ydir = snake.ydir ^ 1;
        snake.draw(&mut screen);
        screen.render();
    }
}

