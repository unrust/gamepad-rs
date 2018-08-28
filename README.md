# unrust / gamepad-rs

[![Build Status](https://travis-ci.org/unrust/gamepad-rs.svg?branch=master)](https://travis-ci.org/unrust/gamepad-rs)
[![Documentation](https://docs.rs/gamepad-rs/badge.svg)](https://docs.rs/gamepad-rs)
[![crates.io](https://meritbadge.herokuapp.com/gamepad-rs)](https://crates.io/crates/gamepad-rs)

This library is a part of [Unrust](https://github.com/unrust/unrust), a pure rust native/wasm game engine.
This library provides a windows, linux, MacOS native gamepad support in rust language.

**This project is under heavily development, all api are very unstable until version 0.2**

## Usage

```toml
[dependencies]
gamepad-rs = "0.1.*"
```

```rust
extern crate gamepad_rs;

use std::thread;
use std::time::Duration;

use gamepad_rs::*;

pub fn main() {
    let mut controller = ControllerContext::new().unwrap();

    loop {
        println!("{} devices", controller.scan_controllers());
        for i in 0..MAX_DEVICES {
            controller.update(i);
            let status = controller.state(i).status;
            if status == ControllerStatus::Connected {
                let nb_buttons;
                let nb_axis;
                {
                    let info = controller.info(i);
                    nb_buttons = info.digital_count;
                    nb_axis = info.analog_count;
                    println!(
                        "[{}] {} {} buttons {} axis",
                        i, info.name, info.digital_count, info.analog_count
                    );
                }
                {
                    let state = controller.state(i);
                    print!("\tbuttons :\n\t  A  B  X  Y  Up Do Le Ri St Bk Lt Rt LB RB\n\t");
                    for i in 0..nb_buttons {
                        print!("  {}", if state.digital_state[i] { 1 } else { 0 });
                    }
                    println!();
                    print!(
                        "\taxis :\n\t  ThumbLX  ThumbLY  LTrigger RTrigger ThumbRX  ThumbRY \n\t"
                    );
                    for i in 0..nb_axis {
                        print!("  {:1.4}", state.analog_state[i]);
                    }
                    println!();
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
```

## Build

Compilation works with current stable Rust (1.28)

```
rustup override set stable
cargo run --example basic --release
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

