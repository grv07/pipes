use rand::{distributions::Uniform, prelude::*};

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

#[derive(Clone)]
struct Pipe {
    points: VecDeque<Point>,
    length: usize,
    border_hit: bool,
}

impl Pipe {
    fn new(points: VecDeque<Point>, length: usize) -> Self {
        Self {
            points,
            length,
            border_hit: false,
        }
    }

    fn pipe_hit_border(&mut self, size: Point, head: &Point) -> bool {
        if !self.border_hit {
            self.border_hit = size.0 <= head.0 || size.1 <= head.1;
        }

        self.border_hit
    }

    // fn print_pipe(&self) {
    //     for p in self.points {}
    // }
}

struct App {
    pipes: Vec<Pipe>,
}

impl App {
    fn new() -> Self {
        let mut s = Self { pipes: vec![] };

        for i in 1..3 {
            let mut p = VecDeque::new();
            p.push_front((i * 3, 0));
            let p = Pipe::new(p, (10) as usize);
            s.pipes.push(p);
        }

        s
        // let mut p1 = VecDeque::new();
        // p1.push_front((1, 0));
        // let p1 = Pipe::new(p1, 20);

        // let mut p2 = VecDeque::new();
        // p2.push_front((10, 0));
        // let p2 = Pipe::new(p2, 10);

        // let mut p3 = VecDeque::new();
        // p3.push_front((40, 0));
        // let p3 = Pipe::new(p3, 10);

        // let mut p4 = VecDeque::new();
        // p4.push_front((45, 0));
        // let p4 = Pipe::new(p4, 10);

        // let mut p5 = VecDeque::new();
        // p5.push_front((50, 0));
        // let p5 = Pipe::new(p5, 10);

        // let mut p6 = VecDeque::new();
        // p6.push_front((35, 0));
        // let p6 = Pipe::new(p6, 10);

        // Self {
        //     pipes: vec![
        //         p1.clone(),
        //         p2.clone(),
        //         p3.clone(),
        //         p4.clone(),
        //         p5.clone(),
        //         p6.clone(),
        //     ],
        // }
    }

    fn run(&mut self) -> io::Result<()> {
        let size = terminal::size()?;

        stdout().queue(terminal::Clear(terminal::ClearType::All))?;

        let mut rng = rand::thread_rng();
        let axis = Uniform::from(388..500);

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
                        q.queue(style::PrintStyledContent(">".dark_green()))?;
                    } else {
                        q.queue(style::PrintStyledContent("●".dark_green()))?;
                    }
                }

                if pipe.points.len() > pipe.length {
                    pipe.points.pop_back();
                }

                if let Some(head) = pipe.points.front() {
                    let mut head = head.clone();
                    if axis.sample(&mut rng) & 2 == 0 {
                        head.1 += 1;
                    } else {
                        head.0 += 1;
                    }

                    if !pipe.pipe_hit_border(size, &head) {
                        pipe.points.push_front(head);
                    } else {
                        pipe.points.pop_back();
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
