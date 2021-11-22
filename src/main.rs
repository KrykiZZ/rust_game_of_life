mod app;

use std::io::{Stdout, Write};
use terminal::{error, Action, Clear, Retrieved, Value, Terminal};
use array2d::Array2D;
use rand::Rng;
use std::{thread, time::Duration};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use crate::app::{TemplateApp, TemplateAppEvent};

fn get_neighbours(screen: &Array2D<i32>, x: u16, y: u16) -> i16 {
    let mut neighbours = 0;

    if (x != 0                                      && y != 0) && screen[((x - 1) as usize, (y - 1) as usize)] == 1 { neighbours += 1; }
    if  y != 0                                      &&            screen[(x as usize, (y - 1) as usize)]       == 1 { neighbours += 1; }
    if (x <= screen.num_rows() as u16               && y != 0) && screen[((x + 1) as usize, (y - 1) as usize)] == 1 { neighbours += 1; }

    if  x != 0                                      && screen[((x - 1) as usize, y as usize)]                  == 1 { neighbours += 1; }
    if  x <= screen.num_rows() as u16               && screen[((x + 1) as usize, y as usize)]                  == 1 { neighbours += 1; }

    if (x != 0 && y <= screen.num_columns() as u16) && screen[((x - 1) as usize, (y + 1) as usize)]            == 1 { neighbours += 1; }
    if  y <= screen.num_columns() as u16            && screen[(x as usize, (y + 1) as usize)]                  == 1 { neighbours += 1; }
    if (x <= screen.num_rows() as u16               && y <= screen.num_rows() as u16) && screen[((x + 1) as usize, (y + 1) as usize)] == 1 { neighbours += 1; }

    return neighbours;
}

fn fill_randomly(screen: &mut Array2D<i32>)
{
    for x in 0..screen.num_rows()
    {
        for y in 0..screen.num_columns()
        {
            if rand::thread_rng().gen_range(0..100) >= 50 {
                screen[(x, y)] = 1;
            }
        }
    }
}

fn get_alive(screen: &Array2D<i32>) -> i32 {
    let mut alive = 0;
    for x in 0..screen.num_rows()
    {
        for y in 0..screen.num_columns()
        {
            if screen[(x, y)] == 1 { alive += 1; }
        }
    }

    return alive;
}

fn render(terminal: &mut Terminal<Stdout>, screen: &Array2D<i32>) {
    for x in 0..screen.num_rows()
    {
        for y in 0..screen.num_columns()
        {
            terminal.batch(Action::MoveCursorTo(x as u16, y as u16)).unwrap();
            terminal.write(format!("{}", if screen[(x, y)] == 0 { " " } else { "*" }).as_bytes()).unwrap();
        }
    }
}

pub fn main() -> error::Result<()> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut terminal = terminal::stdout();
        if let Retrieved::TerminalSize(screen_x, screen_y) = terminal.get(Value::TerminalSize).unwrap()
        {
            terminal.act(Action::HideCursor).unwrap();

            let mut screen = Array2D::filled_with(0, screen_x as usize + 1, screen_y as usize + 1);
            fill_randomly(&mut screen);

            /*screen[(10, 10)] = 1;
            screen[(11, 10)] = 1;
            screen[(12, 10)] = 1;
            screen[(11, 9)] = 1;*/

            terminal.act(Action::ClearTerminal(Clear::All)).unwrap();

            let mut sleep_time = 0;
            let mut pause = true;

            while get_alive(&screen) > 0
            {
                // Островок имени Сахара
                while pause {
                    if let TemplateAppEvent::Pause{ state } = receiver.recv().unwrap() {
                        pause = state;
                    }

                    thread::sleep(Duration::from_millis(100));
                }

                if let Result::Ok(event) = receiver.try_recv() {
                    match event {
                        TemplateAppEvent::Sleep { time } => { sleep_time = time },
                        TemplateAppEvent::Pause { state } => { pause = state },
                        TemplateAppEvent::Restart => {
                            screen = Array2D::filled_with(0, screen_x as usize + 1, screen_y as usize + 1);

                            fill_randomly(&mut screen);
                            render(&mut terminal, &screen);
                        }
                    }
                }

                render(&mut terminal, &screen);

                if sleep_time > 0 { thread::sleep(Duration::from_millis(sleep_time)); }

                let mut new_screen = screen.clone();
                for x in 0..screen_x
                {
                    for y in 0..screen_y
                    {
                        let mut state = screen[(x as usize, y as usize)];
                        let neighbours = get_neighbours(&screen, x, y);

                        if state == 1 && (neighbours == 2 || neighbours == 3) { } // выживает
                        else if state == 1 && (neighbours > 3 || neighbours == 0 || neighbours == 1) { state = 0; }
                        else if state == 0 && neighbours == 3 { state = 1; }

                        new_screen[(x as usize, y as usize)] = state;
                    }
                }

                screen = new_screen;
            }


            terminal.flush_batch().unwrap();
        }
    });

    let app = TemplateApp::new(sender);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);

    Ok(())
}