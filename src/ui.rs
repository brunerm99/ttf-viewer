use std::{io, thread, time::{Duration, Instant}};
use tui::{
    backend::{CrosstermBackend, Backend}, 
    widgets::{Widget, Block, Borders, canvas::{Canvas, Map, MapResolution}, ListItem, ListState, List},
    layout::{Layout, Constraint, Direction},
    style::{Color, Style, Modifier},
    Terminal,
    Frame, text::Span,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn setup_and_run() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);

    let app = App::new();

    let res = run_app(&mut terminal, tick_rate, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration, mut app: App) -> Result<(), io::Error>{
    let mut now = Instant::now();
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        let polling_timeout = tick_rate
            .checked_sub(now.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(polling_timeout)? { // Check for keyboard, mouse, etc. events
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('j') => {
                        app.fnames.next();
                    }
                    KeyCode::Char('k') => {
                        app.fnames.prev();
                    }
                    _ => {}
                }
            }
        }

        if now.elapsed() >= tick_rate {
            // Update display
            now = Instant::now();
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(frame.size());

    let list: Vec<ListItem> = app
        .fnames
        .items
        .iter()
        .map(|i| {
            ListItem::new(*i).style(
                Style::default()
                    .fg(Color::White)
                    // .bg(Color::White)
                )
        })
        .collect();

    let files_block = Block::default()
        .title("TTF Files")
        .borders(Borders::ALL);

    let file_items = List::new(list)
        .block(files_block)
        .highlight_style(Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    )
    .highlight_symbol(">> ");

    frame.render_stateful_widget(file_items, chunks[0], &mut app.fnames.state);

    let block = Block::default()
        .title("Available characters")
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[1]);
}

pub struct StatefulList<T> {
    items: Vec<T>,
    state: ListState,
}

impl <T> StatefulList<T> {
    fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            items: items,
            state: ListState::default(),
        }
    }

    fn next(&mut self) {
        let idx = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    Some(0)
                } else {
                    Some(i + 1)
                }
            }
            None => Some(0),
        };
        self.state.select(idx);
    }

    fn prev(&mut self) {
        let idx = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    Some(self.items.len() - 1)
                } else {
                    Some(i - 1)
                }
            }
            None => Some(0),
        };
        self.state.select(idx);
    }
}

struct App<'a> {
    fnames: StatefulList<&'a str>,
}

impl <'a> App<'a> {
    fn new() -> App<'a> {
        let fnames = vec![
            "/home/marchall/.local/share/fonts/NerdFonts/FiraCodeNerdFont-Regular.ttf",
            "/home/marchall/.local/share/fonts/NerdFonts/FiraCodeNerdFont-Bold.ttf",
            "/home/marchall/.local/share/fonts/NerdFonts/FiraCodeNerdFont-Light.ttf",
            "/usr/share/fonts/TTF/FiraCodeNerdFontPropo-Retina.ttf",
        ];
        // TODO: Add available characters list which is computed by fonttools.rs
        App {
            fnames: StatefulList::new(fnames),
        }
    }
}