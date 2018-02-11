# rust-os

Experimental Project.

## Getting Started

Using `Xargo`. First of all, you need to install `Xargo` to your project.

```
cargo install Xargo
```

If you're getting a linking error about LLD, you need to install lld too.

Let's try to build:

```
RUST_TARGET_PATH=[YOUR DIR PATH]  xargo build --target x86_64-blog_os
```