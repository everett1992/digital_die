# Digital Die

This is my first hardware project, a 'pocket' calculator for D&D and games that use dice.

# Components
 - [3d printed case](https://cad.onshape.com/documents/30c5ec5ec9154a7320fca1ce/w/ae80ca337ce29560b28469cb/e/df584e26d620b28a3dece1ff)
 - [feather m0 express](https://www.adafruit.com/product/1781)
 - [128x32 2.3" OLED](https://www.adafruit.com/product/2675)
 - 14 cherry mx knockoff switches
 - [2200mAh lithium ion battery](https://www.adafruit.com/product/1781)

You can find significanly cheaper micro controllers and displays, I would shop
around before building another.

# Dependencies.

This package uses binutils and
[uf3conv-rs](https://github.com/sajattack/uf2conv-rs), installed via cargo.
It targets thumbv6-none-eabi. It uses rust stable, 1.33.0.

```
$ cargo install cargo-binutils
$ cargo install --git https://github.com/sajattack/uf2conv-rs
$ rustup target add thumbv6m-none-eabi
```

The package was setup following the rust [embedded
book](https://rust-embedded.github.io/book/intro/install.html)

# Build and flash

The flash bash script compiles `digital_die` and creates a uf2 file. It will
mount and copy the file to the device `$0` then unmount the device.

```
$ ./flash /dev/sdb
```
