use std::io::Write;
use terminal::{error, Action, Clear, Retrieved, Value};
use array2d::Array2D;
use rand::Rng;
use std::{thread, time::Duration};

fn get_neighbours(screen: &Array2D<i32>, x: u16, y: u16) -> i16 {
    let mut neighbours = 0;

    if (x != 0                                      && y != 0) && screen[((x - 1) as usize, (y - 1) as usize)] == 1 { neighbours += 1; }
    if  y != 0                                      &&            screen[(x as usize, (y - 1) as usize)]       == 1 { neighbours += 1; }
    if (x <= screen.num_rows() as u16               && y != 0) && screen[((x + 1) as usize, (y - 1) as usize)] == 1 { neighbours += 1; }

    if  x != 0                                      && screen[((x - 1) as usize, y as usize)]                == 1 { neighbours += 1; }
    if  x <= screen.num_rows() as u16               && screen[((x + 1) as usize, y as usize)]                == 1 { neighbours += 1; }

    if (x != 0 && y <= screen.num_columns() as u16) && screen[((x - 1) as usize, (y + 1) as usize)]            == 1 { neighbours += 1; }
    if  y <= screen.num_columns() as u16            && screen[(x as usize, (y + 1) as usize)]                == 1 { neighbours += 1; }
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

pub fn main() -> error::Result<()> {
    // thread::sleep(Duration::from_millis(5000));
    let sleep_time = 1000;
    let mut terminal = terminal::stdout();

    if let Retrieved::TerminalSize(screen_x, screen_y) = terminal.get(Value::TerminalSize)?
    {
        terminal.act(Action::HideCursor).unwrap();

        let mut screen = Array2D::filled_with(0, screen_x as usize + 1, screen_y as usize + 1);
        //fill_randomly(&mut screen);

        screen[(10, 10)] = 1;
        screen[(11, 10)] = 1;
        screen[(12, 10)] = 1;
        screen[(11, 9)] = 1;

        terminal.act(Action::ClearTerminal(Clear::All))?;

        while get_alive(&screen) > 0
        {
            // Show current state of screen.
            for x in 0..screen_x
            {
                for y in 0..screen_y
                {
                    terminal.batch(Action::MoveCursorTo(x, y))?;
                    terminal.write(format!("{}", if screen[(x as usize, y as usize)] == 0 { " " } else { "*" }).as_bytes()).unwrap();
                }
            }

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


        terminal.flush_batch()?;
    }


    /*terminal.act(Action::ClearTerminal(Clear::All))?;

    // batch multiple actions.
    for x in 0..screen_x
    {
        for y in 0..screen_y
        {
            terminal.batch(Action::MoveCursorTo(x, y))?;
            terminal.write(format!("{}", screen[(x as size, y as size)]).as_bytes());
        }
    }

    // execute batch.
    terminal.flush_batch();*/

    Ok(())
}