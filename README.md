# STM32 Blink

This repo hosts the code example of the following tutorial
article: {availaible when published}

This is nothing more than a blinking LED on a Nucleo 64.

Hardware used: Nucleo-64 STM32F401RE

## Running the program

First, install the right target for the board.

```
rustup target add thumbv7em-none-eabi
```

Then, install the `cargo-flash` utility.

```
$ cargo install cargo-flash
```

Finally, flash the program to the device.

```
$ cargo flash --chip stm32f401re --release
```


## Debugging through

1. Flash the card without the `--release` flag to enable debug symbols.
2. Open a terminal and use the `st-util` command (availaible in the `stlink`
    package on archlinux). This will halt the device, initiate a debugging server
    and wait for a debugger to attach to it.
3. In another terminal, open arm-none-eabi-gdb and give it the ELF file as an argument
```
$ arm-none-eabi-gdb ./target/thumbv7em-none-eabi/debug/stm32_blink
```
4. In order to connect to the server, run this command in gdb.
```
$ target extended-remote :4242
```
5. Debug

