use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    io::{BufReader},
    time::{Duration, Instant},
    fs::{File,}
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use serde_json::{Value, Number};

use std::fs::OpenOptions;
use std::io::prelude::*;

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {

    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn init(&mut self) {
        self.state.select(Some(0));
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select(&mut self) -> &T {
        // Get the selected item
        let item = self.items.get(self.state.selected().unwrap()).unwrap();

        // put items into self.sub_items depending on the selected item
        return item;
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.

struct SubItemItem {
    title: &'static str,
    function: (),
    description_id: i32,
}

struct App<'a> {
    items: StatefulList<(&'a str, i32)>,
    sub_items: StatefulList<(&'a str, ())>,
    sub_item_index: Vec<Vec<SubItemItem>>,
    current_menu: i32, // 1 = main menu, 2 = sub menu
    descriptions: Value,
    // a description paragraph object
    description: Paragraph<'a>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Flash Windows 11", 1),
                ("Activate Windows", 2),
                ("Download NexaCrackz", 3),
                ("SpotX Download", 4),
                ("Rust Server Maker", 5),
                ("Settings", 6),
            ]),
            sub_items: StatefulList::with_items(vec![
            ]),

            sub_item_index: vec![
                vec![ // main menu item 0
                    SubItemItem {title: "Flash latest version", function: {}, description_id: 0},
                    SubItemItem {title: "Pick Version", function: {}, description_id: 1},
                ],
                vec![ // main menu item 1
                    SubItemItem {title: "MAS (WindowsAddict)", function: {}, description_id: 2}, // Credit to: https://github.com/massgravel/Microsoft-Activation-Scripts
                    SubItemItem {title: "Custom KMS", function: {}, description_id: 3},
                ],
                vec![
                    SubItemItem {title: "Install NexaCrackz",  function: {}, description_id: 4},
                ],
                vec![
                    SubItemItem {title: "Install SpotX-Win", function: {}, description_id: -1},
                    SubItemItem {title: "Install SpotX-Linux", function: {}, description_id: -1},
                    SubItemItem {title: "Install SpotX-Mac", function: {}, description_id: -1},
                ],
                vec![
                    SubItemItem {title: "Install Rust Server", function: {}, description_id: -1},
                ],
                vec![
                    SubItemItem {title: "Change Theme", function: {}, description_id: -1},
                    SubItemItem {title: "Change Font", function: {}, description_id: -1},
                ],
            ],

            // sub_item_index
            

            current_menu: 1,

            descriptions: Value::String("".to_string()),

            description: Paragraph::new("".to_string())
                .block(Block::default().borders(Borders::ALL).title("Description"))
                .wrap(Wrap { trim: true }),
        }
    }

    /// Rotate through the event list.
    /// This only exists to simulate some kind of "progress"
    fn on_tick(&mut self) {
    }
}

fn handle_select(app: &mut App) {

    // when clicked on a main menu item get the right list from the sub_item_index and set it to the sub_items list and log every step using log_ln fn
    if app.current_menu == 1 {
        let selected_item = app.items.select();
        let sub_item_list = &app.sub_item_index[(selected_item.1 as usize)-1];
        app.sub_items.items = sub_item_list.iter().map(|x| (x.title, x.function)).collect();
        
        app.current_menu = 2;
        app.sub_items.next();
    } else if  app.current_menu == 2 {
        // let selected_item = app.sub_items.select();
        // log_ln(format!("Selected sub item: {}", selected_item.0));
        // app.current_menu = 1;        
    }
}

fn handle_hover(app: &mut App) {
    // log_ln("Hovered".to_string());

    if app.current_menu == 1 {
        let selected_item = app.items.select();
        let sub_item_list = &app.sub_item_index[(selected_item.1 as usize)-1];
        app.sub_items.items = sub_item_list.iter().map(|x| (x.title, x.function)).collect();

    } else if app.current_menu == 2 {

        

        // get description id from the selected item
        let selected_item = app.sub_items.select();
        let sub_item_list = &app.sub_item_index[(app.items.state.selected().unwrap() as usize)-1];
        // get the description from the descriptions json file
        let description = &app.descriptions[description_id.to_string()];

        // set the description paragraph text to the description





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
    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();

    // laod the descriptions.json file into app.descriptions using the serde_json crate
    let file = File::open("C:\\Users\\oredg\\Documents\\coding projects\\winCookie\\src\\descriptions.json")?;
    let reader = BufReader::new(file);
    app.descriptions = serde_json::from_reader(reader)?;

    app.items.init();
    handle_hover(&mut app);

    let res = run_app(&mut terminal, app, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left | KeyCode::Backspace => {
                        if app.current_menu == 2 {
                            app.current_menu = 1;
                            app.sub_items.unselect();
                        }
                        
                    },
                    KeyCode::Down => {
                        if app.current_menu == 1 {
                            app.items.next();
                        } else if app.current_menu == 2 {
                            app.sub_items.next()
                        }
                        handle_hover(&mut app);
                    },
                    KeyCode::Up => {
                        if app.current_menu == 1 {
                            app.items.previous();
                        } else if app.current_menu == 2 {
                            app.sub_items.previous()
                        }
                        handle_hover(&mut app);
                        
                    },
                    KeyCode::Enter | KeyCode::Right => {
                        // let item = app.items.select();

                        handle_select(&mut app);

                        // app.current_menu = 2;
                        // app.sub_items.next();
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space

    // let size = f.size();


    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.0)];

            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL)
        .title("Menu"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);


    // add the sub items block
    let sub_items: Vec<ListItem> = app
        .sub_items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.0)];

            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();
    
    let sub_items = List::new(sub_items)

        .block(Block::default().borders(Borders::ALL)
        .title("Actions"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    
    f.render_stateful_widget(sub_items, chunks[1], &mut app.sub_items.state);

    // add a description block
    let description = Paragraph::new("test")
        .block(Block::default().borders(Borders::ALL).title("Description"))
        .wrap(Wrap { trim: true });

    f.render_widget(description, chunks[2]);



}


fn log_ln(text: String) {
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("C:/Users/oredg/Documents/coding projects/winCookie/src/log.txt")
    .unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Couldn't write to file: {}", e);
    }
}