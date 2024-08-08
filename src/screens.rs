use minifb::{Window, WindowOptions, Key};
use crate::framebuffer::Framebuffer;

pub fn show_success_screen(window_width: usize, window_height: usize) {
    let mut window = Window::new(
        "Success!",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut framebuffer = Framebuffer::new(window_width, window_height);

    // Llena el framebuffer con un color de éxito, por ejemplo, verde
    framebuffer.buffer.fill(crate::framebuffer::Color { r: 0, g: 255, b: 0 });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&framebuffer.buffer.iter().map(|color| {
            ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
        }).collect::<Vec<u32>>(), window_width, window_height).unwrap();
    }
}
