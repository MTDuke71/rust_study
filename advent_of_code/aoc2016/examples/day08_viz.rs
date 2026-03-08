//! Interactive step-through visualization for Day 8: Two-Factor Authentication
//!
//! Navigate through instructions with arrow keys and watch the screen update.
//!
//! Controls:
//!   Right / Enter  — advance one step
//!   Left           — go back one step
//!   Home           — jump to beginning
//!   End            — jump to end
//!   Space          — toggle auto-play
//!   +/-            — adjust auto-play speed
//!   q / Escape     — quit

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    terminal::{self, ClearType},
};
use mission6::Grid;
use std::io::{self, Write};
use std::time::Duration;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn main() -> io::Result<()> {
    let input = include_str!("../inputs/day08.txt");
    let instructions: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let total = instructions.len();

    // Pre-compute all screen states so we can scrub freely
    let states = build_states(&instructions);

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(ClearType::All)
    )?;

    let mut pos: usize = 0; // 0 = initial blank, 1..=total = after instruction N
    let mut auto_play = false;
    let mut speed_ms: u64 = 100;

    render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;

    loop {
        let timeout = if auto_play {
            Duration::from_millis(speed_ms)
        } else {
            Duration::from_millis(500) // just for responsiveness
        };

        if event::poll(timeout)? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => break,

                    KeyCode::Right | KeyCode::Enter => {
                        auto_play = false;
                        if pos < total {
                            pos += 1;
                            render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                        }
                    }

                    KeyCode::Left => {
                        auto_play = false;
                        if pos > 0 {
                            pos -= 1;
                            render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                        }
                    }

                    KeyCode::Home => {
                        auto_play = false;
                        pos = 0;
                        render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                    }

                    KeyCode::End => {
                        auto_play = false;
                        pos = total;
                        render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                    }

                    KeyCode::Char(' ') => {
                        auto_play = !auto_play;
                        render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                    }

                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        speed_ms = speed_ms.saturating_sub(20).max(10);
                        render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                    }

                    KeyCode::Char('-') => {
                        speed_ms = (speed_ms + 20).min(1000);
                        render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
                    }

                    _ => {}
                }
            }
        } else if auto_play && pos < total {
            pos += 1;
            render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
            if pos == total {
                auto_play = false;
                render_frame(&mut stdout, &states[pos], pos, total, &instructions, auto_play, speed_ms)?;
            }
        }
    }

    // Cleanup terminal
    execute!(
        stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn build_states(instructions: &[&str]) -> Vec<Grid<bool>> {
    let mut states = Vec::with_capacity(instructions.len() + 1);
    let mut screen = Grid::new(WIDTH, HEIGHT, false);
    states.push(screen.clone());

    for &inst in instructions {
        apply(&mut screen, inst);
        states.push(screen.clone());
    }

    states
}

fn apply(screen: &mut Grid<bool>, instruction: &str) {
    if let Some(rest) = instruction.strip_prefix("rect ") {
        let (w, h) = rest.split_once('x').unwrap();
        let w: usize = w.parse().unwrap();
        let h: usize = h.parse().unwrap();
        for y in 0..h {
            for x in 0..w {
                screen[(x, y)] = true;
            }
        }
    } else if let Some(rest) = instruction.strip_prefix("rotate row y=") {
        let (y, amount) = rest.split_once(" by ").unwrap();
        let y: usize = y.parse().unwrap();
        let amount: usize = amount.parse().unwrap();
        let old: Vec<bool> = (0..WIDTH).map(|x| screen[(x, y)]).collect();
        for x in 0..WIDTH {
            screen[((x + amount) % WIDTH, y)] = old[x];
        }
    } else if let Some(rest) = instruction.strip_prefix("rotate column x=") {
        let (x, amount) = rest.split_once(" by ").unwrap();
        let x: usize = x.parse().unwrap();
        let amount: usize = amount.parse().unwrap();
        let old: Vec<bool> = (0..HEIGHT).map(|y| screen[(x, y)]).collect();
        for y in 0..HEIGHT {
            screen[(x, (y + amount) % HEIGHT)] = old[y];
        }
    }
}

fn render_frame(
    stdout: &mut io::Stdout,
    screen: &Grid<bool>,
    pos: usize,
    total: usize,
    instructions: &[&str],
    auto_play: bool,
    speed_ms: u64,
) -> io::Result<()> {
    queue!(stdout, cursor::MoveTo(0, 0))?;

    let lit = screen.iter().filter(|&&p| p).count();
    let step_label = if pos == 0 {
        "Initial".to_string()
    } else {
        format!("Step {}/{}", pos, total)
    };

    // Header
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    write!(
        stdout,
        "\x1b[1;36m Day 8: Two-Factor Authentication \x1b[0m  {step_label}\r\n\r\n"
    )?;

    // Screen with row numbers on the left
    for y in 0..HEIGHT {
        queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
        write!(stdout, "  \x1b[38;5;240m{y}\x1b[0m ")?;
        for x in 0..WIDTH {
            if screen[(x, y)] {
                write!(stdout, "\x1b[1;33m\u{2588}\u{2588}\x1b[0m")?;
            } else {
                write!(stdout, "\x1b[38;5;236m\u{00b7} \x1b[0m")?;
            }
        }
        write!(stdout, "\r\n")?;
    }

    // Column numbers below (tens row, then ones row)
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    write!(stdout, "    ")?; // align with grid (past "  N ")
    for x in 0..WIDTH {
        if x % 5 == 0 {
            write!(stdout, "\x1b[38;5;240m{:<2}\x1b[0m", x)?;
        } else {
            write!(stdout, "  ")?;
        }
    }
    write!(stdout, "\r\n")?;

    // Info
    write!(stdout, "\r\n")?;
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    write!(stdout, "  \x1b[1;32mPixels lit: {lit}\x1b[0m\r\n")?;

    // Previous instruction (what got us here)
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    if pos >= 2 {
        write!(
            stdout,
            "  \x1b[38;5;240m  \u{2190} prev:  {}\x1b[0m\r\n",
            instructions[pos - 2]
        )?;
    } else {
        write!(stdout, "\r\n")?;
    }

    // Current instruction (just applied)
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    if pos > 0 {
        write!(
            stdout,
            "  \x1b[1;33m  \u{25b6} applied: {}\x1b[0m\r\n",
            instructions[pos - 1]
        )?;
    } else {
        write!(stdout, "  \x1b[90m  \u{25cb} (initial state)\x1b[0m\r\n")?;
    }

    // Next instruction (what will happen on Right arrow)
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    if pos < total {
        write!(
            stdout,
            "  \x1b[36m  \u{2192} next:  {}\x1b[0m\r\n",
            instructions[pos]
        )?;
    } else {
        write!(stdout, "  \x1b[32m  \u{2713} (complete)\x1b[0m\r\n")?;
    }

    // Progress bar
    write!(stdout, "\r\n")?;
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    let bar_width = 50;
    let filled = if total > 0 { pos * bar_width / total } else { 0 };
    write!(stdout, "  \x1b[36m[")?;
    for i in 0..bar_width {
        if i < filled {
            write!(stdout, "\u{2588}")?;
        } else if i == filled {
            write!(stdout, "\x1b[1;33m\u{25b6}\x1b[36m")?;
        } else {
            write!(stdout, "\x1b[38;5;236m\u{2500}\x1b[36m")?;
        }
    }
    write!(stdout, "\x1b[36m]\x1b[0m\r\n")?;

    // Auto-play indicator
    write!(stdout, "\r\n")?;
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    if auto_play {
        write!(
            stdout,
            "  \x1b[1;32m\u{25b6} Playing\x1b[0m  \x1b[90m({speed_ms}ms/step)\x1b[0m\r\n"
        )?;
    } else {
        write!(stdout, "  \x1b[90m\u{23f8} Paused\x1b[0m\r\n")?;
    }

    // Controls
    queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;
    write!(
        stdout,
        "  \x1b[90m\u{2190}/\u{2192} step  Space auto  +/- speed  Home/End jump  q quit\x1b[0m\r\n"
    )?;

    stdout.flush()?;
    Ok(())
}
