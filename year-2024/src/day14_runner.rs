use std::io::Write;
use crossterm::{cursor, event, execute, queue, style, terminal};
use aoc_days::day14::{Robot, Size};

fn main() -> std::io::Result<()>{
    let mut stdout = std::io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let input = std::fs::read_to_string("../data/2024/day/14/input").expect("Cannot read input");
    let size = Size { width: 101, height: 103 };
    let robots: Vec<Robot> = input.lines()
        .map(|line| Robot::parse(line, &size))
        .collect();

    let mut dt = 6446;

    loop {
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        queue!(
            stdout,
            cursor::MoveToNextLine(2),
            cursor::MoveRight(40),
            style::Print(format!("Current Time: {}", dt)),
            cursor::MoveToNextLine(2),
        )?;

        let map: Vec<char> = robots.clone()
            .iter()
            .map(|robot| robot.teleport(&size, dt))
            .fold(vec!['.'; (size.width * size.height) as usize], |mut map, position| {
                map[position.index(&size)] = '#';
                map
            });

        for line in map.chunks(size.width as usize) {
            let line: String = line.iter().collect();
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        stdout.flush()?;

        match read_char()? {
            'q' => break,
            'f' => dt += 1,
            'b' => dt -= 1,
            _ => continue,
        }
    }

    terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(event::Event::Key(event::KeyEvent {
            code: event::KeyCode::Char(c),
            kind: event::KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read() {
            return Ok(c);
        }
    }
}

