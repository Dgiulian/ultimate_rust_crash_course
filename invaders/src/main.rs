use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame /* Drawable, Frame */},
    render,
};

use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self /* Receiver */},
    time::Duration,
    {io, thread},
};

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

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);

            last_frame = curr_frame;
        }
    });

    // Game Loop
    'gameloop: loop {
        // Per-frame init
        let curr_frame = new_frame();

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
        // Draw and Render
        let _ = render_tx.send(curr_frame); // Ignoring the error

        thread::sleep(Duration::from_millis(1)); // Prevent generating thoutands of frames per seconds
    }

    // Cleanup
    drop(render_tx); // Drop the tx channel
    render_handle.join().unwrap(); // Wait until render thread finishes

    audio.wait();
    stdout.execute(Show)?; // Show the cursor
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
