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
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[derive(PartialEq, Clone, Copy)]
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
            direction: Direction::Right,
        }
    }
}
#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
    size: usize,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, start_idx: usize) -> World {
        let snake = Snake::new(start_idx, 3);
        let size = width * width;

        World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
            points: 0,
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;
        // generate reward outside of snake body
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        Some(reward_cell)
    }

    pub fn restart_game(&mut self, start_idx: usize) {
        self.snake = Snake::new(start_idx, 3);
        self.points = 0;
        self.status = Some(GameStatus::Played);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn points(&self) -> usize {
        self.points
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

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No status"),
        }
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

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
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

                for i in 1..self.snake_length() {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                if self.reward_cell == Some(self.snake_head()) {
                    if self.snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body)
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won);
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
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
