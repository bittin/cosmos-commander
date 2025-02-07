# cosmic-commander

Dual Panel File manager for the COSMIC desktop environment. Think of a less cluttered Krusader.

This project is based on [COSMIC files](https://github.com/pop-os/cosmic-files).

> [!NOTE]
> This project is very early in the development. It barely displays two panels with   tabs and a button bar. Goal is to have an embedded terminal and full interactivity between the two panels that dual pane file manager users are used to.

## Build the project from source

```sh
# Clone the project using `git`
git clone https://github.com/fangornsrealm/cosmic-commander
# Change to the directory that was created by `git`
cd cosmic-commander
# Build an optimized version using `cargo`, this may take a while
cargo build --release
# Run the optimized version using `cargo`
cargo run --release
```

## License

This project is licensed under [GPLv3](LICENSE)
