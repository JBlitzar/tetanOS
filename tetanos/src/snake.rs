#![no_std]
#![no_main]


use core::panic::PanicInfo;
use crate::vga_buffer;
use crate::kb; 


pub struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    pub fn new(seed: u32) -> Self {
        XorShift32 { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }


    pub fn next_range(&mut self, min: usize, max: usize) -> usize {
        (self.next() as usize % (max - min)) + min
    }
}

struct Food {
    x: usize,
    y: usize,
}

impl Food {
    fn new(x: usize, y: usize) -> Self {
        Food { x, y }
    }
    fn new_prng(prng: &mut XorShift32) -> Self {
        let x = prng.next_range(0, vga_buffer::BUFFER_WIDTH);
        let y = prng.next_range(0, vga_buffer::BUFFER_HEIGHT);
        Food { x, y }
    }
    fn draw(&self, writer: &mut vga_buffer::Writer) {
        let RED = vga_buffer::ColorCode::new(vga_buffer::Color::Red, vga_buffer::Color::Black);
        writer.write_char_anywhere(self.y, self.x, b'*', RED);
    }
}


struct Snake {
    body: [(usize, usize); 256],
    body_length: usize,
    direction: (isize, isize),
}

impl Snake {
    fn new() -> Self {
        Snake {
            body: [(0, 0); 256],
            body_length: 1,
            direction: (1, 0),
        }
    }

    fn accept_direction_sc(&mut self, scancode: u8) {
        match scancode {
            0x11 => self.accept_direction((0, -1)), // up
            0x1f => self.accept_direction((0, 1)),  // down
            0x1e => self.accept_direction((-1, 0)), // left
            0x20 => self.accept_direction((1, 0)),  // right
            _ => {}
        }
    }

    fn accept_direction(&mut self, dir: (isize, isize)) {
        self.direction = dir;
    }

    fn step(&mut self, did_eat: bool) -> bool {
        
        
        for i in (1..self.body_length).rev() {
            self.body[i] = self.body[i - 1];
        }

        if did_eat && self.body_length < self.body.len() {
            self.body_length += 1;
        }
        let head = &mut self.body[0];
        let new_x = (head.0 as isize + self.direction.0);
        let new_y = (head.1 as isize + self.direction.1);
       
        
        if new_x < 0 {
            head.0 = vga_buffer::BUFFER_WIDTH - 1;
        } else if new_x >= vga_buffer::BUFFER_WIDTH as isize {
            head.0 = 0;
        } else {
            head.0 = new_x as usize;
        }

        if new_y < 0 {
            head.1 = vga_buffer::BUFFER_HEIGHT - 1;
        } else if new_y >= vga_buffer::BUFFER_HEIGHT as isize {
            head.1 = 0;
        } else {
            head.1 = new_y as usize;
        }

        let h = (head.0, head.1);

        let is_game_over = self.body[1..self.body_length].iter().any(|&(x, y)| x == h.0 && y == h.1);
        is_game_over
    }

    fn draw(&self, writer: &mut vga_buffer::Writer) {
        let GREEN = vga_buffer::ColorCode::new(vga_buffer::Color::Green, vga_buffer::Color::Black);
        let CYAN = vga_buffer::ColorCode::new(vga_buffer::Color::LightBlue, vga_buffer::Color::Black);
        for i in 0..self.body_length {
            let (x, y) = self.body[i];
            writer.write_char_anywhere(y, x, b'#', GREEN);
        }
        let (hx, hy) = self.body[0];
        writer.write_char_anywhere(hy, hx, b'@', CYAN);
    }
}



pub fn go(){

    let RED = vga_buffer::ColorCode::new(vga_buffer::Color::Red, vga_buffer::Color::Black);

    
    let mut WRITER = vga_buffer::_get_writer();
    WRITER.write_char_anywhere(10, 10, b'#', RED);

    let mut snake = Snake::new();

    let mut prng = XorShift32::new(0x12345678);



    let mut food = Food::new_prng(&mut prng);


    


    loop {
        if let Some(scancode) = kb::read_scancode() {

            snake.accept_direction_sc(scancode);
            
            
        }
        let head = &snake.body[0];
        let did_eat = head.0 == food.x && head.1 == food.y;
        let is_game_over = snake.step(did_eat);
        if (did_eat) {
            food = Food::new_prng(&mut prng);
        }
        WRITER.clear();
        snake.draw(&mut WRITER);
        food.draw(&mut WRITER);

        if is_game_over{
            break;
        }



        for _ in 0..500_000 { core::hint::spin_loop(); }

    }
}