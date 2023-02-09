use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders};
use tui::widgets::{Cell, List, ListItem, Row, Table};
use tui::{Frame, Terminal};

use crate::app::App;
use crate::clients::TcpClient;
use crate::clients::UdpClient;

pub fn run(tcp_client: TcpClient, udp_client: UdpClient) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(500);
    let app = App::new(tcp_client, udp_client);
    let result = run_app(&mut terminal, app, tick_rate);

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
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn build_table_widget(name: &str, values: Vec<String>) -> Table {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = values.iter().map(|item| {
        let height = item.chars().filter(|c| *c == '\n').count() + 1;
        let cells = Cell::from(item.clone());
        Row::new(vec![cells]).height(height as u16).bottom_margin(1)
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

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
        .split(chunks[0]);

    let homes = build_table_widget("Homes", vec!["Home1".to_string()]);
    let rooms = build_table_widget("Rooms", vec!["Room1".to_string()]);
    let devices = build_table_widget("Devices", vec!["Device 1".to_string()]);
    let info = build_table_widget("Infos", vec!["Info 1".to_string()]);

    f.render_widget(homes, tables[0]);
    f.render_widget(rooms, tables[1]);
    f.render_widget(devices, tables[2]);
    f.render_widget(info, tables[3]);

    let interaction = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40), // Command input
                Constraint::Percentage(60), // Results output
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let commands: Vec<ListItem> = vec!["AAAA"]
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let commands =
        List::new(commands).block(Block::default().borders(Borders::ALL).title("Commands"));

    let responses: Vec<ListItem> = vec!["BBBB"]
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let responses =
        List::new(responses).block(Block::default().borders(Borders::ALL).title("Responses"));

    f.render_widget(commands, interaction[0]);
    f.render_widget(responses, interaction[1]);
    // f.render_widget(commands, interaction[1]);
}
