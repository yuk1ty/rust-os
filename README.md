# rust-os

Experimental Project. Operation System written in Rust.

## Getting Started

Using `Xargo`. First of all, you need to install `Xargo` to your project.

```
cargo install Xargo
```

If you're getting a linking error about LLD, you need to install lld too.

## Build

Let's try to build:

```
./build.sh
```

## Boostrap

You need to `bootimage` in cargo to create bootstrap image and [QEMU](https://www.qemu.org/) to launch GUI.

```
cargo install bootimage
```

You can launch this OS as below:

```
./bootstrap.sh
```