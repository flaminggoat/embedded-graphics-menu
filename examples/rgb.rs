use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::{thread, time::Duration};

use embedded_graphics_menu::{EntryType, Keys, Menu, MenuEntry, MenuOptions};

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

    let mut food_menu_structure = [
        MenuEntry {
            l: "Cake",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Melon",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Cheese 1",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Cheese 2",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Cheese 3",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Cheese 4",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Cheese 5",
            t: EntryType::Select,
        },
        MenuEntry {
            l: "Back",
            t: EntryType::Return,
        },
    ];

    let options = MenuOptions {
        text: Rgb565::WHITE,
        background: Rgb565::BLUE,
        highlight: Rgb565::GREEN,
        border: 15,
        spacing: 15,
        font: embedded_graphics::fonts::Font6x8,
    };

    let mut food_menu = Menu::new(
        "Food Choices",
        options.clone(),
        display.bounding_box().size,
        &mut food_menu_structure,
    );

    let mut menu_structure = [
        MenuEntry {
            l: "Menu",
            t: EntryType::Menu(&mut food_menu),
        },
        MenuEntry {
            l: "Music on",
            t: EntryType::Bool(false),
        },
        MenuEntry {
            l: "Heater",
            t: EntryType::I32((-3, -10, 10)),
        },
    ];

    let mut m = Menu::new(
        "Shop",
        options,
        display.bounding_box().size,
        &mut menu_structure,
    );

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

        m.update(&keys);
        m.draw(&mut display).unwrap();

        match m.selected_option() {
            Some(entry) => {
                // println!("Selected: {}", entry.l);
            }
            None => {}
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
