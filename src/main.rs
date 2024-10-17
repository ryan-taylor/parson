use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

const OPTIONS: [&str; 10] = [
    "Fast Reading",
    "Data Extraction",
    "Data Validation",
    "File Compression",
    "Multi-File Processing",
    "Custom Data Types",
    "Error Checking",
    "Pretty Printing",
    "Multi-Language Support",
    "Speed Optimization",
];

fn main() -> crossterm::Result<()> {
    let mut selected = 0;
    loop {
        display_menu(selected)?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Up => selected = (selected - 1 + OPTIONS.len()) % OPTIONS.len(),
                KeyCode::Down => selected = (selected + 1) % OPTIONS.len(),
                KeyCode::Enter => {
                    // Handle option selection here
                    println!("Selected: {}", OPTIONS[selected]);
                    break;
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    execute!(stdout(), Show)?; // Show cursor before exiting
    Ok(())
}

fn display_menu(selected: usize) -> crossterm::Result<()> {
    execute!(stdout(), Clear(ClearType::All), Hide)?;
    println!("Parson - Purify your JSON");
    println!("Use ↑ and ↓ arrows to move, Enter to select, 'q' to quit\n");

    for (index, option) in OPTIONS.iter().enumerate() {
        if index == selected {
            println!("→ {}", option);
        } else {
            println!("  {}", option);
        }
    }
    stdout().flush()?;
    Ok(())
}
