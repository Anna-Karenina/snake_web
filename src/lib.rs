use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub struct SnakeCell(usize);

#[wasm_bindgen]
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(span_index: usize, size: usize) -> Snake {
        let mut body = vec![];
        for i in 0..size {
            body.push(SnakeCell(span_index - i));
        }
        Snake {
            body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, start_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(start_idx, 2),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }
    // cannot return a ref to JS cuz of borroving rules
    // pub fn snake_cell(&self) -> &Vec<SnakeCell> {
    //     &self.snake.body
    // }
    // *const is raw pointer
    // borrowing rules does not apply to it

    pub fn snake_cell(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        let next_cell = self.generate_next_snake_cell();
        self.snake.body[0] = next_cell;
    }

    fn generate_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head();
        let row = snake_idx / self.width;

        return match self.snake.direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
        };
    }
}
#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
    size: usize,
}
