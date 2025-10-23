use macroquad::prelude::*;

/// Draws the given text centered in the window with the specified font size and color.
pub fn draw_centered_text(text: &str, font_size: f32, color: Color) {
    let window_width = screen_width();
    let window_height = screen_height();
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (window_width - text_dims.width) / 2.0;
    let y = (window_height - text_dims.height) / 2.0 + text_dims.height;
    draw_text(text, x, y, font_size, color);
}

/// Draws the given text at the bottom center of the window with the specified font size and color.
pub fn draw_bottom_centered_text(text: &str, font_size: f32, color: Color) {
    let window_width = screen_width();
    let window_height = screen_height();
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let x = (window_width - text_dims.width) / 2.0;
    let y = window_height - 20.0; // 20 pixels from the bottom
    draw_text(text, x, y, font_size, color);
}

/// Draws the difficulty selection menu with the given selected option highlighted.
pub fn draw_difficulty_menu(selected: usize) {    
    // Draw title
    let title = "Select difficulty";
    let font_size = 48.0;
    let text_dims = measure_text(title, None, font_size as u16, 1.0);
    let x = (screen_width() - text_dims.width) / 2.0;
    let y = screen_height() / 2.0 - 120.0;
    draw_text(title, x, y, font_size, ORANGE);

    // Draw options
    let options = ["Easy", "Medium", "Hard"];
    for (i, &option) in options.iter().enumerate() {
        let color = if i == selected { BLUE } else { WHITE };
        let symbol = if i == selected { "> " } else { "  " }; // Star for selected, space for others
        let text = format!("{}{}", symbol, option);
        draw_text(
            &text,
            screen_width() / 2.0 - 80.0,
            screen_height() / 2.0 + (i as f32 - 1.0) * 50.0,
            40.0,
            color,
        );
    }
}