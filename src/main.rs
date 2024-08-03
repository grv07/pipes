use log::{debug, info};

use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode},
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
}

#[derive(Clone, Debug)]
struct Message {
    msg_vd: VecDeque<char>,
    lp: usize,
}

#[derive(Clone, Debug)]
struct Pipe {
    msg: Message,
    head: Point,
}

impl Pipe {
    fn new(head: Point, msg: String) -> Self {
        let msg_vd = VecDeque::from_iter(msg.chars());

        Self {
            head,
            msg: Message { msg_vd, lp: 0 },
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

        for i in 1..size.0 / 3 {
            info!("Add pipe number {:?}", i);

            let start = Point { x: i * 3, y: 0 };

            let p = Pipe::new(start, msg.clone());

            debug!("Push new  pipe {i} {:?}", &p);

            s.pipes.push(p);
        }

        s
    }

    fn run(&mut self) -> io::Result<()> {
        stdout().queue(terminal::Clear(terminal::ClearType::All))?;

        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == KeyCode::Esc
                        || (key_event.code == KeyCode::Char('c')
                            && key_event.modifiers == KeyModifiers::CONTROL)
                    {
                        break Ok(());
                    }
                }
            }

            stdout().queue(terminal::Clear(terminal::ClearType::FromCursorUp))?;

            for pipe in self.pipes.iter_mut() {
                let mut cr = 0;
                let mut head_y: i16 = (pipe.head.y) as i16;

                while head_y >= 0 {
                    if let Some(m) = pipe.msg.msg_vd.get(cr + pipe.msg.lp) {
                        let mut stdout = stdout();
                        let q = stdout.queue(cursor::MoveTo(pipe.head.x, head_y as u16))?;
                        q.queue(style::PrintStyledContent(m.dark_green()))?;
                    }

                    head_y -= 1;
                    cr += 1;
                }

                let h = pipe.head;

                if !pipe.pipe_hit_border(self.size, &h) {
                    pipe.head.y += 1;
                } else {
                    pipe.msg.lp += 1;
                }

                // info!("lp {:?}, len: {:?}", pipe.msg.lp, pipe.msg.msg_vd.len());

                if pipe.msg.lp >= pipe.msg.msg_vd.len() {
                    pipe.head.y = 0;
                    pipe.msg.lp = 0;
                }
            }

            stdout().flush()?;
            thread::sleep(Duration::from_millis(200));
        }
    }
}

fn main() -> io::Result<()> {
    let msg = "This is my msg that will run".to_string();

    let msg = msg.split_whitespace().fold(String::new(), |acc, e| {
        let e: String = e.chars().rev().collect();
        format!("{acc} {e}")
    });

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let size = terminal::size()?;

    terminal::enable_raw_mode()?;

    let mut app = App::new(size, msg);

    app.run()?;

    terminal::disable_raw_mode()?;
    stdout().queue(cursor::Show)?;

    Ok(())
}
