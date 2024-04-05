ravedude-test
=============

Rust project for the _Arduino Uno_.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## Hardware

- https://learn.sparkfun.com/tutorials/button-pad-hookup-guide
- https://bettersilicone.en.made-in-china.com/product/ZFOfeRTrnuVP/China-LED-Compatible-Transparent-Conductive-4X4-Keypad-Silicone-Rubber-Button-Pad-for-MIDI-Controller-Keyboard.html
- https://www.adafruit.com/product/3954
- https://crates.io/crates/trellis_m4
- https://llllllll.co/t/diy-monome-compatible-grid-w-adafruit-neotrellis/28106/6?page=65
  
## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
