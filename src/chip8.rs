use std::fs::File;
use std::io;
use std::io::Read;
use crate::memory::Memory;
use crate::cpu::Cpu;
use std::{thread, time};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::unix::WindowExtUnix;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub struct CHIP8 {
    mem: Memory,
    cpu: Cpu,
}

impl CHIP8 {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn clone(mem: Memory, cpu: Cpu) -> Self {
        Self {
            mem,
            cpu,
        }
    }

    pub fn load_rom(&mut self, path: &str) -> io::Result<()> {
        let mut rom_file = File::open(path)?;
        let mut rom_data = Vec::new();

        let rom_size = rom_file.read_to_end(&mut rom_data)?;

        // Load the ROM into main memory at 0x200
        for i in 0..rom_size {
            self.mem.set(0x200 + i,rom_data[i]);
        }

        Ok(())
    }

    pub fn start(mut self) {
        let mut ten_millis = time::Duration::from_millis(16);
        let mut now = time::Instant::now();


        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 10.0, HEIGHT as f64 * 10.0);
            WindowBuilder::new()
                .with_title("Chip 8 emulate")
                .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };


        event_loop.run(move |event, _, control_flow| {
            if now.elapsed() >= ten_millis {
                now = time::Instant::now();
                self.cpu.timer();
            }
            if let Event::RedrawRequested(_) = event {
                self.cpu.cycle(&mut self.mem);
                self.cpu.draw(pixels.get_frame());

               if pixels
                   .render()
                   // .map_err(|e| error!("pixels.render() failed: {}", e))
                   .is_err()
               {
                   *control_flow = ControlFlow::Exit;
                   return;
               }
           }
            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    pixels.resize_surface(size.width, size.height);
                }

                // Update internal state and request a redraw
                // self.cpu.cycle(&mut self.mem);
                window.request_redraw();
            }
        });
    }

    pub fn load_font(&mut self) -> () {
        let font: [u8; 80] =
            [0xF0,
                0x90,
                0x90,
                0x90,
                0xF0,
                0x20,
                0x60,
                0x20,
                0x20,
                0x70,
                0xF0,
                0x10,
                0xF0,
                0x80,
                0xF0,
                0xF0,
                0x10,
                0xF0,
                0x10,
                0xF0,
                0x90,
                0x90,
                0xF0,
                0x10,
                0x10,
                0xF0,
                0x80,
                0xF0,
                0x10,
                0xF0,
                0xF0,
                0x80,
                0xF0,
                0x90,
                0xF0,
                0xF0,
                0x10,
                0x20,
                0x40,
                0x40,
                0xF0,
                0x90,
                0xF0,
                0x90,
                0xF0,
                0xF0,
                0x90,
                0xF0,
                0x10,
                0xF0,
                0xF0,
                0x90,
                0xF0,
                0x90,
                0x90,
                0xE0,
                0x90,
                0xE0,
                0x90,
                0xE0,
                0xF0,
                0x80,
                0x80,
                0x80,
                0xF0,
                0xE0,
                0x90,
                0x90,
                0x90,
                0xE0,
                0xF0,
                0x80,
                0xF0,
                0x80,
                0xF0,
                0xF0,
                0x80,
                0xF0,
                0x80,
                0x80];

        for i in 0..font.len() {
            self.mem.set(i, font[i]);
        }
    }

}
