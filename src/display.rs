use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

/// Display the found [ClState](crate::clinfo::ClState)
pub fn display_opencl_state(cl_state: &crate::clinfo::ClState) -> std::io::Result<()> {
    let platforms = cl_state.get_platforms();
    println!("Found {} platforms", platforms.len());
    for platform in platforms.iter() {
        let devices = platform.devices();
        println!("{} with {} devices", platform.name(), devices.len());
        for device in devices {
            println!("      {}", device.name());
        }
    }

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let ui_drawer = |frame: &mut Frame| {
        frame.render_widget(
            Block::new().borders(Borders::TOP).title("opencl3_select"),
            frame.size(),
        );
        let list = ratatui::widgets::List::new(platforms.iter().map(|platform| platform.name()))
            .block(
                Block::default()
                    .title("opencl3_select")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        frame.render_widget(list, frame.size());
    };
    while !should_quit {
        terminal.draw(ui_drawer)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[1]);
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Left"),
        inner_layout[0],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Right"),
        inner_layout[1],
    );
}
