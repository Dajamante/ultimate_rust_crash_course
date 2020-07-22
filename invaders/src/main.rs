use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use frame::Drawable;
use invaders::frame;
use invaders::{player::Player, render::render};
use rusty_audio::Audio;
use std::error::Error;
use std::io;
use std::{sync::mpsc, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");
    audio.wait();

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separate thread

    let (render_tx, render_rx) = mpsc::channel();
    let _render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, true);
            last_frame = curr_frame;
        }
    });
    // GAME LOOP
    let mut player = Player::new();

    'gameloop: loop {
        //per frame init
        let mut curr_frame = frame::new_frame();
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        //render section
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(2));
    }
    // CLEAN UP

    drop(render_tx);
    audio.wait();
    //Reverse of Terminal actions
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
