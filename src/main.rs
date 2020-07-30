use std::env::args;
use std::io::{Read, Result};
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

fn main() -> Result<()> {
    let (row, col) = parse_argv();
    let mut a = Frame::new(row, col);
    let mut snake = Snake::new((5, 5), Direction::Right, 2); //:= TODO: should random?

    let mut food = a.random_point();
    a.write(food.0 as i32, food.1 as i32, String::from("x"))?;

    let mut stdin = async_stdin().bytes();
    let mut count = 0;
    let mut k = Direction::Right;
    snake.show(&mut a).unwrap();

    loop {
        sleep(Duration::from_millis(50));
        count += 1;
        let b = [stdin.next(), stdin.next(), stdin.next()];
        if let Some(Ok(b'q')) = b[0] {
            break;
        }

        k = if let Some(dd) = parse_key(&b) { dd } else { k };
        if count == 10 {
            count = 0; // cleant count
            match snake.move_one_step(&k) {
                Ok(tt) => {
                    if snake.head() != (food.0 as i32, food.1 as i32) {
                        a.write(tt.0, tt.1, String::from("."))?
                    } else {
                        snake.add_tail(tt);
                        food = a.random_point();
                        while snake.included(&food) {
                            //:= TODO: this part need optimise
                            food = a.random_point();
                        }
                        a.write(food.0 as i32, food.1 as i32, String::from("x"))?;
                    }
                }

                Err(_) => {
                    print!("Dead! Sucker\n\r");
                    break;
                }
            };
            match snake.show(&mut a) {
                Ok(_) => (),
                Err(_) => {
                    print!("Dead! Sucker\n\r");
                    break;
                }
            }
        }
    }
    a.quit();
    Ok(())
}
