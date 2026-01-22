use crossterm::cursor::{self, MoveTo, RestorePosition, SavePosition};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, Clear};
use crossterm::{queue, style::Print};
use std::io::{self, Stdout, Write};

enum EditorModes {
    Normal,
    Typing,
}

fn clear_screen(stdout: &mut Stdout) -> io::Result<()> {
    queue!(stdout, terminal::Clear(terminal::ClearType::All),)?;
    Ok(())
}

fn insert_tildes(stdout: &mut Stdout) -> io::Result<()> {
    let window_height = terminal::size()?.0;
    for i in 1..window_height - 1 {
        queue!(stdout, MoveTo(0, i), Print("~"))?;
    }
    Ok(())
}

fn print_name(stdout: &mut Stdout) -> io::Result<()> {
    let name = "redwrite v0.0.0 (idk i can't think of a name)";
    let name_x_pos = terminal::size()?.0 / 2 - name.len() as u16 / 2;
    let name_y_pos = terminal::size()?.1 - terminal::size()?.1 / 3;

    queue!(stdout, MoveTo(name_x_pos, name_y_pos), Print(name))?;
    Ok(())
}

fn editor_init(stdout: &mut Stdout) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    clear_screen(stdout)?;
    insert_tildes(stdout)?;
    print_name(stdout)?;
    display_bottom_bar(&EditorModes::Typing, stdout)?; // default is Typing
    queue!(stdout, MoveTo(0, 0))?;
    stdout.flush()?;
    Ok(())
}

fn editor_end(stdout: &mut Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    clear_screen(stdout)?;
    Ok(())
}

fn display_bottom_bar(mode: &EditorModes, stdout: &mut Stdout) -> io::Result<()> {
    let last_row = terminal::size()?.1;
    let cursor_pos_x = cursor::position()?.0;
    let cursor_pos_y = cursor::position()?.1;
    let mode_str = match mode {
        EditorModes::Normal => "Normal",
        EditorModes::Typing => "Typing",
    };
    queue!(
        stdout,
        SavePosition,
        MoveTo(0, last_row),
        Clear(terminal::ClearType::UntilNewLine),
        Print(format!(
            "{mode_str} | cursor ({cursor_pos_x}, {cursor_pos_y})"
        )),
        RestorePosition,
    )?;
    Ok(())
}

fn draw_screen(mode: &EditorModes, stdout: &mut Stdout) -> io::Result<()> {
    // queue the carriage and reset for current line
    queue!(
        stdout,
        // Print("\r"),
        Clear(terminal::ClearType::UntilNewLine)
    )?;

    // printing everything on the stored line vec
    // for key in arr {
    //     queue!(stdout, Print(key))?;
    // }

    display_bottom_bar(mode, stdout)?;
    stdout.flush()?; // execute the queued changes
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout(); // create output stream handler
    editor_init(&mut stdout)?;
    // let mut arr: Vec<char> = Vec::new(); // store the characters on the line

    // default mode is typing
    let mut editor_mode = EditorModes::Typing;

    loop {
        let event = match crossterm::event::read() {
            Ok(e) => e,
            Err(_) => {
                println!("read was not successful.");
                break;
            }
        };

        if let Event::Key(key_event) = event {
            // instantly terminate if prompted
            if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL
            {
                editor_end(&mut stdout)?;
                break;
            }

            // change behavior based on 'modes'
            match editor_mode {
                EditorModes::Normal => match key_event.code {
                    KeyCode::Char('i') => editor_mode = EditorModes::Typing,
                    KeyCode::Char('h') => queue!(stdout, cursor::MoveLeft(1))?,
                    KeyCode::Char('j') => queue!(stdout, cursor::MoveDown(1))?,
                    KeyCode::Char('k') => queue!(stdout, cursor::MoveUp(1))?,
                    KeyCode::Char('l') => queue!(stdout, cursor::MoveRight(1))?,
                    _ => (),
                },
                EditorModes::Typing => match key_event.code {
                    KeyCode::Esc => editor_mode = EditorModes::Normal,
                    KeyCode::Backspace => (),
                    KeyCode::Char(_) => (),
                    _ => (),
                },
            }
        }

        draw_screen(&editor_mode, &mut stdout)?;
    }

    Ok(())
}
