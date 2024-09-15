
use std::ops::Add;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum GameState { Playing, Paused, Over }
pub enum PlayerDirection { Up, Down, Right, Left }

#[derive(Copy, Clone, Debug)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
    }
}

#[derive(Copy, Clone)]
pub struct GridSize{pub width: i32, pub height: i32}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
    pub grid_size: GridSize,
}

impl GameContext {
    pub fn new(grid_size: GridSize) -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3, 3),
            grid_size: grid_size
        }
    }

    pub fn next_tick(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }

        let next_head_position = self.get_next_head_position();
        self.check_collision(next_head_position);
        self.advance(next_head_position);
    }

    pub fn move_up(&mut self) {
        if let PlayerDirection::Down = self.player_direction {
            return
        }
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        if let PlayerDirection::Up = self.player_direction {
            return
        }
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        if let PlayerDirection::Left = self.player_direction {
            return
        }
        self.player_direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        if let PlayerDirection::Right = self.player_direction {
            return
        }
        self.player_direction = PlayerDirection::Left;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            _ => self.state
        }
    }

    fn get_next_head_position(&mut self) -> Point {
        let head_position = self.player_position.first().unwrap();
        return match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };
    }

    fn add_head(&mut self, next_head_position: Point) {
        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }

    fn remove_tail(&mut self) {
        self.player_position.pop();
    }

    fn advance(&mut self, next_head_position: Point) {
        self.remove_tail();
        self.add_head(next_head_position);
    }

    fn generate_food(&mut self) {
        let x = rand::thread_rng().gen_range(0..self.grid_size.width);
        let y = rand::thread_rng().gen_range(0..self.grid_size.height);
        self.food = Point(x.try_into().unwrap(), y.try_into().unwrap())
    }

    fn check_collision(&mut self, next_head_position: Point) {
        if next_head_position.0 == -1 || next_head_position.0 == self.grid_size.width || next_head_position.1 == -1 || next_head_position.1 == self.grid_size.height {
            self.state = GameState::Over
        }

        for point in &self.player_position {
            if next_head_position == *point {
                self.state = GameState::Over
            }
        }

        if next_head_position == self.food {
            self.add_head(next_head_position);
            self.generate_food()
        }
    }
}
