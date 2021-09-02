use crossbeam::channel;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use rusty_audio::Audio;
use space_invaders::frame::{new_frame, Drawable};
use space_invaders::invaders::Invaders;
use space_invaders::player::Player;
use space_invaders::{frame, view};
use std::error::Error;
use std::time::{Duration, Instant};
use std::{io, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "assets/audio/explode.wav");
    audio.add("lose", "assets/audio/lose.wav");
    audio.add("move", "assets/audio/move.wav");
    audio.add("pew", "assets/audio/pew.wav");
    audio.add("startup", "assets/audio/startup.wav");
    audio.add("win", "assets/audio/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (sender, receiver) = channel::unbounded();
    let render_thread = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        view::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let next_frame = match receiver.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            view::render(&mut stdout, &last_frame, &next_frame, false);
            last_frame = next_frame;
        }
    });

    // Gameloop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // Init
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut next_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        player.move_left();
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        player.move_right();
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.try_to_shoot() {
                            audio.play("pew");
                        }
                    }
                    _ => {}
                }
            }
        }

        // Update
        player.update(delta);
        if invaders.try_to_move(delta) {
            audio.play("move");
        }

        // Draw
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut next_frame);
        }

        // Render
        let _ = sender.send(next_frame);
        thread::sleep(Duration::from_millis(1))
    }

    // Cleanup
    drop(sender);
    render_thread.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}
