use minifb::{Window, WindowOptions};

fn main() {
    let mut window = match Window::new("Test", 680, 400, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window: {}", err);
            return;
        }
    };

    // Create a buffer to display (e.g., an array for the pixel colors)
    let mut buffer: Vec<u32> = vec![0; 680 * 400]; // 680x400 window with black pixels

    // Main loop to keep the window open
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Here you would update the buffer or render content

        // Update the window with the buffer (draw the content)
        window.update_with_buffer(&buffer, 680, 400).unwrap();
    }
}
