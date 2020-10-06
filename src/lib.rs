use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::fonts::Font;
use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::style::{PrimitiveStyle, TextStyle};

use core::fmt::Write;
use heapless::consts::*;
use heapless::String;

pub enum EntryType {
    /// Calls function when selected
    Select,
    /// Boolean
    Bool(bool),
    /// 32-bit integer (value, min, max)
    I32((i32, i32, i32)),
}

pub struct Keys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
}

pub struct MenuOptions<C: PixelColor, F: Font> {
    pub background: C,
    pub text: C,
    pub highlight: C,
    pub font: F,
    pub border: u32,
    pub spacing: u32,
}
pub struct Menu<'a, C: PixelColor, F: Font> {
    highlighted_option: u8,
    selected: bool,
    redraw: bool,
    structure: &'a mut [(&'a str, EntryType)],
    size: Size,
    options: &'a MenuOptions<C, F>,
}

impl<'a, C, F> Menu<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    pub fn new(
        options: &'a MenuOptions<C, F>,
        size: Size,
        structure: &'a mut [(&'a str, EntryType)],
    ) -> Menu<'a, C, F> {
        Menu {
            redraw: true,
            highlighted_option: 0,
            selected: false,
            structure,
            size,
            options,
        }
    }

    pub fn selected_option(&self) -> Option<u8> {
        if self.selected {
            Some(self.highlighted_option)
        } else {
            None
        }
    }

    pub fn update(&mut self, keys: &Keys) -> bool {
        let mut tmp_opt = self.highlighted_option as i8;

        if keys.up {
            tmp_opt -= 1;
        }
        if keys.down {
            tmp_opt += 1;
        }

        if tmp_opt >= self.structure.len() as i8 {
            tmp_opt = 0;
        } else if tmp_opt < 0 {
            tmp_opt = self.structure.len() as i8 - 1;
        }

        self.highlighted_option = tmp_opt as u8;

        self.selected = keys.a;

        if keys.up || keys.down || keys.a || keys.right || keys.left {
            self.redraw = true;
        }

        match self.structure[self.highlighted_option as usize].1 {
            EntryType::Bool(ref mut val) => {
                if keys.left || keys.right || keys.a {
                    *val = !*val;
                }
            }
            EntryType::I32(ref mut val) => {
                if keys.right && ((*val).0 < (*val).2) {
                    (*val).0 += 1;
                }
                if keys.left && ((*val).0 > (*val).1) {
                    (*val).0 -= 1;
                }
            }
            _ => {}
        }

        false
    }

    pub fn force_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn draw_text<D: DrawTarget<Color = C>>(
        &self,
        display: &mut D,
        text: &str,
        x: i32,
        y: i32,
    ) -> Result<(), D::Error> {
        let text_style = TextStyle::new(Font6x8, self.options.text);
        Text::new(text, Point::new(x, y))
            .into_styled(text_style)
            .draw(display)?;
        Ok(())
    }

    pub fn draw<D: DrawTarget<Color = C>>(&mut self, display: &mut D) -> Result<(), D::Error> {
        if self.redraw {
            self.redraw = false;

            display.clear(self.options.background)?;

            let title_border = 2;
            let font_height = 8;
            let font_width = 6;

            Rectangle::new(
                Point::new(self.options.border as i32, self.options.border as i32),
                Size::new(
                    self.size.width as u32 - self.options.border * 2,
                    font_height + title_border * 2,
                ),
            )
            .into_styled(PrimitiveStyle::with_stroke(self.options.highlight, 1))
            .draw(display)?;

            self.draw_text(
                display,
                "Menu",
                self.size.width as i32 / 2 - "Menu".len() as i32 * font_width / 2,
                self.options.border as i32 + title_border as i32,
            )?;

            let text_x = self.options.border as i32;
            let text_y_start = self.options.border as i32
                + title_border as i32
                + (font_height / 2) as i32
                + self.options.spacing as i32;

            for i in 0..self.structure.len() {
                // let entry_text_x = text_x + (self.structure[i].0.len() as i32 + 1) * 6;
                let text_y = text_y_start + i as i32 * self.options.spacing as i32;

                self.draw_text(display, self.structure[i].0, text_x, text_y)?;

                match self.structure[i].1 {
                    EntryType::Select => {}
                    EntryType::Bool(val) => {
                        let x = match val {
                            true => "<1>",
                            false => "<0>",
                        };
                        self.draw_text(
                            display,
                            x,
                            self.size.width as i32
                                - self.options.border as i32
                                - x.len() as i32 * font_width as i32,
                            text_y,
                        )?;
                    }
                    EntryType::I32(val) => {
                        let mut value_str = String::<U8>::new();
                        write!(value_str, "<{}>", val.0).unwrap();
                        self.draw_text(
                            display,
                            &value_str[..],
                            self.size.width as i32
                                - self.options.border as i32
                                - value_str.len() as i32 * font_width as i32,
                            text_y,
                        )?;
                    }
                }
            }

            // Underline the highlighted option
            Rectangle::new(
                Point::new(
                    self.options.border as i32,
                    text_y_start
                        + font_height as i32
                        + self.highlighted_option as i32 * self.options.spacing as i32,
                ),
                Size::new(
                    self.structure[self.highlighted_option as usize].0.len() as u32 * 6,
                    1,
                ),
            )
            .into_styled(PrimitiveStyle::with_fill(self.options.text))
            .draw(display)?;
        }
        Ok(())
    }
}
