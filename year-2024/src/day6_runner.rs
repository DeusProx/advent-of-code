use std::io::Write;
use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, queue, style, terminal::{self, ClearType}};
use aoc_days::day6::Guard;


fn main() -> std::io::Result<()>{
    let mut stdout = std::io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let input = std::fs::read_to_string("../data/2024/day/6/input").expect("Cannot read input");
    let mut guard = Guard::parse(&input);

    loop {
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in guard.lab.to_string().split('\n') {
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        stdout.flush()?;

        match read_char()? {
            'q' => break,
            'n' => { guard.walk().unwrap(); },
            _ => continue,
        }
    }

    terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read() {
            return Ok(c);
        }
    }
}
