use std::num::ParseIntError;

use namt::{find_unsafe_number, Number};

fn main() -> Result<(), ParseIntError> {
    let numbers: Vec<Number> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let Some(unsafe_number) = find_unsafe_number(100, &numbers) else {
        println!("The tunnel is safe");
        return Ok(());
    };

    println!("The tunnel is about to crumble at: {unsafe_number}");

    Ok(())
}
