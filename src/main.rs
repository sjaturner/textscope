use std::io::{stdout, Write};

use crossterm::{
    event, 
    execute,
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, 
    Result,
};

fn main() -> Result<()> {
    for _ in 1..=10 {
        stdout()
            .execute(Print("\n"))?;
    }
    stdout()
        .execute(cursor::SavePosition)?
        .execute(cursor::MoveUp(3))?
        .execute(Print("Hello"))?
        .execute(cursor::RestorePosition)?;

    Ok(())
}
