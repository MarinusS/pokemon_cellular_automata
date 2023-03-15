use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pixels::{Error, Pixels, SurfaceTexture, TextureError};
use winit::window::Window;

use crate::pokemon::*;

/// Representation of the application state. Current placeholder a box will bounce around the screen.
pub struct Front {
    pokemon_world: PokemonWorld,
    pixels: Pixels,
}

impl Front {
    //Place holder for initializing world
    pub fn new(window: &Window) -> Result<Self, Error> {
        Ok(Self {
            pokemon_world: PokemonWorld::new(WINDOW_WIDTH, WINDOW_HEIGHT),

            pixels: {
                let window_size = window.inner_size();
                let surface_texture =
                    SurfaceTexture::new(window_size.width, window_size.width, window);
                Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?
            },
        })
    }

    pub fn update(&mut self) {
        self.pokemon_world.update();
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) -> Result<(), TextureError> {
        self.pixels.resize_surface(width, height)
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&mut self) -> Result<(), Error> {
        let frame = self.pixels.get_frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            //Pixel x and y coordinates
            let x = (i % WINDOW_WIDTH as usize) as i16;
            let y = (i / WINDOW_WIDTH as usize) as i16;

            let (r, g, b) = self
                .pokemon_world
                .get_pokemon_color(x as usize, y as usize)
                .expect("Coordinates in pokemon world bounds");
            let rgba = [r, g, b, 0xff];

            pixel.copy_from_slice(&rgba);
        }

        self.pixels.render()
    }
}
