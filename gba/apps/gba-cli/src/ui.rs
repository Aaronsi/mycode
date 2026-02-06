use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

pub async fn run_tui(engine: gba_core::Engine) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, engine).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    engine: gba_core::Engine,
) -> Result<()> {
    let mut input = String::new();
    let mut messages: Vec<String> = vec!["Welcome to GBA TUI!".to_string()];

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(f.area());

            // Messages area
            let items: Vec<ListItem> = messages
                .iter()
                .map(|m| ListItem::new(Line::from(Span::raw(m))))
                .collect();

            let messages_list =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(messages_list, chunks[0]);

            // Input area
            let input_paragraph = Paragraph::new(input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Input (Esc to quit)"),
                );
            f.render_widget(input_paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    if !input.is_empty() {
                        messages.push(format!("> {}", input));
                        let result = engine.execute(&input).await?;
                        messages.push(format!("< {}", result));
                        input.clear();
                    }
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            }
        }
    }

    Ok(())
}
