#![no_std]

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

pub enum EntryType<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    /// Select
    Select,
    /// Boolean
    Bool(bool),
    /// 32-bit integer (value, min, max)
    I32((i32, i32, i32)),
    /// Submenu
    Menu(&'a mut Menu<'a, C, F>),
    /// Return from submenu
    Return,
}

#[derive(Default, Clone)]
pub struct Keys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
}

pub struct MenuEntry<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    pub l: &'a str,
    pub t: EntryType<'a, C, F>,
}

#[derive(Clone)]
pub struct MenuOptions<C: PixelColor, F: Font> {
    pub background: C,
    pub text: C,
    pub highlight: C,
    pub font: F,
    pub border: u32,
    pub spacing: u32,
}
pub struct Menu<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    title: &'a str,
    highlighted_option: usize,
    selected: bool,
    redraw: bool,
    size: Size,
    options: MenuOptions<C, F>,
    structure: &'a mut [MenuEntry<'a, C, F>],
    last_keys: Keys,
    submenu: Option<usize>,
}

impl<'a, C, F> Menu<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    pub fn new(
        title: &'a str,
        options: MenuOptions<C, F>,
        size: Size,
        structure: &'a mut [MenuEntry<'a, C, F>],
    ) -> Menu<'a, C, F> {
        Self {
            title,
            redraw: true,
            highlighted_option: 0,
            selected: false,
            structure,
            size,
            options,
            last_keys: Keys::default(),
            submenu: None,
        }
    }

    pub fn selected_option(&self) -> Option<&MenuEntry<'a, C, F>> {
        if self.selected {
            Some(&self.structure[self.highlighted_option])
        } else {
            None
        }
    }

    pub fn entry_at(&self, index: usize) -> Option<&MenuEntry<'a, C, F>> {
        self.structure.get(index)
    }

    // fn get_submenu(&self) -> Option<&MenuEntry<'a, C, F>> {
    //     if self.submenu.is_some() {
    //         match self.structure[self.submenu.unwrap()].t {
    //             EntryType::Menu(ref mut menu) =>  {
    //                 Some(menu)
    //             },
    //             _ => None
    //         }
    //     } else {
    //         None
    //     }

    // }

    pub fn update(&mut self, keys: &Keys) -> bool {
        // If the submenu is visible, pass the keys to that instead
        if self.submenu.is_some() {
            match self.structure[self.submenu.unwrap()].t {
                EntryType::Menu(ref mut menu) => {
                    match menu.selected_option() {
                        Some(entry) => match entry.t {
                            EntryType::Return => self.submenu = None,
                            _ => {}
                        },
                        _ => {}
                    }
                    menu.update(keys);
                }
                _ => {}
            }
            return false;
        }

        let mut tmp_opt = self.highlighted_option as i32;

        let tmp_keys = Keys {
            a: keys.a && !self.last_keys.a,
            b: keys.b && !self.last_keys.b,
            left: keys.left && !self.last_keys.left,
            up: keys.up && !self.last_keys.up,
            down: keys.down && !self.last_keys.down,
            right: keys.right && !self.last_keys.right,
        };

        if tmp_keys.up {
            tmp_opt -= 1;
        }
        if tmp_keys.down {
            tmp_opt += 1;
        }

        if tmp_opt >= self.structure.len() as i32 {
            tmp_opt = 0;
        } else if tmp_opt < 0 {
            tmp_opt = self.structure.len() as i32 - 1;
        }

        self.highlighted_option = tmp_opt as usize;

        self.selected = tmp_keys.a;

        if tmp_keys.up || tmp_keys.down || tmp_keys.a || tmp_keys.right || tmp_keys.left {
            self.redraw = true;
        }

        match self.structure[self.highlighted_option as usize].t {
            EntryType::Bool(ref mut val) => {
                if tmp_keys.left || tmp_keys.right || tmp_keys.a {
                    *val = !*val;
                }
            }
            EntryType::I32(ref mut val) => {
                if tmp_keys.right && ((*val).0 < (*val).2) {
                    (*val).0 += 1;
                }
                if tmp_keys.left && ((*val).0 > (*val).1) {
                    (*val).0 -= 1;
                }
            }
            EntryType::Menu(_) => {
                if tmp_keys.a {
                    self.submenu = Some(self.highlighted_option);
                    match self.structure[self.submenu.unwrap()].t {
                        EntryType::Menu(ref mut menu) => {
                            menu.redraw = true;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        self.last_keys = keys.clone();

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
        if self.submenu.is_some() {
            match self.structure[self.submenu.unwrap()].t {
                EntryType::Menu(ref mut menu) => {
                    // if menu.selected_option().is_some(
                    menu.draw(display)?;
                }
                _ => {}
            }
        } else if self.redraw {
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
                self.title,
                self.size.width as i32 / 2 - self.title.len() as i32 * font_width / 2,
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

                self.draw_text(display, self.structure[i].l, text_x, text_y)?;

                match &self.structure[i].t {
                    EntryType::Bool(val) => {
                        let x = match val {
                            true => "<X>",
                            false => "< >",
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
                    _ => {}
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
                    self.structure[self.highlighted_option as usize].l.len() as u32 * 6,
                    1,
                ),
            )
            .into_styled(PrimitiveStyle::with_fill(self.options.text))
            .draw(display)?;
        }
        Ok(())
    }
}
