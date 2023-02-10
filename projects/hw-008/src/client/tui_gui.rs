use std::borrow::BorrowMut;
use std::error::Error;
use std::io;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table};
use tui::{Frame, Terminal};
use unicode_width::UnicodeWidthStr;

use crate::app::{ApplicationState, SelectedTable};
use crate::commands::ClientCommand;

pub fn run(
    application_state: Arc<Mutex<ApplicationState>>,
    commands_sender: Sender<ClientCommand>,
) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(500);
    let result = run_app(&mut terminal, application_state, commands_sender, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<ApplicationState>>,
    commands_sender: Sender<ClientCommand>,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw(f, app.clone()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let mut app = app.lock().unwrap();

                match key.code {
                    KeyCode::Enter => {
                        let command = app.current_command.clone();
                        app.current_command.clear();
                        app.executed_commands.push(command.clone());
                        commands_sender
                            .send(ClientCommand::ExecuteCommand(command))
                            .unwrap()
                    }
                    KeyCode::Char(c) => {
                        app.current_command.push(c);
                    }
                    KeyCode::Backspace => {
                        app.current_command.pop();
                    }
                    KeyCode::Tab => {
                        let selected = app.current_selected_table;
                        app.current_selected_table = selected.next();
                    }
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Down => {
                        app.highlight_next();
                        let event_type = match app.current_selected_table {
                            SelectedTable::Homes => ClientCommand::GetHomeInfo,
                            SelectedTable::Rooms => ClientCommand::GetRoomInfo,
                            SelectedTable::Devices => ClientCommand::GetDeviceInfo,
                        };
                        commands_sender.send(event_type).unwrap();
                    }
                    KeyCode::Up => {
                        app.highlight_previous();
                        let event_type = match app.current_selected_table {
                            SelectedTable::Homes => ClientCommand::GetHomeInfo,
                            SelectedTable::Rooms => ClientCommand::GetRoomInfo,
                            SelectedTable::Devices => ClientCommand::GetDeviceInfo,
                        };
                        commands_sender.send(event_type).unwrap();
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn build_table_widget<'a>(name: &'a str, values: &'a [String]) -> Table<'a> {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = values.iter().map(|item| {
        let cells = Cell::from(item.clone());
        Row::new(vec![cells])
    });

    Table::new(rows)
        .block(Block::default().title(name).borders(Borders::ALL))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ])
}

fn build_tui_list_widget<'a>(name: &'a str, values: &'a [String]) -> List<'a> {
    let responses: Vec<ListItem> = values
        .iter()
        .map(|line| {
            let content = vec![Spans::from(Span::raw(line))];
            ListItem::new(content)
        })
        .collect();

    List::new(responses).block(Block::default().borders(Borders::ALL).title(name))
}

fn draw<B: Backend>(f: &mut Frame<B>, app_lock: Arc<Mutex<ApplicationState>>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(3),
                Constraint::Percentage(37),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(f.size());

    let msg = vec![
        Span::raw("Press "),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit, "),
        Span::styled("TAB", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to move on entity tables, "),
        Span::styled("help", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to print help information, "),
        Span::styled(
            "UP (↑) / DOWN (↓)",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to select element from the table "),
    ];
    let text = Text::from(Spans::from(msg));
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let tables = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25), // Homes table
                Constraint::Percentage(25), // Rooms table
                Constraint::Percentage(25), // Device table
                Constraint::Percentage(25), // Info table
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let mut app = app_lock.lock().unwrap();
    let homes_list = app.homes.clone();
    let rooms_list = app.rooms.clone();
    let devices_list = app.devices.clone();
    let last_info = app.last_info.clone();

    let homes = build_table_widget("Homes", &homes_list);
    let rooms = build_table_widget("Rooms", &rooms_list);
    let devices = build_table_widget("Devices", &devices_list);
    let status = build_table_widget("Status", &last_info);

    let home_select_state = app.homes_table_select_state.borrow_mut();
    f.render_stateful_widget(homes, tables[0], home_select_state);

    let room_select_state = app.rooms_table_select_state.borrow_mut();
    f.render_stateful_widget(rooms, tables[1], room_select_state);

    let device_select_state = app.devices_table_select_state.borrow_mut();
    f.render_stateful_widget(devices, tables[2], device_select_state);

    f.render_widget(status, tables[3]);

    let interaction = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40), // Command input
                Constraint::Percentage(60), // Results output
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    let commands_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(90), // Commands log
                Constraint::Percentage(10), // Commands input
            ]
            .as_ref(),
        )
        .split(interaction[0]);

    let commands = build_tui_list_widget("Commands", &app.executed_commands);
    f.render_widget(commands, commands_layout[0]);

    let input = Paragraph::new(app.current_command.clone())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, commands_layout[1]);
    f.set_cursor(
        // Put cursor past the end of the input text
        commands_layout[1].x + app.current_command.width() as u16 + 1,
        // Move one line down, from the border to the input line
        commands_layout[1].y + 1,
    );

    let responses = build_tui_list_widget("Responses", &app.last_response);
    f.render_widget(responses, interaction[1]);
}
