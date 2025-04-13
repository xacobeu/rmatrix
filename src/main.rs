use std::io::{self};
use rand::Rng;
use crossterm::{
    execute,
    cursor::{Show, Hide, MoveTo},
    terminal::{size, Clear, ClearType},
    style::{Print, SetForegroundColor, Color}
};

fn main() -> io::Result<()> {

    const ALL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()+={}[]:;<>?/";
    const STREAM_LENGTH: u16 = 10;

    let (cols, rows) = size()?;
    let mut rng = rand::rng();

    // Clear the screen start_positions
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Hide,
    )?;

    for i in 0..cols {

        if rng.random_range(0..=1) == 0 {
            continue;
        }

        for j in 0..rows {

            // make ch a random char
            let ch = ALL_CHARS.chars().nth(rng.random_range(0..ALL_CHARS.len())).unwrap();

            execute!(
                io::stdout(),
                MoveTo(i, j),
                SetForegroundColor(Color::Green),
                Print(ch),
            )?;

            if j >= STREAM_LENGTH {
                execute!(
                    io::stdout(),
                    MoveTo(i, j - STREAM_LENGTH),
                    SetForegroundColor(Color::Black),
                    Print(ch),
                )?;
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    execute!(
        io::stdout(),
        Show,
    )?;

    Ok(())
}