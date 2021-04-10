use crate::snake::*;
use rand::*;
use std::collections::HashSet;
use std::io::{repeat, stdout, Error, ErrorKind, Read, Result, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

use druid::{
    widget::{Align, Flex, Label, Padding},
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, RenderContext, UpdateCtx,
};
use druid::{AppLauncher, Data, PlatformError, Rect, Size, Widget, WindowDesc};

pub struct Frame {
    row: usize,
    col: usize,
    points: RawTerminal<Stdout>,
    rng: rand::rngs::ThreadRng,
    set: HashSet<(usize, usize)>,
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
            set: (0..row)
                .map(|r| (0..col).map(move |c| (r, c)))
                .flatten()
                .collect::<HashSet<(usize, usize)>>(),
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

    pub fn random_point(&mut self, s: &Snake) -> Result<(usize, usize)> {
        let a = s
            .body
            .iter()
            .cloned()
            .map(|a| (a.0 as usize, a.1 as usize))
            .collect::<HashSet<(usize, usize)>>();
        let mut pool = self.set.difference(&a);

        let ind = self.rng.gen_range(0, self.row * self.col - s.body.len());
        match pool.nth(ind) {
            Some((a, b)) => return Ok((*a, *b)),
            None => Err(Error::new(ErrorKind::Other, "Panic")),
        }
    }

    pub fn quit(&mut self) {
        write!(self.points, "{}", termion::cursor::Show).unwrap();
    }

    pub fn show(&mut self, snake: &Snake) -> Result<()> {
        for b in &snake.body {
            self.write(b.0, b.1, String::from("◼"))?;
        }
        Ok(())
    }
}

//impl Widget<Snake> for Frame {}
