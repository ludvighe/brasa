use std::time::{Duration, Instant};

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::Color,
};
use generics::Vec2;
use graphics::G;
use term::Term;
use ttc::TimeToChristmas;

mod generics;
mod graphics;
mod term;
mod ttc;

fn main() {
    event_loop();
}

fn event_loop() {
    let mut term = Term::new();
    let mut running = true;

    let fps_cap = 18;
    let loop_cap_wait_duration = Duration::from_millis(1000 / fps_cap);

    term.clear_all();
    let log_count = 3;
    let mut fire_grid = vec![vec![0u8; 50]; 20]; // Adjust dimensions to fit above logs
    let palette = [
        Color::Black,
        Color::Black,
        Color::DarkRed,
        Color::Red,
        Color::DarkYellow,
        Color::Yellow,
        Color::White,
    ];

    let mut show_stats = false;
    let mut last_frame_time = Instant::now();
    let ttc = TimeToChristmas::new();
    let mut ttc_show_seconds = false;
    let mut show_tree = true;

    while running {
        let now = Instant::now();
        let delta = now.duration_since(last_frame_time);
        last_frame_time = now;

        let fps = 1.0 / delta.as_secs_f32();
        let term_size = Term::size();

        if let Some(event) = term.read_event(0) {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => running = false,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('i'),
                    ..
                }) => show_stats = !show_stats,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    ..
                }) => ttc_show_seconds = !ttc_show_seconds,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('t'),
                    ..
                }) => show_tree = !show_tree,
                _ => {}
            }
        }

        let log_at = term_size - term_size.x() / 2 - (G.log.size() / 2) * log_count;

        let fire_start = Vec2::new(
            log_at.x + G.log.size().x / 3,
            log_at.y - fire_grid.len() as u16,
        );
        let fire_width = fire_grid[0].len();
        let fire_height = fire_grid.len();

        for x in 0..fire_width {
            fire_grid[fire_height - 1][x] = 30;
        }

        for y in (1..fire_height).rev() {
            for x in 0..fire_width {
                let seed = (Instant::now().elapsed().subsec_nanos() as usize) % 6;
                let decay = seed as u8;
                let current_intensity = fire_grid[y][x];
                let new_intensity = current_intensity.saturating_sub(decay);
                let spread_direction = if seed % 3 == 0 && x > 0 {
                    x - 1
                } else {
                    x.min(fire_width - 1)
                };
                fire_grid[y - 1][spread_direction] = new_intensity;
            }
        }

        term.clear_all();
        if show_stats {
            term.write_text(Vec2::new(0, 0), format!("FPS: {fps:.2}"));
            term.write_text(Vec2::new(0, 1), format!("Size: {term_size:?}"));
        }

        if ttc.is_christmas() {
            term.draw_text_bubble(Vec2::new(0, 2), format!("🎄 It's christmas! 🎅"));
        } else {
            let (days, hours, minutes, seconds) = ttc.time_until_christmas();
            let ttc_text = if ttc_show_seconds {
                format!("🎄 {days} days {hours} hours {minutes} minutes {seconds} seconds")
            } else {
                format!("🎄 {days} days {hours} hours {minutes} minutes")
            };
            term.draw_text_bubble(Vec2::new(4, 4), ttc_text);
        }

        if show_tree {
            term.draw_graphic(
                term_size - Vec2::new(32, (term_size.y / 2) + 10),
                G.tree,
                Color::AnsiValue(22),
            );
        }
        for i in 0..log_count {
            term.draw_graphic(log_at + G.log.size().x() * i, G.log, Color::AnsiValue(94));
        }
        for (y, row) in fire_grid.iter().enumerate() {
            for (x, &intensity) in row.iter().enumerate() {
                let color = palette[(intensity as usize / 6).min(palette.len() - 1)];
                if color == Color::Black {
                    continue;
                }
                term.set_pixel_bg(fire_start + Vec2::new(x as u16, y as u16), color);
            }
        }

        std::thread::sleep(loop_cap_wait_duration);
    }
    term.close();
}
