use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::event;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use invaders::frame;
use invaders::render;
use rusty_audio::Audio;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use std::io::{self};
use invaders::frame::{Drawable, new_frame};
use invaders::player::Player;


fn main() -> Result<(), Box<dyn Error>> {

    let mut audio = Audio::new();

    audio.add("explode","./sounds/explode.wav");
    audio.add("lose","./sounds/lose.wav");
    audio.add("move","./sounds/move.wav");
    audio.add("pew","./sounds/pew.wav");
    audio.add("startup","./sounds/startup.wav");
    audio.add("win","./sounds/win.wav");
    audio.play("startup");


    //Render loop
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

            // let curr_match render_rx.recv() {
            //     Ok(x) => x,
            //     Err(_) => break,
            // }
        }
    });



    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        //Per-frame
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        //input
        while event::poll(Duration::default())? {
            if let event::Event::Key(event) = event::read()? {
                match event.code {
                    event::KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    event::KeyCode::Left => player.move_left(),
                    event::KeyCode::Right => player.move_right(),
                    event::KeyCode::Char('a') => {
                        audio.play("move");
                        player.move_left()
                    }
                    event::KeyCode::Char(' ') | event::KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    event::KeyCode::Char('d') => {
                        audio.play("explode");
                    }
                    event::KeyCode::Char('f') => {
                        audio.play("lose");
                    }
                    event::KeyCode::Char('g') => {
                        audio.play("win");
                    }
                    _ => {}
                }
            }
        }
        // Updates


        //Draw & render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //cleanup thread
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //cleanup 
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())   
}
