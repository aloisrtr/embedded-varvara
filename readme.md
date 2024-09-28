# embedded-varvara
A Varvara emulator built on top of [embedded-hal](https://github.com/rust-embedded/embedded-hal) 
abstractions.

It is meant ot run with [`baryuxn`](https://github.com/aloisrtr/baryuxn), an expandable
Uxn stack machine onto which you can plug any instance of `VarvaraDeviceBus`!

![Chart of how all components of a full implementation interact together](./assets/embedded-varvara.svg)

## Devices
The Varvara specification defines a set of devices that can be accessed by the
emulated Uxn CPU and their behavior. Below is a list of currently supported devices
with their support status and required crate feature if relevant.

| Device     | Status | Feature    | Issues |
| ---------- | ------ | ---------- | ------ |
| System     | ✅     |            |        |
| Console    | ✅     | `console`  |        |
| Screen     | ✅     | `graphics` |        |
| Audio      | ❌     |            |        |
| Controller | ❌     |            |        |
| Mouse      | ❌     |            |        |
| File       | ❌     |            |        |
| DateTime   | 🚧     | `datetime` | check for daytime savings missing |

## Implementing for a specific set of devices
To add this crate as a dependency, simply run:
```sh
cargo add embedded-varvara
```
or add it to the dependencies of your `Cargo.toml` directly:
```toml
[dependencies]
embedded-varvara = "0.1"
```

`embedded-varvara` is designed to be easy to implement regardless of the hardware
you are using by taking advantage of [Rust's trait system](https://doc.rust-lang.org/book/ch10-02-traits.html).

For example, you can use any screen to display graphics as long as it implements
the traits from [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics)!

The feature system described above lets you turn on or off devices that you
don't need: if your application doesn't need access to time-related stuff, you
can simply opt-out and not be bothered by it.

To turn on some features, the command changes to:
```sh
cargo add embedded-varvara --features <your features>
```
or in your `Cargo.toml`:
```toml
[dependencies]
embedded-varvara = { version = "0.1", features = [<your features>] }
```

### Debugging
The `VarvaraDeviceBus` structure can log information to a serial interface using
[`defmt`](https://github.com/knurling-rs/defmt). This is disabled by default, and
can be enabled by adding the `defmt` feature.
