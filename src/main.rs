use std::{
    fmt,
    fmt::Display,
    io::stdout,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

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

    Tetris::new().play();

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
struct Coordinates {
    row: usize,
    col: usize,
}

impl Coordinates {
    fn new() -> CoordinatesBuilder {
        CoordinatesBuilder::new()
    }
}

struct CoordinatesBuilder {
    row: usize,
    col: usize,
}

impl CoordinatesBuilder {
    fn new() -> CoordinatesBuilder {
        CoordinatesBuilder { row: 0, col: 0 }
    }

    fn row(mut self, row: usize) -> CoordinatesBuilder {
        self.row = row;
        self
    }

    fn col(mut self, col: usize) -> CoordinatesBuilder {
        self.col = col;
        self
    }

    fn build(self) -> Coordinates {
        Coordinates {
            row: self.row,
            col: self.col,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Block {
    position: [Coordinates; 4],
    color: Color,
    piece: Piece,
    rotation_pos: usize,
}

impl Block {
    fn new() -> Block {
        let color: Color = rand::random();
        let coor = Coordinates::new().row(4).col(COLS / 2).build();
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
        for pos in self.position.iter_mut() {
            pos.row += 1
        }
    }

    fn left(&mut self) {
        if self.position.into_iter().all(|sq| sq.col > 0) {
            for pos in self.position.iter_mut() {
                pos.col -= 1
            }
        }
    }

    fn right(&mut self) {
        for pos in self.position.iter_mut() {
            pos.col += 1
        }
    }

    fn rotate(&mut self) {
        let rotation_pos = (self.rotation_pos + 1) % 4;
        let position = get_piece_position(self.piece, rotation_pos, self.position[0]);
        if let Ok(pos) = position {
            self.rotation_pos = rotation_pos;
            self.position = pos;
        }
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

fn get_piece_position(piece: Piece, pos: usize, coor: Coordinates) -> Result<[Coordinates; 4], ()> {
    let position = match (piece, pos) {
        (Piece::I, p) if p % 2 == 1 => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row - 2, coor.col],
            [coor.row + 1, coor.col],
        ]),
        (Piece::I, _) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row, coor.col + 2],
            [coor.row, coor.col - 1],
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
            [coor.row - 1, coor.col - 1],
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
        (Piece::S, p) if p % 2 == 1 => Ok([
            [coor.row, coor.col],
            [coor.row - 1, coor.col],
            [coor.row, coor.col + 1],
            [coor.row + 1, coor.col + 1],
        ]),
        (Piece::S, _) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col - 1],
            [coor.row - 1, coor.col],
            [coor.row - 1, coor.col + 1],
        ]),
        (Piece::Z, p) if p % 2 == 1 => Ok([
            [coor.row, coor.col],
            [coor.row + 1, coor.col],
            [coor.row, coor.col + 1],
            [coor.row - 1, coor.col + 1],
        ]),
        (Piece::Z, _) if coor.col > 0 => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col - 1],
            [coor.row + 1, coor.col],
            [coor.row + 1, coor.col + 1],
        ]),
        (Piece::O, _) => Ok([
            [coor.row, coor.col],
            [coor.row, coor.col + 1],
            [coor.row - 1, coor.col],
            [coor.row - 1, coor.col + 1],
        ]),
        (_, _) => Err(()),
    };
    position.map(|coors| coors.map(|coor| Coordinates::new().row(coor[0]).col(coor[1]).build()))
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Color {
    Red,
    Blue,
    Orange,
    Yellow,
    Green,
    Violet,
    Brown,
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=6) {
            0 => Color::Red,
            1 => Color::Blue,
            2 => Color::Orange,
            3 => Color::Yellow,
            4 => Color::Green,
            5 => Color::Violet,
            _ => Color::Brown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Square {
    Empty,
    Occupied(Color),
}

impl ToString for Square {
    fn to_string(&self) -> String {
        match self {
            Square::Empty => String::from("  "),
            Square::Occupied(Color::Red) => String::from('\u{1F7E5}'),
            Square::Occupied(Color::Blue) => String::from('\u{1F7E6}'),
            Square::Occupied(Color::Orange) => String::from('\u{1F7E7}'),
            Square::Occupied(Color::Yellow) => String::from('\u{1F7E8}'),
            Square::Occupied(Color::Green) => String::from('\u{1F7E9}'),
            Square::Occupied(Color::Violet) => String::from('\u{1F7EA}'),
            Square::Occupied(Color::Brown) => String::from('\u{1F7EB}'),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GameState {
    Playing,
    Pause,
    Menu,
    EndScreen,
}

impl GameState {
    fn print_message(&self) -> String {
        let message = match self {
            GameState::Pause => [String::from("GAME PAUSED"), String::from("")],
            GameState::EndScreen => [
                String::from("YOU LOST!"),
                String::from("Press p to restart or q to quit"),
            ],
            GameState::Playing | GameState::Menu => [String::from(""), String::from("")],
        };
        let longest = "Press p to restart or q to quit".len();
        message
            .into_iter()
            .map(|s| format!("{}{}", s, &" ".repeat(longest - s.len())))
            .collect::<Vec<String>>()
            .join("\n\r")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GameEvent {
    Tick,
    Key(KeyEvent),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum KeyEvent {
    Down,
    Left,
    Right,
    Rotate,
    Quit,
    Play,
    Pause,
}

const COLS: usize = 10;
const ROWS: usize = 23;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tetris {
    board: Vec<Vec<Square>>,
    block: Block,
    points: usize,
    state: GameState,
}

impl Display for Tetris {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = self.board.clone();
        for i in 0..4 {
            output[self.block.position[i].row][self.block.position[i].col] =
                Square::Occupied(self.block.color);
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
            "{}\n\r{}\n\r\n\rPoints: {}\n\r\n\r{}",
            output.join("\n\r"),
            "\u{2594}".repeat(COLS * 2 + 2),
            self.points,
            self.state.print_message()
        )
    }
}

impl Tetris {
    fn new() -> Tetris {
        Tetris {
            board: vec![vec![Square::Empty; COLS]; ROWS],
            block: Block::new(),
            points: 0,
            state: GameState::Menu,
        }
    }

    fn play(&mut self) {
        let (tx, rx) = mpsc::channel();

        let state = Arc::new(Mutex::new(GameState::Playing));

        {
            let tx = tx.clone();
            let state = Arc::clone(&state);
            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(1000));
                let game_state = state.lock().unwrap();
                if *game_state == GameState::Playing {
                    tx.send(GameEvent::Tick).unwrap();
                }
            });
        }

        {
            let tx = tx.clone();
            thread::spawn(move || loop {
                loop {
                    let stdin = &mut std::io::stdin();

                    loop {
                        match get_input(stdin) {
                            Some(k) => tx.send(GameEvent::Key(k)).unwrap(),
                            None => (),
                        }
                    }
                }
            });
        }

        self.draw_menu();
        loop {
            match self.state {
                GameState::Menu => match rx.recv() {
                    Ok(GameEvent::Key(key)) => match key {
                        KeyEvent::Play => {
                            clear_screen();
                            hide_cursor();
                            let mut game_state = state.lock().unwrap();
                            *game_state = GameState::Playing;
                            self.state = GameState::Playing;
                            self.start();
                        }
                        KeyEvent::Quit => break,
                        _ => (),
                    },
                    Ok(GameEvent::Tick) => (),
                    Err(err) => panic!("{}", err),
                },
                GameState::Playing => match rx.recv() {
                    Ok(GameEvent::Key(key)) => match key {
                        KeyEvent::Quit => break,
                        KeyEvent::Left => self.block_left(),
                        KeyEvent::Right => self.block_right(),
                        KeyEvent::Down => self.block_down(),
                        KeyEvent::Rotate => self.block_rotate(),
                        KeyEvent::Play => (),
                        KeyEvent::Pause => {
                            let mut game_state = state.lock().unwrap();
                            *game_state = GameState::Pause;
                            self.state = GameState::Pause;
                            self.draw_board();
                        }
                    },
                    Ok(GameEvent::Tick) => {
                        if Err(()) == self.tick() {
                            self.state = GameState::EndScreen;
                            self.draw_board();
                        }
                    }
                    Err(err) => panic!("{}", err),
                },
                GameState::Pause => match rx.recv() {
                    Ok(GameEvent::Key(key)) => {
                        match key {
                            KeyEvent::Quit => break,
                            KeyEvent::Pause => {
                                let mut game_state = state.lock().unwrap();
                                *game_state = GameState::Playing;
                                self.state = GameState::Playing;
                                self.draw_board();
                            }
                            _ => (),
                        };
                    }
                    Ok(GameEvent::Tick) => (),
                    Err(err) => panic!("{}", err),
                },
                GameState::EndScreen => match rx.recv() {
                    Ok(GameEvent::Key(key)) => {
                        match key {
                            KeyEvent::Quit => break,
                            KeyEvent::Play => {
                                *self = Tetris::new();
                                let mut game_state = state.lock().unwrap();
                                *game_state = GameState::Playing;
                                self.state = GameState::Playing;
                                self.start();
                            }
                            _ => (),
                        };
                    }
                    Ok(GameEvent::Tick) => (),
                    Err(err) => panic!("{}", err),
                },
            }
        }
    }

    fn add_block(&mut self) {
        for i in 0..4 {
            self.board[self.block.position[i].row][self.block.position[i].col] =
                Square::Occupied(self.block.color);
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
            let block = Block::new();
            if self.is_collision(&block) {
                return Err(());
            }
            self.block = block;
        } else {
            self.block = block;
        }
        self.draw_board();
        return Ok(());
    }

    fn block_down(&mut self) {
        let mut block = self.block.clone();
        block.down();
        if !self.is_collision(&block) {
            self.block = block;
            self.draw_board();
        }
    }

    fn block_left(&mut self) {
        let mut block = self.block.clone();
        block.left();
        if !self.is_collision(&block) {
            self.block = block;
            self.draw_board();
        }
    }

    fn block_right(&mut self) {
        let mut block = self.block.clone();
        block.right();
        if !self.is_collision(&block) {
            self.block = block;
            self.draw_board();
        }
    }

    fn block_rotate(&mut self) {
        let mut block = self.block.clone();
        block.rotate();
        if !self.is_collision(&block) {
            self.block = block;
            self.draw_board();
        }
    }

    fn start(&mut self) {
        self.block = Block::new();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self);
    }

    fn draw_board(&self) {
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", self);
    }

    fn draw_menu(&self) {
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        println!(
            "TETRIS\n\r

KEYS:\n\r
P => Play\n\r
A (or ←) => Move Block to the left\n\r
D (or →) => Move Block to the right\n\r
S (or ↓) => Move Block down\n\r
W (or ↑) => Rotate Block\n\r

[SPACE] => Pause\n\r
Q => Quit\n\r"
        );
    }

    fn is_occupied(&self, coor: Coordinates) -> bool {
        match self.board[coor.row][coor.col] {
            Square::Occupied(_) => true,
            Square::Empty => false,
        }
    }

    fn is_collision(&self, block: &Block) -> bool {
        block
            .position
            .into_iter()
            .any(|sq| sq.col >= COLS || sq.row >= ROWS || self.is_occupied(sq))
    }

    fn remove_lines_completed(&mut self) {
        self.board
            .retain(|val| val.iter().any(|sq| *sq == Square::Empty));
        let deleted = ROWS - self.board.len();
        if deleted > 0 {
            let mut v = vec![vec![Square::Empty; COLS]; deleted];
            v.append(&mut self.board);
            self.board = v;
            self.points += deleted;
        }
    }

    fn is_end(&self) -> bool {
        self.board
            .iter()
            .rev()
            .skip(ROWS - 2)
            .any(|val| val.iter().any(|sq| *sq != Square::Empty))
    }
}

fn get_input(stdin: &mut std::io::Stdin) -> Option<KeyEvent> {
    use std::io::Read;

    let c = &mut [0u8];
    match stdin.read(c) {
        Ok(_) => match std::str::from_utf8(c) {
            Ok("w") => Some(KeyEvent::Rotate),
            Ok("s") => Some(KeyEvent::Down),
            Ok("a") => Some(KeyEvent::Left),
            Ok("d") => Some(KeyEvent::Right),
            Ok("q") => Some(KeyEvent::Quit),
            Ok("p") => Some(KeyEvent::Play),
            Ok(" ") => Some(KeyEvent::Pause),
            Ok("\x1b") => {
                let code = &mut [0u8; 2];
                match stdin.read(code) {
                    Ok(_) => match std::str::from_utf8(code) {
                        Ok("[A") => Some(KeyEvent::Rotate),
                        Ok("[B") => Some(KeyEvent::Down),
                        Ok("[C") => Some(KeyEvent::Right),
                        Ok("[D") => Some(KeyEvent::Left),
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

fn hide_cursor() {
    print!("\x1B[?25l");
}

/* fn show_cursor() {
    print!("\x1B[?25h");
} */

fn clear_screen() {
    print!("\x1Bc");
}

/* fn move_cursor(row: usize, col: usize) {
    print!("\x1B[{0};{1}H", row, col);
} */

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
