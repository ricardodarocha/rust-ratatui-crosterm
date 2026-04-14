use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;
use ratatui::prelude::*;
use ratatui::Terminal;
use ratatui::widgets::Paragraph;
use clap::{Parser, CommandFactory};

/// About documentation starts here  
/// 
/// see https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html  
/// 
/// https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args { 
    /// Name of the person to greet
    #[arg(short, long, value_name = "Anonymous")]
    name: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    if std::env::args().len() == 1 {
        Args::command().print_help().unwrap();
        println!();
        std::process::exit(0);
    }

    // todo! Do Somethin with args
    let args = Args::parse();

    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    Ok(loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Teste RATATUI is Running 🐀!\npress q to quit");
            frame.render_widget(greeting, frame.area());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
            
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('m') == key.code {
                     terminal.show_cursor()?;
                }
            }
        }
    })
}