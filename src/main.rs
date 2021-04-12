use druid::{AppLauncher, Data, PlatformError, Rect, Size, Widget, WindowDesc};
use std::io::{Read, Result};
use std::rc::Rc;
use std::{cell::RefCell, env::args};
use std::{thread::sleep, time::Duration};
use termion::async_stdin;
use tiny_terminal_snake::*;

fn parse_key(buff: &[Option<Result<u8>>; 3]) -> Option<Direction> {
    if buff
        .iter()
        .all(|x| x.is_some() && x.as_ref().unwrap().is_ok())
        && buff[0].as_ref().unwrap().as_ref().unwrap() == &b'\x1B'
        && buff[1].as_ref().unwrap().as_ref().unwrap() == &b'['
    {
        match buff[2] {
            Some(Ok(b'D')) => Some(Direction::Left),
            Some(Ok(b'C')) => Some(Direction::Right),
            Some(Ok(b'A')) => Some(Direction::Up),
            Some(Ok(b'B')) => Some(Direction::Down),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_argv() -> (usize, usize) {
    let mut argv = args();
    let _ = argv.next();
    match (argv.next(), argv.next()) {
        (Some(a), Some(b)) => (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()),
        _ => (10, 20),
    }
}

struct Speed {
    /// length of snake
    last_len: usize,
    current: usize,
    /// speed level ~ snake length / last snake length
    multiples: Vec<f32>,
}

impl Speed {
    fn new(s: &Snake) -> Self {
        Self {
            last_len: s.len(),
            current: 10,
            multiples: vec![4 as f32, 2 as f32, 1.6, 1.4, 1.2, 1.1, 1.1],
        }
    }

    fn adjust(&mut self, s: &Snake) {
        match self.multiples.first() {
            Some(a) => {
                if s.len() as f32 / self.last_len as f32 >= *a {
                    self.current -= 1; // the smaller the faster
                    self.multiples.drain(..1); // next level
                    self.last_len = s.len(); // update length
                };
            }

            None => (),
        }
    }
}

impl PartialEq<usize> for Speed {
    fn eq(&self, other: &usize) -> bool {
        self.current == *other
    }
}

fn main() -> Result<()> {
    let (row, col) = parse_argv();

    ////
    ////
    ////

    let snake = Rc::new(RefCell::new(Snake::new((1, 1), Direction::Right, 2)));
    AppLauncher::with_window(WindowDesc::new(Frame::make_frame_gui(row, col)))
        .launch(Status {
            snake,
            food: (1, 2),
            speed_defer: 50.,
            snake_last_len: 3,
        })
        .unwrap();

    //////////
    //////////
    //////////
    /*let mut a = Frame::new(row, col);
    let mut snake = Snake::new((1, 1), Direction::Right, 2);

    let mut food = a.random_point(&snake).unwrap();
    a.write(food.0 as i32, food.1 as i32, String::from("x"))?;

    let mut stdin = async_stdin().bytes();
    let mut count: usize = 0;
    let mut spd = Speed::new(&snake);

    let mut k = Direction::Right;
    //snake.show(&mut a).unwrap();
    a.show(&snake);

    loop {
        sleep(Duration::from_millis(50));
        count += 1;
        let b = [stdin.next(), stdin.next(), stdin.next()];
        if let Some(Ok(b'q')) = b[0] {
            break;
        }

        k = if let Some(dd) = parse_key(&b) { dd } else { k }; // update input

        if spd == count {
            count = 0; // clean count
            match snake.move_one_step(&k) {
                Ok(tt) => {
                    if snake.head() != (food.0 as i32, food.1 as i32) {
                        a.write(tt.0, tt.1, String::from("."))?
                    } else {
                        snake.add_tail(tt);
                        if snake.len() == row * col {
                            print!("You win!");
                            return Ok(());
                        }

                        food = a.random_point(&snake).unwrap();
                        a.write(food.0 as i32, food.1 as i32, String::from("x"))?;
                    }
                }

                Err(_) => {
                    print!("Dead! Sucker\n\r");
                    break;
                }
            };
            //match snake.show(&mut a) {
            match a.show(&snake) {
                Ok(_) => (),
                Err(_) => {
                    print!("Dead! Sucker\n\r");
                    break;
                }
            }
            spd.adjust(&snake);
        }
    }
    a.quit();*/
    Ok(())
}
