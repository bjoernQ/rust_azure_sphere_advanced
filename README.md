PLEASE FIND A MORE COMPLETE AND UP-TO-DATE SOLUTION AT https://github.com/grandcentrix/rust-on-azure-sphere

# Advanced stuff in Rust for Azure Sphere

Here you can find an example of using Rust to develop something for Azure Sphere.
It opens a server on port 5000 and implements a simple protocol to control the four leds of the dev board.

It features a custom allocator, threads and network IO.

## How to build

Make sure to have your Rust environment switched to `nightly` and have xargo installed.

While it might work without it's a good idea to do `set RUST_TARGET_PATH=...path where the target json resides...` - it's needed if you add dependencies to Cargo.
There is a `setenv.bat` that does it for you.

Go to the `sphere_advanced` directory and do `xargo build --target arm-v7-none-eabi` after the build run `package.bat` to get the application packaged for Azure Sphere.

Now you can deploy it to your Sphere dev board via `deploy.bat`

Before deploying the binary make sure to have your dev board connected to your wifi and check the address with `azsphere device wifi show-status`

When the application is running just connect to it via telnet on port 5000.

The server will greet you with `hello`

Enter a command which looks like this
`[1234][01][01][01]` which is "number of the led", "red", "green" and "blue". 0 means off, 1 means on.

e.g. `2101` turn led 2 red on, green off and blue on.

You can exit the session with `exit`

## Unit Tests

This also contains unit tests running on the host system. Just use `cargo test` (not xargo) to run them.
There is not much tested but it shows how to do it in general.

## Debug

With `debug.bat` you can start a debugging session. It starts telnet showing the debug output (looks awful)
and gdb.

Enter `target remote 192.168.35.2:2345` into gdb to attach the debugger - then enter `c` to continue (and actually run the application).

Since `package.bat` strips the binary this isn't too useful - you need to remove / comment out the strip from `package.bat` for real debugging.

## Please note

Rust is completely new to me so this might be not the ideal and most idiomatic way to do it.

The path to the Azure Sphere SDK is hardcoded and it' assumed to be installed to the default location.

This proof-of-concept uses the Sphere Beta1902 SDK since it creates a server port.
