# Embedded Blockchain
## flashing
```sh
$ cargo embed --chip STM32L476RGT6 # to flash and attach probe
$ cargo flash --connect-under-reset --chip STM32L476RGTx # to reset, then flash
```

## semihosting
run this command in the root folder
```sh
$ openocd
```
and in a different terminal run this in the root folder
```sh
$ gdb -q target/thumbv7em-none-eabi/$DEBUG_OR_RELEASE/embedded_blockchain

(gdb) target remote :3333

(gdb) load

(gdb) monitor arm semihosting enable

(gdb) continue
```
this will print anything that the card prints into the OpenOCD console.

Or you can run the preconfigured gdb script 
```sh
$ gdb -x openocd.gdb target/thumbv7em-none-eabi/$RELEASE_OR_DEBUG/embedded_blockchain
```

alternatively, you can use `cargo` to configure GDB
```sh
$ cargo +nightly run --release
```