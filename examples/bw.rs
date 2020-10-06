use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::{thread, time::Duration};

use embedded_graphics_menu::{EntryType, Keys, Menu, MenuOptions};

fn main() -> Result<(), std::convert::Infallible> {
    let mut bw_display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .scale(1)
        .build();
    let mut bw_window = Window::new("bw", &output_settings);

    let mut keys = Keys {
        a: false,
        b: false,
        up: false,
        down: false,
        left: false,
        right: false,
    };

    let colors = MenuOptions {
        text: BinaryColor::On,
        background: BinaryColor::Off,
        highlight: BinaryColor::On,
        border: 10,
        spacing: 10,
        font: embedded_graphics::fonts::Font6x8,
    };
    let mut menu_structure = [
        ("Start", EntryType::Select),
        ("Sound on", EntryType::Bool(false)),
        ("Volume", EntryType::I32((-3, -10, 10))),
    ];
    let mut m = Menu::new(&colors, bw_display.bounding_box().size, &mut menu_structure);

    'running: loop {
        bw_window.update(&mut bw_display);

        for event in bw_window.events() {
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
        m.draw(&mut bw_display).unwrap();

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
