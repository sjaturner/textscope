use std::io::stdout;

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    execute, style,
};

fn work() {
    // with macro
    if false {
        let _ = execute!(
            stdout(),
            SavePosition,
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            style::Print("\n"),
            //      MoveTo(10, 10),
            style::Print("Cheers you fuckers"),
            EnableBlinking,
            DisableBlinking,
            RestorePosition,
            style::Print("\n"),
        );
    }
    stdout().execute(MoveTo(11, 11)).unwrap().execute(RestorePosition);

    //  stdout().execute(MoveTo(11, 11))?.execute(RestorePosition);
}

fn main() {
    let _ = work();
}
