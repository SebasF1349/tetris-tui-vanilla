use std::{
    fmt,
    fmt::Display,
    io::{stdout, Write},
    time::Duration,
};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute, style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to disable raw mode")
    }
}

async fn print_events() {
    let mut board = Board::new(10, 10);
    board.start();
    let mut reader = EventStream::new();
    let mut limit = 12;

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => {
                limit -= 1;
                if limit == 0 {
                    break;
                }
                board.down();
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('a').into()) {
                            board.left();
                        }

                        if event == Event::Key(KeyCode::Char('d').into()) {
                            board.right();
                        }

                        if event == Event::Key(KeyCode::Char('s').into()) {
                            board.down();
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::Hide,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    print_events().await;

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

#[derive(Clone)]
struct Block {
    x: usize,
    y: usize,
}

impl Block {
    fn new(x: usize, y: usize) -> Block {
        Block { x, y }
    }

    fn down(&mut self) {
        self.y += 1;
    }

    fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn right(&mut self) {
        self.x += 1;
    }
}

struct Board {
    cols: usize,
    rows: usize,
    board: Vec<i32>,
    block: Block,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printed_board = self.board.clone();
        printed_board[self.cols * self.block.y + self.block.x] = 1;
        let output = printed_board
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (pos, val)| {
                let text = if val == &0 {
                    " ".to_string()
                } else {
                    val.to_string()
                };
                //                let text = val.to_string();
                if pos % self.cols == 0 {
                    acc + "\n\r" + &text
                } else {
                    acc + " " + &text
                }
            });
        write!(f, "{}", output)
    }
}

impl Board {
    fn new(cols: usize, rows: usize) -> Board {
        Board {
            cols,
            rows,
            board: vec![0; cols * rows],
            block: Block::new(0, 0),
        }
    }

    fn add_block(&mut self, row: usize, col: usize) {
        self.board[self.cols * row + col] = 1;
    }

    fn down(&mut self) -> Result<(), Error> {
        let mut block = self.block.clone();
        block.down();
        if self.is_collision(block) {
            Err(Error::Collision)
        } else {
            self.block.down();
            self.draw();
            Ok(())
        }
    }

    fn left(&mut self) -> Result<(), Error> {
        let mut block = self.block.clone();
        block.left();
        if self.is_collision(block) {
            Err(Error::Collision)
        } else {
            self.block.left();
            self.draw();
            Ok(())
        }
    }

    fn right(&mut self) -> Result<(), Error> {
        let mut block = self.block.clone();
        block.right();
        if self.is_collision(block) {
            Err(Error::Collision)
        } else {
            self.block.right();
            self.draw();
            Ok(())
        }
    }

    fn start(&mut self) {
        self.block = Block { x: 3, y: 0 };
        println!("{}", self);
    }

    fn end(&self) {
        //show_cursor();
    }

    fn draw(&self) {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self);
    }

    fn is_collision(&self, block: Block) -> bool {
        if block.x >= self.cols || block.y >= self.rows {
            return true;
        }
        self.board[self.cols * block.y + block.x] != 0
    }
}

enum Error {
    Collision,
}

fn hide_cursor() {
    print!("\x1B[?25l");
}

fn show_cursor() {
    print!("\x1B[?25h");
}

fn clear_screen() {
    print!("\x1Bc");
}

fn move_cursor(row: usize, col: usize) {
    print!("\x1B[{0};{1}H", row, col);
}
