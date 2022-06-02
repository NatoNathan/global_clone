# Global_clone

Global_clone is a simple CLI tool for cloning git repositories to templated paths.

This is a rust re-implementation of [global-git-clone](https://github.com/NatoNathan/global-git-clone), originally written in typescript for NodeJS.
I started this project manly as a way to learn and practice rust. 

I would recommend this implementation over global-git-clone as it's a bit more stable.

If you use this project and come across any issues please open an issue on github. 

## Usage

```sh
$ global_clone clone [options] <repository> -t <template>
```


## Installation

Latest release can be found on [GitHub](https://github.com/natonathan/global_clone/releases).

Currently I have only setup amd64 builds, but do plain to add arm64 builds soon. 

- Debian or Ubuntu including WSL (amd64): Download and install using deb package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_linux_amd64.deb -O
sudo dpkg -i global_clone_linux_amd64.deb
```

- Fedora or CentOS including WSL (amd64): Download and install using rpm package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_linux_amd64.rpm -O
sudo rpm -i global_clone_linux_amd64.rpm
```

- linux (amd64) including WSL : Download and install using tar.gz package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_linux_amd64.tar.gz -O
tar -xzf global_clone_linux_amd64.tar.gz
sudo cp global_clone_linux_amd64/global_clone /usr/local/bin/global_clone # or wherever you want to install it Just make sure it on the PATH
```

- MacOS (amd64): Download and install using tar package.

Due to MacOS codesiging, you will not be able run the MacOS binary if you don't download though the terminal or compile it your self.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_macos_amd64.tar.gz -O
tar -xvf global_clone_macos_amd64.tar.gz
sudo cp global_clone_macos_amd64/global_clone /usr/local/bin/global_clone
```

- Windows (amd64): Download [here](https://github.com/natonathan/global_clone/releases/latest/download/global_clone.exe) and move to the appropriate directory (e.g. C:\Program Files\global_clone) so it's on the PATH.

## Build Instructions

You will need the rust toolchain, I recommended you use [rustup](https://rustup.rs/)

rustup should ensure all build dependencies are installed.

1. Clone the repository
2. Run `cargo build` or `cargo run` this will download and compile all dependencies before building the project.

## License

MIT License (see [License](./License) )
