use std::{fmt, fmt::Display, io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode},
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

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => {
                let collision = board.down();
                if let Err(_) = collision {
                    board.add_block(0, 5);
                    board.draw();
                }
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('a').into()) {
                            let _ = board.left();
                        }

                        if event == Event::Key(KeyCode::Char('d').into()) {
                            let _ = board.right();
                        }

                        if event == Event::Key(KeyCode::Char('s').into()) {
                            let _ = board.down();
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Block {
    col: usize,
    row: usize,
}

impl Block {
    fn new(col: usize, row: usize) -> Block {
        Block { col, row }
    }

    fn down(&mut self) {
        self.row += 1;
    }

    fn left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    fn right(&mut self) {
        self.col += 1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board {
    cols: usize,
    rows: usize,
    board: Vec<Vec<i32>>,
    block: Block,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output: Vec<String> = self
            .board
            .clone()
            .iter_mut()
            .enumerate()
            .map(|(pos, val)| {
                if self.block.row == pos {
                    val[self.block.col] = 1;
                }
                let ret: Vec<String> = val
                    .iter()
                    .map(|num| {
                        if num == &0 {
                            " ".to_string()
                        } else {
                            num.to_string()
                        }
                    })
                    .collect();
                format!("|{}|", ret.join(""))
            })
            .collect();
        println!(
            "{}-{}-{}",
            self.block.row, self.block.col, self.board[self.block.row][self.block.col]
        );
        write!(f, "{}\n\r {}", output.join("\n\r"), "-".repeat(self.cols))
    }
}

impl Board {
    fn new(cols: usize, rows: usize) -> Board {
        Board {
            cols,
            rows,
            board: vec![vec![0; cols]; rows],
            block: Block::new(0, 0),
        }
    }

    fn add_block(&mut self, row: usize, col: usize) {
        self.board[self.block.row][self.block.col] = 1;
        self.block = Block::new(col, row);
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
        self.block = Block { col: 3, row: 0 };
        println!("{}", self);
    }

    fn draw(&self) {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self);
    }

    fn is_collision(&self, block: Block) -> bool {
        block.col >= self.cols || block.row >= self.rows || self.board[block.row][block.col] != 0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut board = Board::new(10, 10);
        board.add_block(3, 4);
        assert_eq!(board.block, Block { row: 3, col: 4 });
        assert_eq!(board.board[0][0], 1);
    }

    #[test]
    fn test_collision() {
        let mut board = Board::new(10, 10);
        board.board[2][3] = 1;
        let block = Block { row: 2, col: 3 };
        assert!(board.is_collision(block));
    }
}
