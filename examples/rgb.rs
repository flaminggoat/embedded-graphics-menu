use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::{thread, time::Duration};

use embedded_graphics_menu::{EntryType, Keys, Menu, MenuEntry, MenuOptions};
use generic_array::GenericArray;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(128, 160));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("rgb", &output_settings);

    let mut keys = Keys {
        a: false,
        b: false,
        up: false,
        down: false,
        left: false,
        right: false,
    };

    let options = MenuOptions {
        text: Rgb565::WHITE,
        background: Rgb565::BLUE,
        highlight: Rgb565::GREEN,
        border: 15,
        spacing: 17,
        font: embedded_graphics::fonts::Font6x8,
    };
    let menu_structure = GenericArray::from([
        MenuEntry {
            l: "Menu",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Music on",
            t: EntryType::Bool(false),
        },
        MenuEntry {
            l: "Heater",
            t: EntryType::I32((-3, -10, 10)),
        },
    ]);
    let start_menu_structure = GenericArray::from([
        MenuEntry {
            l: "Cake",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Melon",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Cheese",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Back",
            t: EntryType::Select,
        },
    ]);

    let mut m = Menu::new(
        "Shop",
        options.clone(),
        display.bounding_box().size,
        menu_structure,
    );
    let mut m2 = Menu::new(
        "Food Choices",
        options,
        display.bounding_box().size,
        start_menu_structure,
    );

    let mut current_menu = 0;
    let mut heater = -3;

    'running: loop {
        // display.clear(Rgb565::WHITE)?;

        window.update(&mut display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Left => keys.left = true,
                        Keycode::Right => keys.right = true,
                        Keycode::Up => keys.up = true,
                        Keycode::Down => keys.down = true,
                        Keycode::Return => keys.a = true,
                        _ => {}
                    };
                }
                _ => {}
            }
        }

        if current_menu == 0 {
            m.update(&keys);
            m.draw(&mut display).unwrap();

            match m.selected_option() {
                Some(index) => {
                    if index == 0 {
                        current_menu = 1;
                        m2.force_redraw();
                    }
                }
                None => {}
            }
        } else {
            m2.update(&keys);
            m2.draw(&mut display).unwrap();
            match m2.selected_option() {
                Some(index) => {
                    if index == 3 {
                        current_menu = 0;
                        m.force_redraw();
                    }
                }
                None => {}
            }
        }

        // Read heater value from menu
        let h = Some(m.entry_at(2).and_then(|entry| match entry.t {
            EntryType::I32((h, _, _)) => Some(h),
            _ => None,
        }))
        .unwrap_or(None)
        .unwrap_or(0);

        if h != heater {
            heater = h;
            println!("Heater {}", heater);
        }

        keys.a = false;
        keys.b = false;
        keys.up = false;
        keys.down = false;
        keys.left = false;
        keys.right = false;

        thread::sleep(Duration::from_millis(30));
    }
}
