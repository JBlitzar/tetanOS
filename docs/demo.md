# tetanOS demo

Since this is an operating system rather than a self-contained command-line tool, there's no easy `cargo install` command.

Luckily, there are still two options! I've got a nice demo video, and a `qemu` command for those intrepid enough to try it out for themselves. (qemu emulates an operating system without having to boot from a USB or something, which makes it really great for testing these things).

## Video

![](snake_gameplay.gif)

## Run it for real

First, you'll need to install `qemu` if you haven't already. Specific instructions on [the qemu website](https://www.qemu.org/download/), but here's the basic trichotomy:

- Mac: `brew install qemu`
- Linux: `apt-get install qemu-system` (untested)
- Windows (64-bit only): [see the website.](https://www.qemu.org/download/#windows)

Then download and run!

```bash
curl -LO https://github.com/JBlitzar/tetanOS/releases/download/v0.1.0-snake-1/bootimage-tetanos.bin
qemu-system-x86_64 -drive format=raw,file=bootimage-tetanos.bin
```

> ![NOTE]
> If the download fails, you can grab the file manually from the releases page.

A window will open with the snake game. Click in and use WASD to navigate.
