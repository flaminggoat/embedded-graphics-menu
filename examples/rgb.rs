use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::{thread, time::Duration};

use embedded_graphics_menu::{EntryType, Keys, Menu, MenuOptions};

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
    let mut menu_structure = [
        ("Start", EntryType::Select),
        ("Sound on", EntryType::Bool(false)),
        ("Volume", EntryType::I32((-3,-10, 10))),
    ];
    let mut start_menu_structure = [
        ("Cake", EntryType::Select),
        ("Mouse", EntryType::Select),
        ("Melon", EntryType::Select),
        ("Back", EntryType::Select),
    ];

    let mut m = Menu::new(&options, display.bounding_box().size, &mut menu_structure);
    let mut m2 = Menu::new(
        &options,
        display.bounding_box().size,
        &mut start_menu_structure,
    );

    let mut current_menu = 0;

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

        // if g.game_loop(&keys, &mut display).unwrap() == false {
        //     break 'running Ok(())
        // }

        keys.a = false;
        keys.b = false;
        keys.up = false;
        keys.down = false;
        keys.left = false;
        keys.right = false;

        thread::sleep(Duration::from_millis(30));
    }
}
