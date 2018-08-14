extern crate linux_embedded_hal as hal;
extern crate embedded_graphics;
extern crate ssd1306;
extern crate machine_ip;

use hal::{Spidev, Pin};
use hal::sysfs_gpio::Direction;
use hal::spidev::SpidevOptions;
use hal::Delay;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
use embedded_graphics::fonts::Font6x8;
use ssd1306::prelude::*;
use ssd1306::Builder;

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").expect("Could not open SPI device");
    let options = SpidevOptions::new().max_speed_hz(50_000).build();
    spi.configure(&options).expect("SPI configure error");

    let mut reset = Pin::new(24);
    reset.export().unwrap();
    while !reset.is_exported() {}
    reset.set_direction(Direction::Out).unwrap();

    let dc = Pin::new(25);
    dc.export().unwrap();
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).unwrap();

    let mut delay = Delay {};

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut reset, &mut delay);
    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(Line::new((8, 16 + 16), (8 + 16, 16 + 16), 1).into_iter());
    disp.draw(Line::new((8, 16 + 16), (8 + 8, 16), 1).into_iter());
    disp.draw(Line::new((8 + 16, 16 + 16), (8 + 8, 16), 1).into_iter());

    disp.draw(Rect::new((48, 16), (48 + 16, 16 + 16), 1u8).into_iter());

    disp.draw(Circle::new((96, 16 + 8), 8, 1u8).into_iter());

    let local_addr = machine_ip::get().unwrap();

    disp.draw(
            Font6x8::render_str(&format!("IP: {}", local_addr.to_string()))
                .translate((0, 56))
                .into_iter(),
        );

    disp.flush().unwrap();
}
