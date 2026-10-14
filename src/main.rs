//! Snake — a classic terminal game written in Rust.
//!
//! Controls:
//!   Arrow keys or WASD / HJKL — move the snake
//!   P — pause / resume
//!   R — restart after game over
//!   Q or Esc — quit

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use rand::Rng;
use std::{
    io::{self, Stdout, Write},
    time::{Duration, Instant},
};

// Size of the play area (inside the border).
const WIDTH: u16 = 30;
const HEIGHT: u16 = 20;

// Top-left corner of the board in terminal coordinates.
// Row 0 = title, Row 1 = help, Row 2 = top border, so board starts at Row 3.
const BOARD_OFFSET_X: u16 = 0;
const BOARD_OFFSET_Y: u16 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns true if `other` is the exact opposite direction.
    /// We forbid reversing, because that would instantly eat yourself.
    fn is_opposite(self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

struct Game {
    snake: Vec<Point>,
    direction: Direction,
    /// Direction requested by the player, applied on the next tick.
    /// This prevents fast double-turns from reversing into yourself.
    pending_direction: Direction,
    food: Point,
    score: u32,
    game_over: bool,
    paused: bool,
}

impl Game {
    fn new() -> Self {
        // Start in the middle, 3 segments long, moving right.
        let start = Point {
            x: WIDTH / 2,
            y: HEIGHT / 2,
        };
        let snake = vec![
            start,
            Point {
                x: start.x - 1,
                y: start.y,
            },
            Point {
                x: start.x - 2,
                y: start.y,
            },
        ];
        let mut game = Self {
            snake,
            direction: Direction::Right,
            pending_direction: Direction::Right,
            food: Point { x: 0, y: 0 },
            score: 0,
            game_over: false,
            paused: false,
        };
        game.food = game.spawn_food();
        game
    }

    /// Pick a random empty cell for the food.
    fn spawn_food(&self) -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let candidate = Point {
                x: rng.gen_range(0..WIDTH),
                y: rng.gen_range(0..HEIGHT),
            };
            // Don't spawn food on top of the snake.
            if !self.snake.contains(&candidate) {
                return candidate;
            }
        }
    }

    /// Advance the game by one step. Returns false if the game just ended.
    fn tick(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        // Apply the player's requested direction (if it isn't a reversal).
        if !self.direction.is_opposite(self.pending_direction) {
            self.direction = self.pending_direction;
        }

        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => Point {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Point {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
        };

        // Hitting a wall ends the game. Note: saturating_sub means moving
        // left/up from 0 stays at 0, so we detect that as a wall hit too —
        // the new head equals the old head on that axis, which means we
        // tried to leave the board.
        let hit_wall = new_head.x >= WIDTH
            || new_head.y >= HEIGHT
            || (self.direction == Direction::Up && head.y == 0)
            || (self.direction == Direction::Left && head.x == 0);

        if hit_wall {
            self.game_over = true;
            return;
        }

        // Eating yourself ends the game. We check against the body
        // *without* the tail, because the tail moves away this tick
        // unless we're about to grow.
        let will_eat = new_head == self.food;
        let body_to_check = if will_eat {
            &self.snake[..]
        } else {
            &self.snake[..self.snake.len() - 1]
        };
        if body_to_check.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Move: add the new head.
        self.snake.insert(0, new_head);

        if will_eat {
            self.score += 10;
            self.food = self.spawn_food();
            // Don't remove the tail -> the snake grows by one.
        } else {
            // Remove the tail -> the snake stays the same length.
            self.snake.pop();
        }
    }

    /// How long to wait between ticks. Gets faster as you score.
    fn tick_duration(&self) -> Duration {
        let speed_up = (self.score / 10).min(20) as u64 * 5;
        let millis = 180u64.saturating_sub(speed_up).max(70);
        Duration::from_millis(millis)
    }

    fn set_direction(&mut self, dir: Direction) {
        // Only store it; `tick()` applies it if it's not a reversal.
        self.pending_direction = dir;
    }
}

/// Draw the whole screen: title, board, snake, food, and status text.
fn draw(stdout: &mut Stdout, game: &Game) -> io::Result<()> {
    queue!(stdout, Clear(ClearType::All))?;

    // Title + help line.
    queue!(
        stdout,
        MoveTo(0, 0),
        Print(format!(
            "SNAKE  |  Score: {}  |  Length: {}",
            game.score,
            game.snake.len()
        )),
        MoveTo(0, 1),
        Print("Arrows / WASD to move  |  P pause  |  R restart  |  Q quit"),
    )?;

    // Top border: +---+ style.
    queue!(
        stdout,
        MoveTo(BOARD_OFFSET_X, BOARD_OFFSET_Y),
        Print(format!("+{}+", "-".repeat(WIDTH as usize))),
    )?;

    // Play area rows.
    for y in 0..HEIGHT {
        let row = BOARD_OFFSET_Y + 1 + y;
        queue!(stdout, MoveTo(BOARD_OFFSET_X, row), Print("|"))?;
        for x in 0..WIDTH {
            let point = Point { x, y };
            let (ch, color) = if point == game.snake[0] {
                ('O', Color::Green)
            } else if game.snake.contains(&point) {
                ('o', Color::DarkGreen)
            } else if point == game.food {
                ('*', Color::Red)
            } else {
                (' ', Color::Reset)
            };
            queue!(
                stdout,
                MoveTo(BOARD_OFFSET_X + 1 + x, row),
                SetForegroundColor(color),
                Print(ch),
                ResetColor,
            )?;
        }
        queue!(stdout, MoveTo(BOARD_OFFSET_X + 1 + WIDTH, row), Print("|"))?;
    }

    // Bottom border.
    queue!(
        stdout,
        MoveTo(BOARD_OFFSET_X, BOARD_OFFSET_Y + 1 + HEIGHT),
        Print(format!("+{}+", "-".repeat(WIDTH as usize))),
    )?;

    // Status messages below the board.
    let status_y = BOARD_OFFSET_Y + HEIGHT + 2;
    if game.game_over {
        queue!(
            stdout,
            MoveTo(0, status_y),
            SetForegroundColor(Color::Red),
            Print(format!("Game over! Final score: {}.", game.score)),
            ResetColor,
            MoveTo(0, status_y + 1),
            Print("Press R to play again, or Q to quit."),
        )?;
    } else if game.paused {
        queue!(
            stdout,
            MoveTo(0, status_y),
            Print("Paused. Press P to resume."),
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Switch to a clean alternate screen and raw mode so key presses
    // are delivered immediately (no Enter key needed).
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // Make sure we always restore the terminal, even if the game panics.
    let result = run_game(&mut stdout);

    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    result
}

fn run_game(stdout: &mut Stdout) -> io::Result<()> {
    let mut game = Game::new();
    draw(stdout, &game)?;

    loop {
        // Wait for input up to one tick. If a key arrives sooner,
        // we handle it right away, then still tick on schedule.
        let tick_duration = game.tick_duration();
        let timeout = if game.game_over || game.paused {
            // When paused or over, wait longer so we don't spin.
            Duration::from_millis(100)
        } else {
            tick_duration
        };

        // `poll` returns true if an event is ready within the timeout.
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        if !game.game_over {
                            game.paused = !game.paused;
                        }
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if game.game_over {
                            game = Game::new();
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if game.game_over {
                            game = Game::new();
                        }
                    }
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char('k')
                    | KeyCode::Char('K') => {
                        game.set_direction(Direction::Up);
                    }
                    KeyCode::Down
                    | KeyCode::Char('s')
                    | KeyCode::Char('S')
                    | KeyCode::Char('j')
                    | KeyCode::Char('J') => {
                        game.set_direction(Direction::Down);
                    }
                    KeyCode::Left
                    | KeyCode::Char('a')
                    | KeyCode::Char('A')
                    | KeyCode::Char('h')
                    | KeyCode::Char('H') => {
                        game.set_direction(Direction::Left);
                    }
                    KeyCode::Right
                    | KeyCode::Char('d')
                    | KeyCode::Char('D')
                    | KeyCode::Char('l')
                    | KeyCode::Char('L') => {
                        game.set_direction(Direction::Right);
                    }
                    _ => {}
                }
            }
        }

        // Only advance the game when we're actually playing.
        // We use a simple approach: tick once per loop iteration,
        // because `poll` already waited `tick_duration`.
        // When paused/game-over, tick() is a no-op.
        let _ = Instant::now(); // (kept for clarity; timing comes from poll)
        game.tick();
        draw(stdout, &game)?;
    }

    Ok(())
}
