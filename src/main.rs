use log::{debug, info};
use log4rs;

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand,
};
use std::{
    collections::VecDeque,
    io::{self, stdout, Write},
    thread,
    time::Duration,
};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: u16,
    y: u16,
    // hidden: bool,
}

// type Point = (u16, u16);

#[derive(Clone, Debug)]
struct Pipe {
    points: VecDeque<Point>,
    length: usize,
    start_point: Point,
    msg: String,
}

impl Pipe {
    fn new(points: VecDeque<Point>, start: Point, length: usize, msg: String) -> Self {
        Self {
            points,
            length,
            start_point: start,
            msg,
        }
    }

    fn pipe_hit_border(&mut self, size: (u16, u16), head: &Point) -> bool {
        size.0 <= head.x || size.1 <= head.y
    }
}

struct App {
    pipes: Vec<Pipe>,
    size: (u16, u16),
}

impl App {
    fn new(size: (u16, u16), msg: String) -> Self {
        info!("Start a new app ...");
        let mut s = Self {
            pipes: vec![],
            size,
        };
        info!("terminal size is: {:?}", &size);

        for i in 10..s.size.1 {
            info!("Add pipe number {:?}", i);

            let mut p = VecDeque::new();
            let start = Point {
                x: i * 3,
                y: 0,
                // hidden: false,
            };
            p.push_front(start);

            let p = Pipe::new(p, start, 30, msg.clone());

            debug!("Push new  pipe {i} {:?}", &p);

            s.pipes.push(p);
        }

        s
    }

    fn run(&mut self) -> io::Result<()> {
        stdout().queue(terminal::Clear(terminal::ClearType::All))?;

        loop {
            stdout().queue(terminal::Clear(terminal::ClearType::FromCursorUp))?;

            for pipe in self.pipes.iter_mut() {
                let msg = pipe.msg.split_whitespace().fold(String::new(), |acc, e| {
                    let e: String = e.chars().rev().collect();
                    format!("{acc} {e}")
                });

                // info!("pipes generated msg {}", msg);

                for (p, m) in pipe.points.iter().zip(msg.chars()) {
                    let mut stdout = stdout();
                    let q = stdout
                        .queue(cursor::MoveTo(p.x, p.y))?
                        .queue(cursor::Hide)?;
                    q.queue(style::PrintStyledContent(m.dark_green()))?;
                }

                if pipe.points.len() > pipe.length {
                    pipe.points.pop_back();
                }

                if let Some(head) = pipe.points.front() {
                    let mut head = head.clone();
                    head.y += 1;

                    if !pipe.pipe_hit_border(self.size, &head) {
                        pipe.points.push_front(head);
                    } else {
                        pipe.points.pop_back();

                        if pipe.points.is_empty() {
                            pipe.points.push_front(pipe.start_point);
                        }
                    }
                }
            }

            stdout().flush()?;
            thread::sleep(Duration::from_millis(200));
        }
    }
}

fn main() -> io::Result<()> {
    let msg = "This is my msg that will run".to_string();

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let size = terminal::size()?;
    let mut app = App::new(size, msg);
    app.run()?;

    Ok(())
}
