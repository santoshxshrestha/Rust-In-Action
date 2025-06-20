#![allow(path_statements)]
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, MouseButton, MouseEvent, MouseEventKind, poll, read},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use rand::Rng;
use std::{
    collections::HashMap,
    io::{Write, stdout},
    time::{Duration, Instant},
};

#[derive(Clone)]
struct Target {
    x: u16,
    y: u16,
    spawn_time: Instant,
    lifetime: Duration,
    symbol: char,
}

struct Game {
    targets: HashMap<usize, Target>,
    next_target_id: usize,
    score: u32,
    misses: u32,
    max_misses: u32,
    difficulty: Difficulty,
    last_spawn: Instant,
    game_over: bool,
    width: u16,
    height: u16,
}

#[derive(Clone, Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn spawn_interval(&self) -> Duration {
        match self {
            Difficulty::Easy => Duration::from_millis(2000),
            Difficulty::Medium => Duration::from_millis(1200),
            Difficulty::Hard => Duration::from_millis(800),
        }
    }

    fn target_lifetime(&self) -> Duration {
        match self {
            Difficulty::Easy => Duration::from_millis(3000),
            Difficulty::Medium => Duration::from_millis(2000),
            Difficulty::Hard => Duration::from_millis(1500),
        }
    }
}

impl Game {
    fn new(difficulty: Difficulty) -> Result<Self, Box<dyn std::error::Error>> {
        let (width, height) = size()?;
        Ok(Game {
            targets: HashMap::new(),
            next_target_id: 0,
            score: 0,
            misses: 0,
            max_misses: 5,
            difficulty,
            last_spawn: Instant::now(),
            game_over: false,
            width,
            height: height.saturating_sub(3), // Reserve space for UI
        })
    }

    fn spawn_target(&mut self) {
        if self.last_spawn.elapsed() >= self.difficulty.spawn_interval() {
            let mut rng = rand::thread_rng();
            let symbols = ['●', '▲', '■', '♦', '★'];

            let target = Target {
                x: rng.gen_range(1..self.width.saturating_sub(1)),
                y: rng.gen_range(3..self.height), // Start below UI area
                spawn_time: Instant::now(),
                lifetime: self.difficulty.target_lifetime(),
                symbol: symbols[rng.gen_range(0..symbols.len())],
            };

            self.targets.insert(self.next_target_id, target);
            self.next_target_id += 1;
            self.last_spawn = Instant::now();
        }
    }

    fn update_targets(&mut self) {
        let mut expired_targets = Vec::new();

        for (&id, target) in &self.targets {
            if target.spawn_time.elapsed() >= target.lifetime {
                expired_targets.push(id);
            }
        }

        for id in expired_targets {
            self.targets.remove(&id);
            self.misses += 1;
            if self.misses >= self.max_misses {
                self.game_over = true;
            }
        }
    }

    fn handle_click(&mut self, x: u16, y: u16) -> bool {
        let mut hit_target = None;

        for (&id, target) in &self.targets {
            if target.x == x && target.y == y {
                hit_target = Some(id);
                break;
            }
        }

        if let Some(id) = hit_target {
            self.targets.remove(&id);
            self.score += 10;
            true
        } else {
            false
        }
    }

    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();

        // Clear screen
        queue!(stdout, Clear(ClearType::All))?;

        // Draw UI
        queue!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(Color::Cyan),
            Print(format!(
                "Score: {} | Misses: {}/{} | Difficulty: {:?}",
                self.score, self.misses, self.max_misses, self.difficulty
            )),
            ResetColor,
        )?;

        queue!(
            stdout,
            MoveTo(0, 1),
            SetForegroundColor(Color::Yellow),
            Print("Click the targets before they disappear! Press 'q' to quit."),
            ResetColor,
        )?;

        // Draw targets
        for target in self.targets.values() {
            let age_ratio =
                target.spawn_time.elapsed().as_millis() as f32 / target.lifetime.as_millis() as f32;
            let color = if age_ratio < 0.5 {
                Color::Green
            } else if age_ratio < 0.8 {
                Color::Yellow
            } else {
                Color::Red
            };

            queue!(
                stdout,
                MoveTo(target.x, target.y),
                SetForegroundColor(color),
                Print(target.symbol),
                ResetColor,
            )?;
        }

        if self.game_over {
            let game_over_msg = format!("GAME OVER! Final Score: {}", self.score);
            let x = (self.width.saturating_sub(game_over_msg.len() as u16)) / 2;
            let y = self.height / 2;

            queue!(
                stdout,
                MoveTo(x, y),
                SetForegroundColor(Color::Red),
                Print(game_over_msg),
                MoveTo(x, y + 1),
                Print("Press any key to exit..."),
                ResetColor,
            )?;
        }

        stdout.flush()?;
        Ok(())
    }
}

fn select_difficulty() -> Result<Difficulty, Box<dyn std::error::Error>> {
    println!("Select Difficulty:");
    println!("1. Easy (Slow spawn, long lifetime)");
    println!("2. Medium (Normal spawn, medium lifetime)");
    println!("3. Hard (Fast spawn, short lifetime)");
    print!("Choose (1-3): ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => Ok(Difficulty::Easy),
        "2" => Ok(Difficulty::Medium),
        "3" => Ok(Difficulty::Hard),
        _ => Ok(Difficulty::Medium), // Default
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let difficulty = select_difficulty()?;

    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    crossterm::event::EnableMouseCapture;

    let mut game = Game::new(difficulty)?;

    // Game loop
    loop {
        // Handle events
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                    if game.game_over {
                        break;
                    }
                }
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    column: x,
                    row: y,
                    ..
                }) => {
                    if !game.game_over {
                        game.handle_click(x, y);
                    }
                }
                _ => {}
            }
        }

        if !game.game_over {
            game.spawn_target();
            game.update_targets();
        }

        game.draw()?;
    }

    // Cleanup
    execute!(stdout(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
