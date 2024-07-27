// use rand::{distributions::Uniform, prelude::*};

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

type Point = (u16, u16);

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

    fn pipe_hit_border(&mut self, size: Point, head: &Point) -> bool {
        size.0 <= head.0 || size.1 <= head.1
    }
}

struct App {
    pipes: Vec<Pipe>,
}

impl App {
    fn new() -> Self {
        let mut s = Self { pipes: vec![] };

        for i in 1..3 {
            let mut p = VecDeque::new();
            let start = (i * 3, 0);
            p.push_front((start.0, start.1));

            let p = Pipe::new(p, start, (10) as usize);
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
                        .queue(cursor::MoveTo(p.0, p.1))?
                        .queue(cursor::Hide)?;

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
                    head.1 += 1;

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
    let mut app = App::new();
    app.run()?;

    Ok(())
}
