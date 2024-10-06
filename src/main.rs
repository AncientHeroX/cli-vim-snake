mod apple;
mod screen;
mod snake;

use crossterm::{
    cursor::MoveUp,
    event::{poll, read, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    ExecutableCommand,
};
use screen::Screen;
use snake::Snake;
use std::{io::stdout, time::Duration, usize};

const WIDTH: usize = 100;
const HEIGHT: usize = 50;
const BACKGROUND_CHAR: char = ' ';

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT, BACKGROUND_CHAR);
    let mut snake = Snake::new(49, 24, 8);
    let mut apple = apple::Apple::new(HEIGHT, WIDTH);
    let mut points = 0;
    let clear = terminal::Clear;
    snake.change_dir(1, 0);

    loop {
        println!("points: {}", points);
        match snake.draw(&mut screen) {
            Err(e) => {
                println!("---- You lose!\n{e}");
                break;
            }
            _ => {}
        }
        match apple.draw(&mut screen) {
            Err(e) => {
                println!("{e}");
                break;
            }
            _ => {}
        }

        if apple.check_collision(snake.get_head()) {
            apple.re_pos();
            snake.add_tail();
            points += 1;
        };
        screen.render();

        stdout().execute(MoveUp((HEIGHT as u16) + 1)).unwrap();

        enable_raw_mode().unwrap();
        if poll(Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char(e) => match e {
                        'j' => snake.change_dir(0, 1),
                        'k' => snake.change_dir(0, -1),
                        'h' => snake.change_dir(-1, 0),
                        'l' => snake.change_dir(1, 0),
                        _ => (),
                    },
                    KeyCode::Esc => {
                        disable_raw_mode().unwrap();
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
        stdout().execute(clear(ClearType::FromCursorDown)).unwrap();
    }
    disable_raw_mode().unwrap();
}
