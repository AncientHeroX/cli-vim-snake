use super::screen::Screen;

pub struct SnakeBody {
    pub m_x: i32,
    pub m_y: i32
}
impl SnakeBody {
    pub fn new (x: i32, y: i32) -> SnakeBody {
        SnakeBody { m_x: x, m_y: y }
    }
}
pub struct Snake {
    body: Vec<SnakeBody>,
    xdir: i32,
    ydir: i32
}

impl Snake {
    pub fn new(x: i32, y: i32, size: i32) -> Snake{
        let mut ret_snake = Snake { 
            body: vec![SnakeBody::new(x, y)],
            xdir: 1,
            ydir: 0
        };
        for _ in 0..size {
            ret_snake.add_tail();
        };
        ret_snake
    }
    pub fn add_tail(&mut self) {
        let head = &self.body[self.body.len() - 1];
        let n_head = SnakeBody::new(head.m_x + &self.xdir, head.m_y + self.ydir);

        self.body.push(n_head);
    }
    pub fn change_dir (&mut self, new_x: i32, new_y: i32) {
        if (self.xdir != new_x * -1) || 
           (self.ydir != new_y * -1) {
            self.xdir = new_x;
            self.ydir = new_y;
        }
    }
    pub fn adjust(&mut self) -> Result<(), String> {
        self.add_tail();
        self.body.remove(0);
        self.check_collision()
    }
    fn check_collision(&self) -> Result<(), String> {
        let head = &self.get_head();
        for (i, part) in self.body.iter().enumerate() {
            if i == self.body.len() - 1 {
                continue;
            }
            if head.m_x == part.m_x && head.m_y == part.m_y {
                return Err("Touched yourself".to_string());
            }
        }
        Ok(())
    }
    pub fn get_head<'a>(&'a self) -> &'a SnakeBody {
        &self.body[self.body.len() - 1]
    }
    pub fn draw(&mut self, s: &mut Screen) -> Result<(), String>{
        match self.adjust() {
            Err(e) => {
                return Err(format!("{e}"))
            },
            _ => {}
        };

        for part in &self.body {
            match s.draw(part.m_x, part.m_y, '0') {
                Ok(_) => continue,
                Err(e) => return Err(format!("Snake part err: {e}"))
            }
        }
        Ok(())
    }
}
