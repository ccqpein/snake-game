use rand::*;
use std::io::{repeat, stdout, Error, ErrorKind, Read, Result, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Frame {
    row: usize,
    col: usize,
    points: RawTerminal<Stdout>,
    rng: rand::rngs::ThreadRng,
}

impl Frame {
    pub fn new(row: usize, col: usize) -> Self {
        let mut std_out = stdout().into_raw_mode().unwrap();
        let mut a = vec![0; col];
        repeat(b'.').read_exact(&mut a).unwrap();
        a.push(b'\n');
        a.push(b'\r');

        write!(
            std_out,
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            "'q' to quit",
            termion::cursor::Goto(1, 2),
            termion::cursor::Hide
        )
        .unwrap();

        for _ in 0..row {
            std_out.write_all(&a).unwrap();
            std_out.flush().unwrap();
        }

        Self {
            row,
            col,
            points: std_out,
            rng: rand::thread_rng(),
        }
    }

    pub fn write(&mut self, row: i32, col: i32, s: String) -> Result<()> {
        if row < 0 || col < 0 {
            return Err(Error::new(ErrorKind::Other, "beyond boundary"));
        }

        if row as usize >= self.row || col as usize >= self.col {
            return Err(Error::new(ErrorKind::Other, "beyond boundary"));
        }

        if col == 0 {
            write!(
                self.points,
                "{}",
                format_args!("\x1b[s\x1b[{}A\r{}\x1b[u", self.row - row as usize, s)
            )
            .unwrap()
        } else {
            write!(
                self.points,
                "{}",
                format_args!(
                    "\x1b[s\x1b[{}A\r\x1b[{}C{}\x1b[u",
                    self.row - row as usize,
                    col,
                    s
                )
            )
            .unwrap()
        };
        self.points.flush()
    }

    pub fn random_point(&mut self) -> (usize, usize) {
        (
            self.rng.gen_range(0, self.row),
            self.rng.gen_range(0, self.col),
        )
    }

    pub fn quit(&mut self) {
        write!(self.points, "{}", termion::cursor::Show).unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    body: Vec<(i32, i32)>,
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

    pub fn head(&self) -> (i32, i32) {
        self.body.first().unwrap().clone()
    }

    pub fn add_tail(&mut self, d: (i32, i32)) {
        self.body.push(d);
    }

    pub fn show(&mut self, frame: &mut Frame) -> Result<()> {
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
