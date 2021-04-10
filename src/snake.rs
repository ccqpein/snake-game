use crate::frame::*;
use std::io::{Error, ErrorKind, Result};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    pub body: Vec<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    pub fn new(head: (i32, i32), dir: Direction, length: usize) -> Self {
        let a = match dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, 1),
            Direction::Right => (0, -1),
        };

        // make tail
        let b = (0..length)
            .map(|x| (head.0 + x as i32 * a.0, head.1 + x as i32 * a.1))
            .collect::<Vec<_>>();

        if !b.iter().all(|x| x.0 >= 0 && x.1 >= 0) {
            return Self { body: vec![], dir };
        }

        Self {
            body: b.iter().map(|x| (x.0, x.1)).collect::<Vec<_>>(),
            dir,
        }
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn head(&self) -> (i32, i32) {
        self.body.first().unwrap().clone()
    }

    pub fn add_tail(&mut self, d: (i32, i32)) {
        self.body.push(d);
    }

    pub fn show(&self, frame: &mut Frame) -> Result<()> {
        for b in &self.body {
            frame.write(b.0, b.1, String::from("◼"))?;
        }
        Ok(())
    }

    // return the tail
    pub fn move_one_step(&mut self, k: &Direction) -> Result<(i32, i32)> {
        self.dir = *k;
        let step = match self.dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let tail = self.body.pop().unwrap();
        let a = self.body.first().unwrap();
        let new_head = (a.0 as i32 + step.0, a.1 as i32 + step.1);

        if self.included(&(new_head.0 as usize, new_head.1 as usize)) {
            return Err(Error::new(ErrorKind::AlreadyExists, ""));
        }

        self.body.insert(0, new_head);

        Ok(tail)
    }

    pub fn included(&self, point: &(usize, usize)) -> bool {
        self.body.contains(&(point.0 as i32, point.1 as i32))
    }
}
