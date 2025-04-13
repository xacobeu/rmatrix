use std::io::{self};
use rand::Rng;
use crossterm::{
    execute,
    cursor::{Show, Hide, MoveTo},
    terminal::{size, Clear, ClearType},
    style::{Print, SetForegroundColor, Color}
};

const ALL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()+={}[]:;<>?/";
const STREAM_LENGTH: u16 = 10;

fn draw_stream(col: u16, rows: u16) -> Result<(), io::Error> {

    let mut rng = rand::rng();

    for row in 0..rows+STREAM_LENGTH {

        let ch = ALL_CHARS.chars().nth(rng.random_range(0..ALL_CHARS.len())).unwrap();

        execute!(
            io::stdout(),
            MoveTo(col, row),
            SetForegroundColor(Color::Green),
            Print(ch),
        )?;

        if row >= STREAM_LENGTH {
            execute!(
                io::stdout(),
                MoveTo(col, row - STREAM_LENGTH),
                Print(" "),
            )?;

        }

        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    
    let (cols, rows) = size()?;
    let mut rng = rand::rng();

    // Clear the screen start_positions
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Hide,
    )?;

    // Draw stream at random column.
    for _ in 0..cols {
        let col = rng.random_range(0..cols);
        draw_stream(col, rows)?;

    }

    execute!(
        io::stdout(),
        Show,
    )?;

    Ok(())
}