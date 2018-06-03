extern crate ncurses;
extern crate rand;

use std::char;
use ncurses::*;


use rand::Rng;
pub struct Display {
    pub max_x: i32,
    pub max_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub food_pos_x: i32,
    pub food_pos_y: i32,
    pub direction: Direction,
    height: i32,
    width: i32,
    snake_window: WINDOW,
    food_window: WINDOW
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nil
}

impl Display {
    pub fn new() -> Display {
        // start curses mode
        initscr();

        // line buffering disabled
        raw();

        // enable F keys
        keypad(stdscr(), true);

        noecho();

        // invisible cursor
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        // get the screen bounds
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let height = 2;
        let width = 3;

        // calculating for a center placement of the window
        let pos_y = (max_y - height) / 2;
        let pos_x = (max_x - width) / 2;

        printw("Press q to exit");
        refresh();

        let snake_window = create_window(height, width, pos_x, pos_y);
        let food_window = create_window(1, 1, pos_x, pos_y);

        Display {
            max_x: max_x,
            max_y: max_y,
            pos_x: pos_x,
            pos_y: pos_y,
            food_pos_x: rand::thread_rng().gen_range(0, max_x),
            food_pos_y: rand::thread_rng().gen_range(0, max_y),
            height: height,
            width: width,
            direction: Direction::Nil,
            snake_window: snake_window,
            food_window: food_window
        }

    }

    pub fn border_collision_detected(&self) -> bool {
        self.pos_x > self.max_x
            || self.pos_y > self.max_y
            || self.pos_x < 1
            || self.pos_y < 1
    }

    pub fn food_collision_detected(&self) -> bool {
        self.pos_x == self.food_pos_x
            && self.pos_y == self.food_pos_y
    }

    pub fn update_stats(&self, score: u32){
        let window = create_window(30, 25, self.max_x - 25, self.max_y - 15);
        mvwprintw(window, 1, 2, &format!("food x: {:?}", self.food_pos_x));
        mvwprintw(window, 2, 2, &format!("food y: {:?}", self.food_pos_y));
        mvwprintw(window, 3, 2, &format!("points: {:?}", score));
        mvwprintw(window, 4, 2, &format!("x: {:?}", self.pos_x));
        mvwprintw(window, 5, 2, &format!("y: {:?}", self.pos_y));

        wrefresh(window);
    }

    pub fn render(&mut self, score: u32) {

        let mut score = score;
        destroy_window(self.food_window);
        self.food_window = create_window(2, 2, self.food_pos_x, self.food_pos_y);

        while !self.border_collision_detected() {	
            self.update_stats(score);

            if self.food_collision_detected(){
                destroy_window(self.food_window);
                score += 1;
                self.food_window = create_window(1, 1, 
                                                 rand::thread_rng().gen_range(0, self.max_x - 5),
                                                 rand::thread_rng().gen_range(0, self.max_y - 5));
                refresh();
                
            }

            match getch() {
                0x71 => { break; },
                KEY_LEFT => { 
                    if self.direction != Direction::Left {
                    self.direction = Direction::Left; 
                    }
                },
                KEY_RIGHT => { self.direction = Direction::Right; },
                KEY_UP => { self.direction = Direction::Up; },
                KEY_DOWN => { self.direction = Direction::Down; },
                _ => { }
            }

            match self.direction {
                Direction::Up => {
                    self.pos_y -= 1; 
                },
                Direction::Down => {
                    self.pos_y += 1; 
                },
                Direction::Left => {
                    self.pos_x -= 1; 
                },
                Direction::Right => {
                    self.pos_x += 1; 
                },
                _ => {}
            }

            destroy_window(self.snake_window);
            self.snake_window = create_window(self.height, self.width, self.pos_x, self.pos_y);
        }

        // end curses mode
        endwin();
    }
}

fn create_window(height: i32, width: i32, pos_x: i32, pos_y: i32) -> WINDOW {	
    let window = newwin(height, width, pos_y, pos_x);

    timeout(300);

    // 0, 0 gives default characters for the vertical and horizontal lines
    box_(window, 0, 0);

    // display box
    wrefresh(window);
    window
}

fn destroy_window(window: WINDOW) {	
    /* box(local_win, ' ', ' '); : This won't produce the desired
     * result of erasing the window. It will leave it's four corners 
     * and so an ugly remnant of window. 
     */
    let ch = ' ' as chtype;
    wborder(window, ch, ch, ch, ch, ch, ch, ch, ch);
    /* The parameters taken are 
     * 1. win: the window on which to operate
     * 2. ls: character to be used for the left side of the window 
     * 3. rs: character to be used for the right side of the window 
     * 4. ts: character to be used for the top side of the window 
     * 5. bs: character to be used for the bottom side of the window 
     * 6. tl: character to be used for the top left corner of the window 
     * 7. tr: character to be used for the top right corner of the window 
     * 8. bl: character to be used for the bottom left corner of the window 
     * 9. br: character to be used for the bottom right corner of the window
     */
    wrefresh(window);
    delwin(window);
}
