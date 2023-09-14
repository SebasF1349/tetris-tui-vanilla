use std::{fmt, fmt::Display, io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

async fn print_events() {
    let mut board = Board::new(10, 10);
    board.start();
    let mut reader = EventStream::new();
    let mut limit = 10;

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
                board.draw();
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('a').into()) {
                            board.left();
                            board.draw();
                        }

                        if event == Event::Key(KeyCode::Char('d').into()) {
                            board.right();
                            board.draw();
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
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    print_events().await;

    execute!(stdout, DisableMouseCapture)?;

    clear_screen();
    show_cursor();

    disable_raw_mode()
}

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
        self.x -= 1;
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
    fn add_block(&mut self, row: usize, col: usize) {
        self.board[self.cols * row + col] = 1;
    }

    fn down(&mut self) {
        self.block.down();
    }

    fn left(&mut self) {
        self.block.left();
    }

    fn right(&mut self) {
        self.block.right();
    }

    fn new(cols: usize, rows: usize) -> Board {
        Board {
            cols,
            rows,
            board: vec![0; cols * rows],
            block: Block::new(0, 0),
        }
    }

    fn start(&mut self) {
        hide_cursor();
        clear_screen();
        self.block = Block { x: 3, y: 0 };
        println!("{}", self);
    }

    fn end(&self) {
        show_cursor();
    }

    fn draw(&self) {
        clear_screen();
        println!("{}", self);
    }
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
