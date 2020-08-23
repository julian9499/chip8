use pixels::{Pixels, SurfaceTexture, Error};
use winit::window::Window;
use winit::event_loop::EventLoop;
use winit::dpi::{PhysicalSize, LogicalSize, LogicalPosition};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

pub struct Screen {
    window: Window,
    height: u32,
    width: u32,
    pixels: Pixels,
}

impl Screen {
    pub fn new(title: &str,
               event_loop: &EventLoop<()>,
    ) -> Self {
        create_window(title, event_loop)
    }

    pub fn clear_screen(&mut self) {
        let frame = self.pixels.get_frame();
        for pixel in frame.chunks_exact_mut(4) {
            Screen::set_pixel(pixel, 0x00);
        }
        self.pixels.render();
    }

    pub fn draw_sprite(&mut self, sprite: [u8; 15], x: u8, y: u8) -> bool {
        let mut flag = false;
        for (index, byte) in sprite.iter().enumerate() {
            for i in 0..7 {
                if (byte >> i) & 1 != 0 {
                    let x_coord: u16 = ((x as u16 + 7 - i) % 64) as u16;
                    let y_coord: u16 = (((y.wrapping_add(index as u8)) % 32) as u16).wrapping_mul((64 as u16));
                    let cord = x_coord + y_coord;
                    let frame = self.pixels.get_frame().chunks_exact_mut(4);
                    for (j, pixel) in frame.enumerate() {
                        if j == cord as usize {
                            if Screen::is_pixel_white(pixel) {
                                Screen::set_pixel(pixel, 0x00);
                                flag = true;
                            } else {
                                Screen::set_pixel(pixel, 0xFF);
                            }
                        }
                    }
                }
            }
        }
        self.pixels.render();
        return flag;
    }

    pub fn is_pixel_white(pixel: &mut [u8]) -> bool {
        return if pixel[0] > 0 {
            true
        } else {
            false
        };
    }

    pub fn set_pixel(pixel: &mut [u8], color: u8) -> () {
        pixel[0] = color; // R
        pixel[1] = color; // G
        pixel[2] = color; // B
        pixel[3] = 0xff; // A
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.pixels.render()
    }
}


/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> Screen {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        let size = window.current_monitor().size();
        (
            size.width as f64 / hidpi_factor,
            size.height as f64 / hidpi_factor,
        )
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let surface = pixels::wgpu::Surface::create(&window);
    let size = default_size.to_physical::<f64>(hidpi_factor);
    let height = size.height.round() as u32;
    let width = size.width.round() as u32;

    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture);
    match pixels {
        Ok(p) => Screen {
            window,
            height,
            width,
            pixels: p,
        },
        Err(err) => panic!("Creating pixels screen went wrong: {}", err)
    }
}