use crossterm::{
    cursor::{Hide, MoveTo, Show}, event::{poll, read, Event, KeyCode}, execute, style::{
        Attribute::{Bold, Dim}, Color, Print, ResetColor, SetAttribute, SetForegroundColor
    }, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}
};
use rand::Rng;
use std::{collections::VecDeque, io::{self, stdout, Write}, time::Duration};

// CONSTANTS
const ALL_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()+={}[]:;<>?/";

const MIN_STREAM_LEN: usize = 5;
const MAX_STREAM_LEN: usize = 10;
const STREAM_SPAWN_PROBABILITY: f32 = 1.0;

const FRAME_DELAY_MS: u64 = 30; // Milliseconds between frames

// STRUCTS
struct Stream {
    col: u16,
    y: u16,
    max_len: usize,
    chars: VecDeque<char>,
    is_dying: bool,
}

impl Stream {
    fn new(col: u16) -> Self {
        let mut rng = rand::rng();
        let max_len = rng.random_range(MIN_STREAM_LEN..=MAX_STREAM_LEN);
        Stream {
            col,
            y: 0, // Start just above the screen
            max_len,
            chars: VecDeque::with_capacity(max_len),
            is_dying: false,
        }
    }

    fn update(&mut self, screen_height: u16) {
        if self.is_dying {
            return;
        }

        let mut rng = rand::rng();

                // Add new character to the head
        let new_char = ALL_CHARS
            .chars()
            .nth(rng.random_range(0..ALL_CHARS.len()))
            .unwrap_or(' '); // Default to space if something goes wrong
        
        self.chars.push_front(new_char);

        // Remove oldest character if max length reached
        if self.chars.len() > self.max_len {
            self.chars.pop_back();
        }

        // Advance position
        self.y += 1;

        // Deactivate if the *tail* has gone past the bottom
        let tail_y = self.y - self.chars.len() as u16;
        if tail_y >= screen_height as u16 {
            self.is_dying = true;
        }
    }

    fn draw(&self, stdout: &mut io::Stdout, screen_height: u16) -> Result<(), std::io::Error> {

        if self.is_dying {
            return Ok(());
        }

        for (i, &ch) in self.chars.iter().enumerate() {
            let current_y = self.y - i as u16;
            if current_y >= screen_height{ // Skip if above or under screen
                continue;
            }

            // Style: Brightest head, then normal, then maybe dim tail
            let (color, attribute) = if i == 0 {
                (Color::White, Bold) // Bright white head
            } else if i < self.chars.len() / 2 {
                 (Color::Green, Bold) // Normal green middle
            } else {
                (Color::Green, Dim) // Dimmer green tail
                // Or use Attribute::Dim with Color::Green if preferred
                // (Color::Green, Dim)
            };

            execute!(
                stdout,
                MoveTo(self.col, current_y),
                SetForegroundColor(color),
                SetAttribute(attribute),
                Print(ch),
                ResetColor // Reset color too
            )?;
        }

        // Erase the character just behind the tail
        let erase_y = self.y - self.chars.len() as u16;
        if erase_y < screen_height {
            
            execute!(stdout, MoveTo(self.col, erase_y), Print(" "))?;
            
        }

        Ok(())
    }
}

// Clear the screen start_positions
fn clear_screen() -> Result<(), io::Error> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0), Hide,)?;
    Ok(())
}

fn restore_cursor() -> Result<(), io::Error> {
    execute!(stdout(), Show, ResetColor, Clear(ClearType::All), MoveTo(0,0))?;
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let (mut cols, mut rows) = size()?;
    let mut rng = rand::rng();
    let mut stdout = stdout();
    
    let mut streams: Vec<Stream> = (0..cols).map(|c| Stream {
        col: c,
        y: 0,
        max_len: 0, // Will be set when activated
        chars: VecDeque::new(),
        is_dying: false,
    }).collect();

    clear_screen()?;

    loop {
        // --- Event Handling (check for resize or quit) ---
        if poll(Duration::from_millis(0))? { // Check if event is available without blocking
             match read()? {
                Event::Key(event) => {
                    // Quit on control + c
                    if  event.modifiers == crossterm::event::KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                        break;
                    }
                    // Quit on escape
                    if event.code == KeyCode::Esc {
                        break;
                    }
                    // Consider adding Ctrl+C handling via `ctrlc` crate for robustness
                },
                Event::Resize(new_cols, new_rows) => {
                    cols = new_cols;
                    rows = new_rows;
                    execute!(stdout, Clear(ClearType::All))?; // Clear on resize
                    // Resize stream vector (simplistic: just recreate)
                    streams = (0..cols).map(|c| Stream {
                        col: c, y: 0, max_len: 0, chars: VecDeque::new(), is_dying: true
                    }).collect();
                },
                _ => {} // Ignore other events
            }
        }

        for stream in streams.iter_mut() {
            if !stream.is_dying {
                stream.update(rows);
                stream.draw(&mut stdout, rows)?;
            } else {
                if rng.random::<f32>() < STREAM_SPAWN_PROBABILITY / (cols as f32) { // Lower probability per column
                    *stream = Stream::new(stream.col); // Activate it!
                    // Draw the newly activated stream immediately
                    stream.draw(&mut stdout, rows)?;
                }
            }
        }

        stdout.flush()?;

        // Frame delay
        std::thread::sleep(Duration::from_millis(FRAME_DELAY_MS));
    }

    restore_cursor()?;
    disable_raw_mode()?;
    
    Ok(())
}
