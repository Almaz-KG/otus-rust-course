#![allow(dead_code)]

use clap::Parser;
use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

struct TcpClient {
    host: String,
    port: u16,
}

impl TcpClient {
    pub fn new(host: String, port: u16) -> Self {
        TcpClient { host, port }
    }

    fn write_to_console(&self, content: &str) -> Result<(), String> {
        if !content.is_empty() {
            print!("{}", content);
            if content.as_bytes()[content.len() - 1] != b'\n' {
                println!();
            }
            stdout()
                .flush()
                .map_err(|e| format!("Couldn't flush stdout: {:?}", e))?;
        }
        Ok(())
    }

    fn write_prompt(&self) -> Result<(), String> {
        print!("->");
        stdout()
            .flush()
            .map_err(|e| format!("Couldn't flush stdout: {:?}", e))?;

        Ok(())
    }

    fn read_data(&self, socket: &mut TcpStream) -> Result<String, String> {
        let mut reader = BufReader::new(socket);
        let mut buf = [0; 4];
        reader.read_exact(&mut buf).unwrap();
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        reader.read_exact(&mut buf).unwrap();
        let result = String::from_utf8(buf).map_err(|e| e.to_string())?;

        Ok(result)
    }

    fn write_data(&self, socket: &mut TcpStream, data: &[u8]) -> Result<(), String> {
        socket
            .set_write_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| format!("Unable to set write timeout: {:?}", e))?;

        socket
            .write_all(data)
            .map_err(|e| format!("Error: {:?}", e))?;
        Ok(())
    }

    pub fn run(&self) -> Result<(), String> {
        let mut stream = TcpStream::connect((self.host.clone(), self.port))
            .expect("Unable to connect to the host");

        self.write_data(&mut stream, b"handshake")?;

        let handshake = self.read_data(&mut stream)?;

        if handshake.trim() != "handshake" {
            return Err(format!("Expected handshake message, but got {}", handshake));
        }

        println!(
            "Connection established.\n \
                Type --help to get detailed info. \
                Type `exit` or `quit` to exit from the app"
        );

        let mut stdin = std::io::stdin().lock();
        let mut quit = false;

        while !quit {
            let mut command = String::new();
            self.write_prompt()?;

            stdin.read_line(&mut command).expect("Unable read command");
            let command = command.trim();

            if command == "quit" || command == "exit" {
                quit = true;
                continue;
            }

            self.write_data(&mut stream, command.as_bytes())?;
            let response = self.read_data(&mut stream)?;
            self.write_to_console(&response)?;
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ClientArgs {
    /// The server host to connect
    #[arg(long, value_name = "host")]
    pub host: String,

    /// The server port to connect
    #[arg(short = 'p', long)]
    pub port: u16,
}

// fn main() {
//     let args = ClientArgs::parse();
//
//     let host = args.host;
//     let port = args.port;
//
//     let client = TcpClient::new(host, port);
//     let result = client.run();
//     if result.is_err() {
//         println!("[Client] Error: {}", result.err().unwrap())
//     }
//
//
//
// }

/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[2]);
}
