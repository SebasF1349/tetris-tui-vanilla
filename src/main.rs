use std::{fmt, fmt::Display, io::stdout, sync::mpsc, thread, time::Duration};

use crossterm::{
    cursor, execute, style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::{distributions::Standard, prelude::Distribution};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::Hide,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let mut tetris = Tetris::new(10, 10);
    tetris.play();

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
    color: Color,
}

impl Block {
    fn new(col: usize, row: usize) -> Block {
        let color: Color = rand::random();
        Block { col, row, color }
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

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Color {
    RED,
    BLUE,
    ORANGE,
    YELLOW,
    GREEN,
    VIOLET,
    BROWN,
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=6) {
            0 => Color::RED,
            1 => Color::BLUE,
            2 => Color::ORANGE,
            3 => Color::YELLOW,
            4 => Color::GREEN,
            5 => Color::VIOLET,
            _ => Color::BROWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Square {
    EMPTY,
    OCCUPIED(Color),
}

impl ToString for Square {
    fn to_string(&self) -> String {
        match self {
            Square::EMPTY => String::from("  "),
            Square::OCCUPIED(Color::RED) => String::from('\u{1F7E5}'),
            Square::OCCUPIED(Color::BLUE) => String::from('\u{1F7E6}'),
            Square::OCCUPIED(Color::ORANGE) => String::from('\u{1F7E7}'),
            Square::OCCUPIED(Color::YELLOW) => String::from('\u{1F7E8}'),
            Square::OCCUPIED(Color::GREEN) => String::from('\u{1F7E9}'),
            Square::OCCUPIED(Color::VIOLET) => String::from('\u{1F7EA}'),
            Square::OCCUPIED(Color::BROWN) => String::from('\u{1F7EB}'),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GameEvents {
    TICK,
    KEY(Key),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Key {
    DOWN,
    LEFT,
    RIGHT,
    CHAR(char),
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
        output[self.block.row][self.block.col] = Square::OCCUPIED(self.block.color);
        let output: Vec<String> = output
            .iter_mut()
            .map(|val| {
                let ret: Vec<String> = val.iter().map(|num| num.to_string()).collect();
                format!("\u{2590}{}\u{258C}", ret.join(""))
            })
            .collect();
        write!(
            f,
            "{}\n\r{}",
            output.join("\n\r"),
            "\u{2594}".repeat(self.cols * 2 + 2)
        )
    }
}

impl Tetris {
    fn new(cols: usize, rows: usize) -> Tetris {
        Tetris {
            cols,
            rows,
            board: vec![vec![Square::EMPTY; cols]; rows],
            block: Block::new(0, 0),
        }
    }

    fn play(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.start();

        {
            let tx = tx.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(1000));
                tx.send(GameEvents::TICK).unwrap();
            });
        }

        {
            let tx = tx.clone();
            thread::spawn(move || loop {
                loop {
                    let stdin = &mut std::io::stdin();

                    loop {
                        match get_input(stdin) {
                            Some(k) => tx.send(GameEvents::KEY(k)).unwrap(),
                            None => (),
                        }
                    }
                }
            });
        }

        loop {
            match rx.recv() {
                Ok(update) => {
                    match update {
                        GameEvents::KEY(key) => {
                            match key {
                                Key::CHAR('q') => break,
                                k => {
                                    self.keypress(k);
                                }
                            };
                        }
                        GameEvents::TICK => {
                            self.tick();
                        }
                    };
                }
                Err(err) => panic!("{}", err),
            }
        }
    }

    fn keypress(&mut self, key: Key) {
        if key == Key::LEFT {
            self.block_left();
        }

        if key == Key::RIGHT {
            self.block_right();
        }

        if key == Key::DOWN {
            self.block_down();
        }

        /* if key == Event::Key(KeyCode::Esc.into()) {
            break;
        } */
    }

    fn add_block(&mut self, row: usize, col: usize) {
        self.board[self.block.row][self.block.col] = Square::OCCUPIED(self.block.color);
        self.block = Block::new(col, row);
    }

    fn tick(&mut self) {
        let mut block = self.block.clone();
        block.down();
        if self.is_collision(block) {
            self.add_block(0, 5);
        } else {
            self.block.down();
        }
        self.draw();
    }

    fn block_down(&mut self) {
        let mut block = self.block.clone();
        block.down();
        if !self.is_collision(block) {
            self.block.down();
            self.draw();
        }
    }

    fn block_left(&mut self) {
        let mut block = self.block.clone();
        block.left();
        if !self.is_collision(block) {
            self.block.left();
            self.draw();
        }
    }

    fn block_right(&mut self) {
        let mut block = self.block.clone();
        block.right();
        if !self.is_collision(block) {
            self.block.right();
            self.draw();
        }
    }

    fn start(&mut self) {
        self.block = Block::new(5, 0);
        println!("{}", self);
    }

    fn draw(&self) {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self);
    }

    fn is_occupied(&self, row: usize, col: usize) -> bool {
        match self.board[row][col] {
            Square::OCCUPIED(_) => true,
            Square::EMPTY => false,
        }
    }

    fn is_collision(&self, block: Block) -> bool {
        block.col >= self.cols || block.row >= self.rows || self.is_occupied(block.row, block.col)
    }
}

fn get_input(stdin: &mut std::io::Stdin) -> Option<Key> {
    use std::io::Read;

    let c = &mut [0u8];
    match stdin.read(c) {
        Ok(_) => {
            match std::str::from_utf8(c) {
                Ok("s") => Some(Key::DOWN),
                Ok("a") => Some(Key::LEFT),
                Ok("d") => Some(Key::RIGHT),
                Ok("\x1b") => {
                    let code = &mut [0u8; 2];
                    match stdin.read(code) {
                        Ok(_) => {
                            match std::str::from_utf8(code) {
                                //Ok("[A") => Some(Key::Up),
                                Ok("[B") => Some(Key::DOWN),
                                Ok("[D") => Some(Key::LEFT),
                                Ok("[C") => Some(Key::RIGHT),
                                _ => None,
                            }
                        }
                        Err(msg) => {
                            panic!("{}", format!("could not read from standard in: {}", msg))
                        }
                    }
                }
                Ok(n) => Some(Key::CHAR(n.chars().next().unwrap())),
                _ => None,
            }
        }
        Err(msg) => panic!("{}", format!("could not read from standard in: {}", msg)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut tetris = Tetris::new(10, 10);
        tetris.add_block(3, 4);
        assert_eq!(tetris.block, Block::new(3, 4));
        assert_eq!(tetris.board[0][0], Square::OCCUPIED(tetris.block.color));
    }

    #[test]
    fn test_collision() {
        let mut tetris = Tetris::new(10, 10);
        tetris.board[2][3] = Square::OCCUPIED(tetris.block.color);
        let block = Block {
            row: 2,
            col: 3,
            color: tetris.block.color,
        };
        assert!(tetris.is_collision(block));
    }
}
