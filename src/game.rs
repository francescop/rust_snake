extern crate snake;
extern crate display;

use snake::*;
use display::*;

pub struct Game {
    pub display: Display,
    pub snake: Snake
}

impl Game {
    pub fn new() -> Game {
        let mut snake = Snake::new();
        let mut display = Display::new();

        Game { snake: snake, display: display }
    }

    pub fn init(&mut self){
        self.display.render(self.snake.length);

        println!("game over, total score: {}", self.snake.length);
    }
}

#[test]
fn create_game(){
    let mut game = Game::new();
    assert_eq!(game.snake.length, 1);

    game.snake.eat();
    assert_eq!(game.snake.length, 2);
}
