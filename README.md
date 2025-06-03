# commander

Dual Panel File manager for the COSMIC desktop environment. Think of a less cluttered Krusader.

![commander](assets/commander.png)

This project is based on [COSMIC files](https://github.com/pop-os/cosmic-files) and the Terminal on [COSMIC term](https://github.com/pop-os/cosmic-term).

> [!NOTE]
> The basic functionality is working. Two panels, copying or moving files or tabs between them. A Terminal to run commands. And all the features of COSMIC Files that it inherits. Good enough to close krusader and use commander instead.
>
> Drag'nDrop is only working out of Commander panels into other programs. Dropping files into directories or paths into the terminal does not work. Implementing a drop-target for pane_grid resulted in a drop-area that does not call the specified handler functions. The problem has to be deeper in the guts of iced.

## Install

```sh
# Clone the project using `git`
git clone https://github.com/fangornsrealm/commander
# Change to the directory that was created by `git`
cd commander
# Build an optimized version using `cargo`, this may take a while
cargo build --release
# install
sudo just install
```

## Packaging

```sh
# Clone the project using `git`
git clone https://github.com/fangornsrealm/commander
# Change to the directory that was created by `git`
cd commander
# Build an optimized version using `cargo`, this may take a while
just build-release
# build Debian / Ubuntu package
just build-deb

# build Redhat / Fedora / Suse package 
just build-rpm
```

## Build the project from source

```sh
# Clone the project using `git`
git clone https://github.com/fangornsrealm/commander
# Change to the directory that was created by `git`
cd commander
# Build an optimized version using `cargo`, this may take a while
cargo build --release
# Run the optimized version using `cargo`
cargo run --release
```

## License

This project is licensed under [GPLv3](LICENSE)
