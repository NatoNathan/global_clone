# Global_clone

Global_clone is a tool for cloning a repository to templated paths.


## Installation

Latest release can be found at [GitHub](https://github.com/natonathan/global_clone/releases).

- Debian or Ubuntu (amd64): Download and install using deb package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_linux_amd64.deb -O
sudo dpkg -i global_clone_linux_amd64.deb
```

- Fedora or CentOS (amd64): Download and install using rpm package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_linux_amd64.rpm -O
sudo rpm -i global_clone_linux_amd64.rpm
```

- linux (amd64): Download and install using tar.gz package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_linux_amd64.tar.gz -O
tar -xzf global_clone_linux_amd64.tar.gz
sudo cp global_clone_linux_amd64/global_clone /usr/local/bin/global_clone # or wherever you want to install it Just make sure it on the PATH
```

- MacOS (amd64): Download and install using tar package.

```bash
curl -fsSL github.com/NatoNathan/global_clone/releases/latest/download/global_clone_macos_amd64.tar.gz -O
tar -xvf global_clone_macos_amd64.tar.gz
sudo cp global_clone_macos_amd64/global_clone /usr/local/bin/global_clone
```

- Windows (amd64): Download [here](https://github.com/natonathan/global_clone/releases/latest/download/global_clone.exe) and move to the appropriate directory (e.g. C:\Program Files\global_clone) so it's on the PATH.

## Usage

```bash
$ global_clone [options] <repository> -t <template>
```
