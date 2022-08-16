use std::io::{stdout, Write};

use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};

fn main() -> Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here.\n"))?;

    stdout()
        .execute(Print("Styled text here.\n"))?
        .execute(Print("Styled text here.\n"))?
        .execute(Print("Styled text here.\n"))?
        .execute(Print("Styled text here.\n"))?
        .execute(ResetColor)?;

    Ok(())
}
