use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Drawable /* Drawable, Frame */},
    invaders::Invaders,
    player::Player,
    render,
};

use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self /* Receiver */},
    time::{Duration, Instant},
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

    // Player
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    // Game Loop
    'gameloop: loop {
        // Per-frame init

        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew")
                        }
                    }
                    _ => {}
                }
            }
        }
        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // We could call the draw
        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame)
        }

        // Draw and Render
        let _ = render_tx.send(curr_frame); // Ignoring the error

        thread::sleep(Duration::from_millis(1)); // Prevent generating thoutands of frames per seconds

        // Win or Loose
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
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
