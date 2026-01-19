use crossterm::terminal::{self, Clear};
use crossterm::{cursor::MoveTo, execute, queue, style::Print};
use std::io::{self, Read, Stdout, Write};

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
    let name = "rw v0.0.0";
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

fn editor_end() -> io::Result<()> {
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let stdin = io::stdin(); // create input stream handler
    let mut stdout = io::stdout(); // create output stream handler
    let input = stdin.bytes(); // read input bytes

    editor_init(&mut stdout)?;

    let mut arr: Vec<char> = Vec::new(); // store the characters on the line

    for character in input {
        let b = character.unwrap();
        let c = b as char;

        if b == 3 {
            editor_end()?;
            break;
        } else if b == 127 {
            arr.pop();
        } else {
            arr.push(c);
        }

        execute!(
            stdout,
            Print("\r"),
            Clear(terminal::ClearType::UntilNewLine)
        )?;
        for thing in &arr {
            queue!(stdout, Print(thing))?;
        }
        stdout.flush()?;
    }
    Ok(())
}
