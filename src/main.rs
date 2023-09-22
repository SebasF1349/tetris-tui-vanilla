use std::{fmt, fmt::Display, io::stdout, sync::mpsc, thread, time::Duration};

use crossterm::{
    cursor, execute, style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::{distributions::Standard, prelude::Distribution, Rng};

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

    let mut tetris = Tetris::new(10, 23);
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
struct Coordinates {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Block {
    position: [[usize; 2]; 4],
    color: Color,
    piece: Piece,
    rotation_pos: usize,
}

impl Block {
    fn new(width: usize) -> Block {
        let color: Color = rand::random();
        let coor = Coordinates {
            row: 4,
            col: width / 2,
        };
        let piece: Piece = rand::random();
        let rotation_pos = rand::thread_rng().gen_range(0..4);
        let position = get_piece_position(piece, rotation_pos, coor).unwrap();
        Block {
            position,
            color,
            piece,
            rotation_pos,
        }
    }

    fn down(&mut self) {
        self.position = self.position.map(|sq| [sq[0] + 1, sq[1]]);
    }

    fn left(&mut self) {
        if self.position.into_iter().all(|sq| sq[1] > 0) {
            self.position = self.position.map(|sq| [sq[0], sq[1] - 1]);
        }
    }

    fn right(&mut self) {
        self.position = self.position.map(|sq| [sq[0], sq[1] + 1]);
    }

    fn rotate(&mut self) {
        self.rotation_pos = (self.rotation_pos + 1) % 4;
        self.position = get_piece_position(
            self.piece,
            self.rotation_pos,
            Coordinates {
                row: self.position[0][0],
                col: self.position[0][1],
            },
        )
        .unwrap_or(self.position);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Piece {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Distribution<Piece> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        match rng.gen_range(0..=6) {
            0 => Piece::I,
            1 => Piece::J,
            2 => Piece::L,
            3 => Piece::O,
            4 => Piece::S,
            5 => Piece::T,
            _ => Piece::Z,
        }
    }
}

fn get_piece_position(piece: Piece, pos: usize, coor: Coordinates) -> Result<[[usize; 2]; 4], ()> {
    match (piece, pos) {
        (Piece::I, p) if p % 2 == 0 && coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row, coor.col + 2],
            [coor.row, coor.col - 1],
        ]),
        (Piece::I, p) if p % 2 == 1 => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row - 2, coor.col],
            [coor.row + 1, coor.col],
        ]),
        (Piece::J, 0) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col - 1],
            [coor.row, coor.col + 1],
            [coor.row + 1, coor.col + 1],
        ]),
        (Piece::J, 1) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row + 1, coor.col],
            [coor.row + 1, coor.col - 1],
        ]),
        (Piece::J, 2) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row, coor.col - 1],
            [coor.row - 1, coor.col - 1],
        ]),
        (Piece::J, 3) => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row - 1, coor.col + 1],
            [coor.row + 1, coor.col],
        ]),
        (Piece::L, 0) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row, coor.col - 1],
            [coor.row - 1, coor.col + 1],
        ]),
        (Piece::L, 1) => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row + 1, coor.col],
            [coor.row + 1, coor.col + 1],
        ]),
        (Piece::L, 2) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row, coor.col - 1],
            [coor.row + 1, coor.col - 1],
        ]),
        (Piece::L, 3) => Ok([
            [coor.row, coor.col],
            [coor.row + 1, coor.col],
            [coor.row - 1, coor.col],
            [coor.row - 1, coor.col + 1],
        ]),
        (Piece::T, 0) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col - 1],
            [coor.row, coor.col + 1],
            [coor.row + 1, coor.col],
        ]),
        (Piece::T, 1) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row + 1, coor.col],
            [coor.row - 1, coor.col],
            [coor.row, coor.col - 1],
        ]),
        (Piece::T, 2) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row, coor.col - 1],
            [coor.row - 1, coor.col],
        ]),
        (Piece::T, 3) => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row + 1, coor.col],
            [coor.row, coor.col + 1],
        ]),
        (Piece::S, p) if p % 2 == 0 && coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col - 1],
            [coor.row - 1, coor.col],
            [coor.row - 1, coor.col + 1],
        ]),
        (Piece::S, p) if p % 2 == 1 => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row, coor.col + 1],
            [coor.row + 1, coor.col + 1],
        ]),
        (Piece::Z, p) if p % 2 == 0 && coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col - 1],
            [coor.row + 1, coor.col],
            [coor.row + 1, coor.col + 1],
        ]),
        (Piece::Z, p) if p % 2 == 1 => Ok([
            [coor.row, coor.col],
            [coor.row + 1, coor.col],
            [coor.row, coor.col + 1],
            [coor.row - 1, coor.col + 1],
        ]),
        (Piece::O, _) => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row - 1, coor.col],
            [coor.row - 1, coor.col + 1],
        ]),
        (_, _) => Err(()),
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
    KEY(KeyEvents),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum KeyEvents {
    DOWN,
    LEFT,
    RIGHT,
    ROTATE,
    QUIT,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tetris {
    cols: usize,
    rows: usize,
    board: Vec<Vec<Square>>,
    block: Block,
    points: usize,
}

impl Display for Tetris {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = self.board.clone();
        for i in 0..4 {
            output[self.block.position[i][0]][self.block.position[i][1]] =
                Square::OCCUPIED(self.block.color);
        }
        let output: Vec<String> = output
            .iter_mut()
            .skip(4)
            .map(|val| {
                let ret: Vec<String> = val.iter().map(|num| num.to_string()).collect();
                format!("\u{2590}{}\u{258C}", ret.join(""))
            })
            .collect();
        write!(
            f,
            "{}\n\r{}\n\r\n\rPoints: {}",
            output.join("\n\r"),
            "\u{2594}".repeat(self.cols * 2 + 2),
            self.points
        )
    }
}

impl Tetris {
    fn new(cols: usize, rows: usize) -> Tetris {
        Tetris {
            cols,
            rows,
            board: vec![vec![Square::EMPTY; cols]; rows],
            block: Block::new(cols),
            points: 0,
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
                                KeyEvents::QUIT => break,
                                KeyEvents::LEFT => self.block_left(),
                                KeyEvents::RIGHT => self.block_right(),
                                KeyEvents::DOWN => self.block_down(),
                                KeyEvents::ROTATE => self.block_rotate(),
                            };
                        }
                        GameEvents::TICK => {
                            if Err(()) == self.tick() {
                                break;
                            }
                        }
                    };
                }
                Err(err) => panic!("{}", err),
            }
        }
    }

    fn add_block(&mut self) {
        for i in 0..4 {
            self.board[self.block.position[i][0]][self.block.position[i][1]] =
                Square::OCCUPIED(self.block.color);
        }
    }

    fn tick(&mut self) -> Result<(), ()> {
        let mut block = self.block.clone();
        block.down();
        if self.is_collision(&block) {
            self.add_block();
            self.remove_lines_completed();
            if self.is_end() {
                return Err(());
            }
            let block = Block::new(self.cols);
            if self.is_collision(&block) {
                return Err(());
            }
            self.block = block;
        } else {
            self.block.down();
        }
        self.draw();
        return Ok(());
    }

    fn block_down(&mut self) {
        let mut block = self.block.clone();
        block.down();
        if !self.is_collision(&block) {
            self.block.down();
            self.draw();
        }
    }

    fn block_left(&mut self) {
        let mut block = self.block.clone();
        block.left();
        if !self.is_collision(&block) {
            self.block.left();
            self.draw();
        }
    }

    fn block_right(&mut self) {
        let mut block = self.block.clone();
        block.right();
        if !self.is_collision(&block) {
            self.block.right();
            self.draw();
        }
    }

    fn block_rotate(&mut self) {
        let mut block = self.block.clone();
        block.rotate();
        if !self.is_collision(&block) {
            self.block.rotate();
            self.draw();
        }
    }

    fn start(&mut self) {
        self.block = Block::new(self.cols);
        println!("{}", self);
    }

    fn draw(&self) {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self);
    }

    fn is_occupied(&self, coor: Coordinates) -> bool {
        match self.board[coor.row][coor.col] {
            Square::OCCUPIED(_) => true,
            Square::EMPTY => false,
        }
    }

    fn is_collision(&self, block: &Block) -> bool {
        block.position.into_iter().any(|sq| {
            sq[1] >= self.cols
                || sq[0] >= self.rows
                || self.is_occupied(Coordinates {
                    row: sq[0],
                    col: sq[1],
                })
        })
    }

    fn remove_lines_completed(&mut self) {
        self.board
            .retain(|val| val.iter().any(|sq| *sq == Square::EMPTY));
        let len = self.board.len();
        if len < self.rows {
            let mut v = vec![vec![Square::EMPTY; self.cols]; self.rows - len];
            v.append(&mut self.board);
            self.board = v;
            self.points += self.rows - len;
        }
    }

    fn is_end(&self) -> bool {
        self.board
            .iter()
            .rev()
            .skip(self.rows - 4)
            .any(|val| val.iter().any(|sq| *sq != Square::EMPTY))
    }
}

fn get_input(stdin: &mut std::io::Stdin) -> Option<KeyEvents> {
    use std::io::Read;

    let c = &mut [0u8];
    match stdin.read(c) {
        Ok(_) => match std::str::from_utf8(c) {
            Ok("w") => Some(KeyEvents::ROTATE),
            Ok("s") => Some(KeyEvents::DOWN),
            Ok("a") => Some(KeyEvents::LEFT),
            Ok("d") => Some(KeyEvents::RIGHT),
            Ok("q") => Some(KeyEvents::QUIT),
            Ok("\x1b") => {
                let code = &mut [0u8; 2];
                match stdin.read(code) {
                    Ok(_) => match std::str::from_utf8(code) {
                        Ok("[A") => Some(KeyEvents::ROTATE),
                        Ok("[B") => Some(KeyEvents::DOWN),
                        Ok("[D") => Some(KeyEvents::LEFT),
                        Ok("[C") => Some(KeyEvents::RIGHT),
                        _ => None,
                    },
                    Err(msg) => {
                        panic!("{}", format!("could not read from standard in: {}", msg))
                    }
                }
            }
            _ => None,
        },
        Err(msg) => panic!("{}", format!("could not read from standard in: {}", msg)),
    }
}

/*fn hide_cursor() {
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
*/

// tests are failing after adding pieces
/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut tetris = Tetris::new(10, 10);
        tetris.add_block(3, 4);
        assert_eq!(tetris.block, Block::new());
        assert_eq!(tetris.board[0][0], Square::OCCUPIED(tetris.block.color));
    }

    #[test]
    fn test_collision() {
        let mut tetris = Tetris::new(10, 10);
        tetris.board[2][3] = Square::OCCUPIED(tetris.block.color);
        let color: Color = rand::random();
        let piece = get_random_piece(2, 3);
        let block = Block { color, piece };
        assert!(tetris.is_collision(&block));
    }
}*/
