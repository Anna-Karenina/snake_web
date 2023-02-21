use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone, Copy)]
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
pub struct World {
    width: usize,
    snake: Snake,
    size: usize,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, start_idx: usize) -> World {
        let size = width * width;
        let reward_cell = random(size);
        let start_snake_length = 3;

        World {
            width,
            size,
            snake: Snake::new(start_idx, start_snake_length),
            next_cell: None,
            reward_cell,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
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
        let temp = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
            }
        }

        let len = self.snake_length();

        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
    }

    // operation % and / to much expensive so refactor
    // fn generate_next_snake_cell(&self) -> SnakeCell {
    //     let snake_idx = self.snake_head();
    //     let row = snake_idx / self.width;

    //     return match self.snake.direction {
    //         Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
    //         Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
    //         Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
    //         Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
    //     };
    // }
    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let threshold = row * self.width;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let threshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
}
