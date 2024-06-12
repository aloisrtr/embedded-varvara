use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    primitives::Rectangle,
};

/// # Screen device
/// Emulates a screen using [embedded-graphics](https://docs.rs/embedded-graphics)
/// [DrawTarget](https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html)
/// trait. Any peripheral implementing the [DrawTarget](https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html)
/// trait can thus easily be used with an embedded Varvara machine.

/// The colour palette used by the Varvara ordinator.
/// It is composed of 4 colours, encoded as 12 bits RGB.
///
/// Note that these colours are public, meaning that you could set them to values
/// like #ffff, which would be invalid. In such case, only the first 12 bits
/// of the colour would be used.
/// The last 4 bits may be used as an alpha value in the future.
#[repr(packed)]
pub struct ColourPalette {
    pub colour0: u16,
    pub colour1: u16,
    pub colour2: u16,
    pub colour3: u16,
}

/// A Varvara screen interface using embedded-graphics' [`DrawTarget`] trait.
pub struct Screen<C> {
    pub(crate) canvas: C,
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) sprite_address: u16,

    pub(crate) auto_x: bool,
    pub(crate) auto_y: bool,
    pub(crate) auto_sprite: bool,
    pub(crate) length: u8,
}
impl<C> Screen<C> {
    pub fn new(canvas: C) -> Self {
        Self {
            canvas,
            x: 0,
            y: 0,
            sprite_address: 0,
            auto_x: false,
            auto_y: false,
            auto_sprite: false,
            length: 0,
        }
    }

    pub fn current_x(&self) -> u16 {
        self.x
    }
    pub fn current_y(&self) -> u16 {
        self.y
    }
    pub fn current_sprite_address(&self) -> u16 {
        self.sprite_address
    }
}
impl<C> Screen<C>
where
    C: DrawTarget,
{
    pub fn width(&self) -> u16 {
        self.canvas.bounding_box().size.width as u16
    }
    pub fn height(&self) -> u16 {
        self.canvas.bounding_box().size.height as u16
    }

    pub fn draw_fill(
        &mut self,
        foreground: bool,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        colour: u8,
    ) -> Result<(), <C as DrawTarget>::Error> {
        self.canvas.fill_solid(
            &Rectangle::new(
                Point::new(x as i32, y as i32),
                Size::new(width as u32, height as u32),
            ),
            colour,
        )?;
        Ok(())
    }

    pub fn draw_pixel(&mut self, colour: u8) -> Result<(), <C as DrawTarget>::Error> {
        self.canvas.fill_solid(
            &Rectangle::new(Point::new(self.x as i32, self.y as i32), Size::new(1, 1)),
            colour,
        )?;
        if self.auto_x {
            self.x += 1
        }
        if self.auto_y {
            self.y += 1
        }
        Ok(())
    }
}
