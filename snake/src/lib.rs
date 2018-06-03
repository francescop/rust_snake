pub struct Snake {
    pub length: u32
}

impl Snake {
    pub fn new() -> Snake {
        Snake { length: 1 }
    }

    pub fn eat(&mut self) {
        self.length += 1;
    }
}

#[test]
fn create_snake() {
    let snake = Snake::new();
    assert_eq!(snake.length, 1);
}

#[test]
fn snake_eat_food() {
    let mut snake = Snake::new();
    snake.eat();
    assert_eq!(snake.length, 2);
}
