# tetanOS

<img src="docs/logo.svg" style="width: 100px;">

> _written in Rust, runs on bare metal_

A minimal OS written in Rust.

# Progress (reverse chronological order)

Got snake working!!

![](docs/snake_gameplay.gif)

I've decided to go off-piste to the blog post and I implemented (hacky) keyboard polling
![](docs/video_typing1.gif)

Got a VGA buffer implementation up and running, so now we have proper `println!`
![](docs/vga_buffer.png)

So far, I've gotten arbitrary strings writable to the VGA buffer in a bootable disk image
![](docs/hello_world.png)

# Quickstart with a prebuilt binary

(generated with `cargo bootimage --release` in `target/x86_64-tetanos/release/bootimage-tetanos.bin`)

```bash
wget https://github.com/JBlitzar/tetanOS/releases/download/v0.1.0-snake-1/bootimage-tetanos.bin
qemu-system-x86_64 -drive format=raw,file=bootimage-tetanos.bin
```

# Running (development)

```bash
git clone https://github.com/JBlitzar/tetanOS && cd tetanOS
cd tetanos
cargo install bootimage
rustup component add rust-src --toolchain nightly-aarch64-apple-darwin
rustup component add llvm-tools-preview
brew install qemu
cargo run
```

# License

[it's the GNU GPLv3.](LICENSE)

# credit

Huge thanks to https://os.phil-opp.com/ -- tetanOS is based on it, and I wouldn't be able to do this without this guide.
