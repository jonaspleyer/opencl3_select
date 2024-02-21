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
const TEXT_COLOR: Color = tailwind::ZINC.c200;

struct PlatformItem {
    info: clinfo::PlatformInfo,
}

struct DeviceItem {
    info: clinfo::DeviceInfo,
}

struct StatefulList {
    currently_left: bool,
    state_left: ListState,
    state_right: ListState,
    items: Vec<PlatformItem>,
    priority_list: Vec<usize>,
    remaining: Vec<usize>,
}

struct App {
    items: StatefulList,
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
            items: StatefulList::from_platforms(platforms),
            divider_percentage: 40,
        }
    }

    /// Changes the status of the selected list item
    fn change_status(&mut self) {}

    fn go_top(&mut self) {
        self.items.state_left.select(Some(0))
    }

    fn go_bottom(&mut self) {
        self.items
            .state_left
            .select(Some(self.items.items.len() - 1))
    }

    fn move_divider(&mut self, length: i16) {
        self.divider_percentage = self
            .divider_percentage
            .saturating_add_signed(length)
            .min(100);
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
                        Char('h') | Left => self.items.unselect(),
                        Char('j') | Down => self.items.next(),
                        Char('k') | Up => self.items.previous(),
                        Char('l') | Right | Enter => self.change_status(),
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
        let items = List::new(items)
            .block(inner_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        StatefulWidget::render(items, inner_area, buf, &mut self.items.state_left);
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

        if let Some(si) = self.items.state_left.selected() {
            let current_devices = self.items.items[si].info.devices();
            let items: Vec<ListItem> = current_devices
                .iter()
                .enumerate()
                .map(|(i, device)| device.to_list_item(i))
                .collect();
            let items = List::new(items)
                .block(inner_block)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::REVERSED)
                        .fg(SELECTED_STYLE_FG),
                )
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(items, inner_area, buf, &mut self.items.state_left);
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

impl StatefulList {
    fn from_platforms(platforms: &Vec<clinfo::PlatformInfo>) -> StatefulList {
        StatefulList {
            currently_left: true,
            state_left: ListState::default(),
            state_right: ListState::default(),
            items: platforms
                .clone()
                .into_iter()
                .map(|platform_info| PlatformItem {
                    info: platform_info,
                })
                .collect(),
            priority_list: Vec::new(),
            remaining: (0..platforms.len()).collect(),
        }
    }

    fn next(&mut self) {
        let i = match self.state_left.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state_left.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state_left.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state_left.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state_left.offset();
        self.state_left.select(None);
        *self.state_left.offset_mut() = offset;
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
            Span::raw(format!("{}", self.vendor_id())),
            Span::raw(self.vendor_id_text()),
            Span::raw(self.name()),
            Span::raw(self.version()),
            Span::raw(format!("{}", self.r#type())),
            Span::raw(self.type_text()),
            Span::raw(self.profile()),
            Span::raw(self.extensions()),
            Span::raw(self.opencl_c_version()),
            Span::raw(format!("{}", self.svm_mem_capability())),
        ]);

        ListItem::new(text).bg(bg_color)
    }
}
