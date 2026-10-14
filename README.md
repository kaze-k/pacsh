# pacsh

A fast and lightweight SSH login banner generator written in Rust.

Display useful information when logging into your server through SSH, including system info, user sessions, terminal information, and remote connection details.

## Features

- 🦀 Written in Rust
- ⚡ Fast startup time
- 🪶 Lightweight
- 🎨 Colored terminal output
- 🧩 Modular internal design
- 🔐 SSH session information

## Example

```text
╭── 󰮯 ───────────────────────────────────╮
│                                        │
│    [ok]      SSH LOGIN SUCCESSFUL      │
│    [ok]      SESSION kaze@kaze-arch    │
│                                        │
│    OS        Arch Linux                │
│    TTY       /dev/pts/0                │
│    FROM      192.168.10.7:6119         │
│    TO        192.168.10.5:22           │
│    SESSIONS  2                         │
│                                        │
╰─────────────────────────────────── 󰊠 ──╯
```

## Information

- **SSH LOGIN SUCCESSFUL**
  - Shows the current SSH login status.

- **SESSION**
  - Displays the current user and hostname.

- **OS**
  - Shows the operating system information.

- **TTY**
  - Displays the current terminal device.

- **FROM**
  - Shows the remote client address and port.

- **TO**
  - Shows the destination SSH server address and port.

- **SESSIONS**
  - Displays the number of active login sessions.

## Dependencies

- [Nerd Fonts](https://www.nerdfonts.com/)

## Installation

```bash
git clone https://github.com/kaze-k/pacsh.git
cd pacsh
cargo build --release
```

Copy the binary:

```bash
sudo install target/release/pacsh /usr/local/bin/
```

Then add `pacsh` to your shell startup file.

Configure SSH to run pacsh on login by adding the following to /etc/ssh/sshrc:

```bash
if [ -t 1 ]; then # Avoid running for non-interactive SSH sessions (e.g. stf)
    pacsh
fi
```

## License

GPL-3.0 license
