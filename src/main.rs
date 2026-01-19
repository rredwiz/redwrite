use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, Clear};
use crossterm::{cursor::MoveTo, execute, queue, style::Print};
use std::io::{self, Stdout, Write};

fn clear_screen(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All),)?;
    Ok(())
}

fn insert_tildes(stdout: &mut Stdout) -> io::Result<()> {
    let window_height = terminal::size()?.0;
    for i in 1..window_height {
        execute!(stdout, MoveTo(0, i), Print("~"))?;
    }
    Ok(())
}

fn print_name(stdout: &mut Stdout) -> io::Result<()> {
    let name = "redwrite v0.0.0 (idk i can't think of a name)";
    let name_x_pos = terminal::size()?.0 / 2 - name.len() as u16 / 2;
    let name_y_pos = terminal::size()?.1 - terminal::size()?.1 / 3;

    execute!(stdout, MoveTo(name_x_pos, name_y_pos), Print(name))?;
    Ok(())
}

fn editor_init(stdout: &mut Stdout) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    clear_screen(stdout)?;
    insert_tildes(stdout)?;
    print_name(stdout)?;
    execute!(stdout, MoveTo(0, 0))?;
    Ok(())
}

fn editor_end(stdout: &mut Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    clear_screen(stdout)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout(); // create output stream handler

    editor_init(&mut stdout)?;

    let mut arr: Vec<char> = Vec::new(); // store the characters on the line

    loop {
        let event = match crossterm::event::read() {
            Ok(e) => e,
            Err(_) => {
                dbg!("read was not successful.");
                break;
            }
        };

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Backspace => {
                    arr.pop();
                }
                KeyCode::Char('c') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        editor_end(&mut stdout)?;
                        break;
                    }
                }
                KeyCode::Char(some) => {
                    arr.push(some);
                }
                _ => (),
            }
        }

        clear_screen(&mut stdout)?;
        for key in &arr {
            queue!(stdout, Print(key))?;
        }
        stdout.flush()?;
    }
    Ok(())
}
