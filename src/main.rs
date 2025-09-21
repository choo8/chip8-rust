use std::{
    env, fs, io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use chip8_rust::chip8::Chip8;

const CYCLES_PER_SECOND: u32 = 540;
const TARGET_FPS: u64 = 60;
const MICROSECONDS_PER_FRAME: u64 = 1_000_000 / TARGET_FPS;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];
    let rom_data = fs::read(rom_path)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom_data);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let cycles_per_frame = CYCLES_PER_SECOND / TARGET_FPS as u32;
    let mut last_frame_time = Instant::now();

    loop {
        let elaspsed = last_frame_time.elapsed();
        if elaspsed < Duration::from_micros(MICROSECONDS_PER_FRAME) {
            std::thread::sleep(Duration::from_micros(MICROSECONDS_PER_FRAME) - elaspsed);
        }
        last_frame_time = Instant::now();

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    break; // Exit loop
                }
                // TODO: Implement keyboard mapping to chip8.keyboard
            }
        }

        for _ in 0..cycles_per_frame {
            chip8.emulate_cycle();
        }

        chip8.update_timers();

        terminal.draw(|f| ui(f, &chip8))?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, chip8: &Chip8) {
    // CHIP-8 display is 64x32
    let display_width = 64;
    let display_height = 32;

    // Get the terminal size
    let term_area = f.area();

    // Center the display and clamp to terminal bounds
    let area = Rect {
        x: term_area.x + (term_area.width.saturating_sub(display_width as u16 + 2)) / 2,
        y: term_area.y + (term_area.height.saturating_sub(display_height as u16 + 2)) / 2,
        width: display_width as u16 + 2,
        height: display_height as u16 + 2,
    };

    if term_area.width < display_width as u16 + 2 || term_area.height < display_height as u16 + 2 {
        let message = Paragraph::new("Terminal too small!\nPlease resize to at least 64x32.")
            .alignment(Alignment::Center);
        // Center the message in the available space
        let area = centered_rect(50, 50, term_area);
        f.render_widget(message, area);
        return; // Stop rendering the main UI
    }

    let display_buffer = chip8.display.get_buffer();
    let mut display_text = String::new();

    for row in display_buffer {
        for &pixel in row {
            if pixel {
                display_text.push('█');
            } else {
                display_text.push(' ');
            }
        }
        display_text.push('\n');
    }

    let display = Paragraph::new(display_text)
        .style(Style::default().fg(Color::Green))
        .block(
            Block::default()
                .title("CHIP-8 Display")
                .borders(Borders::ALL),
        );

    f.render_widget(display, area);
}

/// Helper function to create a centered rect inside an area
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn map_key(key_code: KeyCode) -> Option<u8> {
    match key_code {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('q') => Some(0x4),
        KeyCode::Char('w') => Some(0x5),
        KeyCode::Char('e') => Some(0x6),
        KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('a') => Some(0x7),
        KeyCode::Char('s') => Some(0x8),
        KeyCode::Char('d') => Some(0x9),
        KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('x') => Some(0x0),
        KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('v') => Some(0xF),
        _ => None,
    }
}
