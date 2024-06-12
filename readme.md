# embedded-varvara
A Varvara emulator built on top of [embedded-hal](https://github.com/rust-embedded/embedded-hal) 
abstractions.

# Devices
The Varvara specification defines a set of devices that can be accessed by the
emulated Uxn CPU and their behavior. Below is a list of currently supported devices
with their support status and required crate feature if relevant.

| Device     | Status | Feature    | Issues |
| ---------- | ------ | ---------- | ------ |
| System     | ✅     |            |        |
| Console    | ✅     |            |        |
| Screen     | ✅     | `graphics` |        |
| Audio      | ❌     |            |        |
| Controller | ❌     |            |        |
| Mouse      | ❌     |            |        |
| File       | ❌     |            |        |
| DateTime   | 🚧     | `chrono`   | check for daytime savings missing |
