mod clinfo;
mod display;
mod error;
mod storage;

use clinfo::DeviceInfo;
use error::Result;

use std::{io, io::stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::palette::tailwind, style::Stylize, widgets::*};

const HEADER_BG: Color = tailwind::ZINC.c950;
const NORMAL_ROW_COLOR: Color = tailwind::ZINC.c950;
const ALT_ROW_COLOR: Color = tailwind::ZINC.c900;
const SELECTED_STYLE_FG: Color = tailwind::ZINC.c300;
const SELECTED_STYLE_FG_LIGHT: Color = tailwind::ZINC.c500;
const TEXT_COLOR: Color = tailwind::ZINC.c200;

struct PlatformItem {
    info: clinfo::PlatformInfo,
    devices: DeviceList,
}

struct DeviceItem {
    info: clinfo::DeviceInfo,
}

struct PlatformList {
    state: ListState,
    items: Vec<PlatformItem>,
}

struct DeviceList {
    state: ListState,
    items: Vec<DeviceItem>,
}

struct App {
    currently_left: bool,
    items: PlatformList,
    divider_percentage: u16,
}

fn main() -> Result<()> {
    // setup terminal
    let terminal = init_terminal()?;

    let cl_state = clinfo::get_setup()?;

    // create app and run it
    App::new(&cl_state.get_platforms()).run(terminal)?;

    restore_terminal()?;

    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

impl App {
    fn new<'a>(platforms: &Vec<clinfo::PlatformInfo>) -> App {
        App {
            currently_left: true,
            items: PlatformList::from_platforms(platforms),
            divider_percentage: 40,
        }
    }

    /// Changes the status of the selected list item
    fn change_status(&mut self) {}

    fn go_top(&mut self) {
        if self.currently_left {
            self.items.state.select(Some(0));
        } else if let Some(i) = self.items.state.selected() {
            self.items
                .items
                .get_mut(i)
                .unwrap()
                .devices
                .state
                .select(Some(0));
        }
    }

    fn go_bottom(&mut self) {
        if self.currently_left {
            self.items.state.select(Some(self.items.items.len() - 1))
        } else if let Some(i) = self.items.state.selected() {
            let device_list = &mut self.items.items.get_mut(i).unwrap().devices;
            device_list
                .state
                .select(Some(device_list.items.len() - 1));
        }
    }

    fn move_divider(&mut self, length: i16) {
        self.divider_percentage = self
            .divider_percentage
            .saturating_add_signed(length)
            .min(100);
    }

    fn move_right(&mut self) {
        self.currently_left = false;
    }

    fn move_left(&mut self) {
        self.currently_left = true;
    }

    fn next(&mut self) {
        if self.currently_left {
            self.items.next();
        } else if let Some(i) = self.items.state.selected() {
            let device_list = &mut self.items.items.get_mut(i).unwrap().devices;
            device_list.next();
        }
    }

    fn previous(&mut self) {
        if self.currently_left {
            self.items.previous();
        } else if let Some(i) = self.items.state.selected() {
            let device_items = &mut self.items.items.get_mut(i).unwrap().devices;
            device_items.previous();
        }
    }
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => return Ok(()),
                        Char('h') | Left => self.move_left(),
                        Char('j') | Down => self.next(),
                        Char('k') | Up => self.previous(),
                        Char('l') | Right | Enter => self.move_right(),
                        Char('g') => self.go_top(),
                        Char('G') => self.go_bottom(),
                        Char('H') => self.move_divider(-5),
                        Char('L') => self.move_divider(5),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        // Create two chunks with equal vertical screen space. One for the list and the other for
        // the info block.
        let vertical = Layout::horizontal([
            Constraint::Percentage(self.divider_percentage),
            Constraint::Percentage(100 - self.divider_percentage),
        ]);
        let [left_platform_list, right_device_list] = vertical.areas(rest_area);

        self.render_title(header_area, buf);
        self.render_platforms(left_platform_list, buf);
        self.render_devices(right_device_list, buf);
        // TODO self.render_priority_list(.., buf);
        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("opencl3_select")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn get_fg_style(&self, is_left: bool) -> Style {
        if self.currently_left == is_left {
            Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
            .fg(SELECTED_STYLE_FG)
        } else {
            Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
            .fg(SELECTED_STYLE_FG_LIGHT)
        }
    }

    fn render_platforms(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(HEADER_BG)
            .title("Platforms")
            .title_alignment(Alignment::Center);
        let inner_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // We can render the header in outer_area.
        outer_block.render(outer_area, buf);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .items
            .items
            .iter()
            .enumerate()
            .map(|(i, platform_info)| platform_info.to_list_item(i))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let style = self.get_fg_style(true);
        let items = List::new(items)
            .block(inner_block)
            .highlight_style(style)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        StatefulWidget::render(items, inner_area, buf, &mut self.items.state);
    }

    fn render_devices(&mut self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(HEADER_BG)
            .title("Devices")
            .title_alignment(Alignment::Center);
        let inner_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);
        outer_block.render(outer_area, buf);

        // Find index of platform
        if let Some(si) = self.items.state.selected() {
            // Obtain all devices under platform
            let style = self.get_fg_style(false);
            let current_devices = &mut self.items.items.get_mut(si).unwrap();
            let items: Vec<ListItem> = current_devices
                .devices
                .items
                .iter()
                .enumerate()
                .map(|(i, device)| device.info.to_list_item(i))
                .collect();
            let items = List::new(items)
                .block(inner_block)
                .highlight_style(style)
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(items, inner_area, buf, &mut current_devices.devices.state);
        }
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.",
        )
        .centered()
        .render(area, buf);
    }
}

impl PlatformList {
    fn from_platforms(platforms: &Vec<clinfo::PlatformInfo>) -> PlatformList {
        PlatformList {
            state: ListState::default(),
            items: platforms
                .clone()
                .into_iter()
                .map(|platform_info| {
                    let items = platform_info
                        .devices()
                        .into_iter()
                        .map(|info| DeviceItem { info })
                        .collect();
                    PlatformItem {
                        info: platform_info,
                        devices: DeviceList {
                            state: ListState::default(),
                            items,
                        },
                    }
                })
                .collect(),
        }
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
}

impl DeviceList {
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
}

fn style_platform_name<'a>(name: String, style_string: String) -> Span<'a> {
    if name.to_lowercase().contains("nvidia") {
        return Span::raw(style_string).green();
    }
    if name.to_lowercase().contains("intel") {
        return Span::raw(style_string).blue();
    }
    if name.to_lowercase().contains("amd") {
        return Span::raw(style_string).red();
    }
    Span::raw(style_string)
}

impl PlatformItem {
    fn to_list_item(&self, index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };

        let mut text = Text::default();
        text.extend([
            style_platform_name(self.info.name(), self.info.name()),
            style_platform_name(self.info.name(), self.info.version()),
            style_platform_name(self.info.name(), self.info.vendor()),
            style_platform_name(self.info.name(), self.info.profile()),
        ]);

        ListItem::new(text).bg(bg_color)
    }
}

impl DeviceInfo {
    fn to_list_item(&self, index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };

        let mut text = Text::default();
        text.extend([
            Span::raw(self.vendor()),
            Span::raw(format!("Vendor Id: {}", self.vendor_id())),
            Span::raw(self.vendor_id_text()),
            Span::raw(self.name()),
            Span::raw(self.version()),
            Span::raw(format!("Type: {}", self.r#type())),
            Span::raw(self.type_text()),
            Span::raw(self.profile()),
            Span::raw(self.extensions()),
            Span::raw(self.opencl_c_version()),
            Span::raw(format!("SVM Mem Capability: {}", self.svm_mem_capability())),
        ]);

        ListItem::new(text).bg(bg_color)
    }
}
