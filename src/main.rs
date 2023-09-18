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
    let mut tetris = Tetris::new(10, 10);
    tetris.start();
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => {
                let collision = tetris.block_down();
                if let Err(_) = collision {
                    tetris.add_block(0, 5);
                    tetris.draw();
                }
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('a').into()) {
                            let _ = tetris.block_left();
                        }

                        if event == Event::Key(KeyCode::Char('d').into()) {
                            let _ = tetris.block_right();
                        }

                        if event == Event::Key(KeyCode::Char('s').into()) {
                            let _ = tetris.block_down();
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
enum Square {
    Empty,
    Occupied,
}

/*impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Empty => write!(f, " "),
            Square::Occupied => write!(f, "█"),
        }
    }
}*/

impl ToString for Square {
    fn to_string(&self) -> String {
        match self {
            Square::Empty => String::from(" "),
            Square::Occupied => String::from("█"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tetris {
    cols: usize,
    rows: usize,
    board: Vec<Vec<Square>>,
    block: Block,
}

impl Display for Tetris {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = self.board.clone();
        output[self.block.row][self.block.col] = Square::Occupied;
        let output: Vec<String> = output
            .iter_mut()
            .map(|val| {
                let ret: Vec<String> = val.iter().map(|num| num.to_string()).collect();
                format!("|{}|", ret.join(""))
            })
            .collect();
        write!(f, "{}\n\r {}", output.join("\n\r"), "-".repeat(self.cols))
    }
}

impl Tetris {
    fn new(cols: usize, rows: usize) -> Tetris {
        Tetris {
            cols,
            rows,
            board: vec![vec![Square::Empty; cols]; rows],
            block: Block::new(0, 0),
        }
    }

    fn add_block(&mut self, row: usize, col: usize) {
        self.board[self.block.row][self.block.col] = Square::Occupied;
        self.block = Block::new(col, row);
    }

    fn block_down(&mut self) -> Result<(), Error> {
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

    fn block_left(&mut self) -> Result<(), Error> {
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

    fn block_right(&mut self) -> Result<(), Error> {
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
        block.col >= self.cols
            || block.row >= self.rows
            || self.board[block.row][block.col] == Square::Occupied
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
        let mut tetris = Tetris::new(10, 10);
        tetris.add_block(3, 4);
        assert_eq!(tetris.block, Block { row: 3, col: 4 });
        assert_eq!(tetris.board[0][0], Square::Occupied);
    }

    #[test]
    fn test_collision() {
        let mut tetris = Tetris::new(10, 10);
        tetris.board[2][3] = Square::Occupied;
        let block = Block { row: 2, col: 3 };
        assert!(tetris.is_collision(block));
    }
}
