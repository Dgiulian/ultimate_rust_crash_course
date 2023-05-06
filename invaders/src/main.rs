use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use rusty_audio::Audio;
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./audio/explode.wav");
    audio.add("lose", "./audio/lose.wav");
    audio.add("move", "./audio/move.wav");
    audio.add("pew", "./audio/pew.wav");
    audio.add("startup", "./audio/startup.wav");
    audio.add("win", "./audio/win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();

    // Enable raw mode so we can get access to keyboard
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?; // Hide the cursor

    // Game Loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    audio.wait();
    stdout.execute(Show)?; // Show the cursor
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
