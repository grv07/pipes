use log::{debug, error, info, warn};
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
    hidden: bool,
}

// type Point = (u16, u16);

#[derive(Clone, Debug)]
struct Pipe {
    points: VecDeque<Point>,
    length: usize,
    start_point: Point,
}

impl Pipe {
    fn new(points: VecDeque<Point>, start: Point, length: usize) -> Self {
        Self {
            points,
            length,
            start_point: start,
        }
    }

    fn pipe_hit_border(&mut self, size: (u16, u16), head: &Point) -> bool {
        size.0 <= head.x || size.1 <= head.y
    }
}

struct App {
    pipes: Vec<Pipe>,
}

impl App {
    fn new() -> Self {
        info!("Start a new app ...");
        let mut s = Self { pipes: vec![] };

        for i in 1..3 {
            info!("Add pipe number {:?}", i);
            let mut p = VecDeque::new();
            let start = Point {
                x: i * 3,
                y: 0,
                hidden: false,
            };
            p.push_front(start);

            let p = Pipe::new(p, start, (10) as usize);
            debug!("Push new  pipe {i} {:?}", &p);
            s.pipes.push(p);
        }

        s
    }

    fn run(&mut self) -> io::Result<()> {
        let size = terminal::size()?;

        stdout().queue(terminal::Clear(terminal::ClearType::All))?;

        loop {
            thread::sleep(Duration::from_millis(50));
            stdout().queue(terminal::Clear(terminal::ClearType::All))?;

            for pipe in self.pipes.iter_mut() {
                for (i, p) in pipe.points.iter().enumerate() {
                    let mut stdout = stdout();
                    let q = stdout
                        .queue(cursor::MoveTo(p.x, p.y))?
                        .queue(cursor::Hide)?;

                    if p.hidden {
                        info!("hidden point");
                        continue;
                    }

                    if i == 0 {
                        q.queue(style::PrintStyledContent("●".dark_green()))?;
                    } else {
                        q.queue(style::PrintStyledContent("●".dark_green()))?;
                    }
                }

                if pipe.points.len() > pipe.length {
                    pipe.points.pop_back();
                }

                if let Some(head) = pipe.points.front() {
                    let mut head = head.clone();
                    head.y += 1;

                    if !pipe.pipe_hit_border(size, &head) {
                        pipe.points.push_front(head);
                    } else {
                        pipe.points.pop_back();

                        if pipe.points.is_empty() {
                            pipe.points.push_front(pipe.start_point);
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(200));
            stdout().flush()?;
        }
    }
}

fn main() -> io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let mut app = App::new();
    info!(">>>>> booting up");
    warn!(">>>>> booting up");
    app.run()?;

    Ok(())
}
