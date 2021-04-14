use crate::snake::*;
use rand::*;
use std::io::{repeat, stdout, Error, ErrorKind, Read, Result, Stdout, Write};
use std::time::Duration;
use std::{cell::RefCell, collections::HashSet};
use termion::raw::{IntoRawMode, RawTerminal};

use druid::{
    widget::{Align, Flex, Label, Padding},
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, RenderContext, UpdateCtx,
};
use druid::{AppLauncher, Data, PlatformError, Rect, Size, Widget, WindowDesc};
use std::rc::Rc;

#[derive(Clone, Data)]
pub struct Status {
    pub snake: Rc<RefCell<Snake>>,
    pub food: (usize, usize),

    pub speed_defer: f32,

    pub snake_last_len: usize,

    pub win: bool,
    pub lose: bool,
}

pub struct Frame {
    row: usize,
    col: usize,
    points: Option<RawTerminal<Stdout>>,
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
            points: Some(std_out),
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
                self.points.as_mut().unwrap(),
                "{}",
                format_args!("\x1b[s\x1b[{}A\r{}\x1b[u", self.row - row as usize, s)
            )
            .unwrap()
        } else {
            write!(
                self.points.as_mut().unwrap(),
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
        self.points.as_mut().unwrap().flush()
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
        write!(self.points.as_mut().unwrap(), "{}", termion::cursor::Show).unwrap();
    }

    pub fn show(&mut self, snake: &Snake) -> Result<()> {
        for b in &snake.body {
            self.write(b.0, b.1, String::from("◼"))?;
        }
        Ok(())
    }

    pub fn make_frame_gui(row: usize, col: usize) -> Flex<Status> {
        Flex::column().with_flex_child(
            Flex::row().with_flex_child(
                Frame {
                    row,
                    col,
                    points: None,
                    rng: rand::thread_rng(),
                    set: (0..row)
                        .map(|r| (0..col).map(move |c| (r, c)))
                        .flatten()
                        .collect::<HashSet<(usize, usize)>>(),
                },
                1.0,
            ),
            1.0,
        )
    }
}

impl Widget<Status> for Frame {
    /// need this function for handling the arrow keys.
    fn event(&mut self, ctx: &mut EventCtx<'_, '_>, event: &Event, data: &mut Status, _env: &Env) {
        //:= more event, like keys,...
        //:= and quit
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                // next time update
                ctx.request_timer(Duration::from_millis(50).mul_f32(data.speed_defer));
            }
            Event::Timer(_) => {
                let tail = data.snake.borrow_mut().move_one_step(&Direction::Right);
                match tail {
                    Ok(tt) => {
                        if data.snake.borrow().head() == (data.food.0 as i32, data.food.1 as i32) {
                            data.snake.borrow_mut().add_tail(tt);
                            if data.snake.borrow().len() == self.row * self.col {
                                //:= TODO: alart you win
                                //:= end
                            }
                            data.food = self.random_point(&data.snake.borrow()).unwrap();

                            //:= need change speed
                            ctx.request_timer(Duration::from_millis(50).mul_f32(data.speed_defer));
                        } else {
                            ctx.request_timer(Duration::from_millis(50).mul_f32(data.speed_defer));
                        }

                        ctx.request_paint();
                    }
                    Err(_) => {
                        //:= TODO
                    }
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx<'_, '_>,
        _event: &LifeCycle,
        _data: &Status,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx<'_, '_>,
        _old_data: &Status,
        _data: &Status,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx<'_, '_>,
        _bc: &BoxConstraints,
        _data: &Status,
        _env: &Env,
    ) -> Size {
        let col = self.col as f64 * 10.;
        let row = self.row as f64 * 10.;
        Size::new(row, col)
    }

    //:= can optimize this part like only update the change part
    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &Status, _env: &Env) {
        if data.win {}
        let cell_size = Size {
            width: 10.,
            height: 10.,
        };

        for row in 0..self.row {
            for col in 0..self.col {
                let point = Point {
                    x: 10. * row as f64,
                    y: 10. * col as f64,
                };

                if data.food == (row, col) {
                    let rect = Rect::from_origin_size(point, cell_size);
                    ctx.fill(rect, &Color::rgb8(252, 0, 0))
                } else {
                    //:= what if I dont paint this part
                    let rect = Rect::from_origin_size(point, cell_size);
                    ctx.fill(rect, &Color::rgb8(252, 252, 252))
                }
            }
        }

        for b in &data.snake.borrow().body {
            let point = Point {
                x: 10. * b.0 as f64,
                y: 10. * b.1 as f64,
            };
            let rect = Rect::from_origin_size(point, cell_size);
            ctx.fill(rect, &Color::rgb8(0, 0, 0))
        }
    }
}
