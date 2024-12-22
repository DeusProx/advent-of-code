use std::io::Write;
use crossterm::{cursor, event, execute, queue, style, terminal};
use days::day15::{Robot, Warehouse};

fn main() -> std::io::Result<()>{
    let mut stdout = std::io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let input = std::fs::read_to_string("../data/2024/day/15/input").expect("Cannot read input");
    // let input = std::fs::read_to_string("../data/2024/day/15/testinput").expect("Cannot read input");
    // let input = std::fs::read_to_string("../data/2024/day/15/testinput2").expect("Cannot read input");

    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut warehouse = Warehouse::parse(map);

    warehouse.expand();

    let mut robot = Robot::parse(moves, warehouse.find_robot());
    let mut moves = robot.moves.clone().into_iter();

    loop {
        let map: Vec<String> = warehouse.printable().collect();

        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in map.iter() {
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        stdout.flush()?;

        let mut move_handle = |mov: Option<char>| {
            match mov {
                None => (),
                Some(mov) => robot.walk(&mut warehouse, mov),
            }
        };
        match read_char()? {
            'q' => break,
            'w' => move_handle(Some('^')),
            's' => move_handle(Some('v')),
            'a' => move_handle(Some('<')),
            'd' => move_handle(Some('>')),
            'n' => move_handle(moves.next()),
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

