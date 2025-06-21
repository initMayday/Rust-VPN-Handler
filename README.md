# Rust VPN Handler
This is a simple script, written in rust, which allows you to easily enable a wireguard interface, disable one, toggle between them, and see which is currently active. This is mainly for personal usage. Which interface is chosen, is at random. It internally just calls wg-quick.

## Usage
1) Place all desired interfaces (.conf files) in /etc/wireguard.
2) Bind shortcuts to sudo vpn_handler [arguments]. Please ensure you have a way of answering the sudo request command, eg. by pre-pending konsole -e, or another terminal (konsole -e sudo vpn_handler up, in my case)
3) Call the executable on startup, eg. via a systemd service, if you want it to activate on startup.

## List of arguments
swap - Swaps the active interface with another (internally just calls down then up)
up - Brings an interface up, if one is not already active
down - Brings the active interface down, if there is one
which - Prints the active interface's name
help - displays the commands

## Building
Clone the repository, then simply run "cargo build --release", if you just want the pure executable. Otherwise, on arch-based distributions, run makepkg -si in the cloned repository to install the program. Dependencies are openresolv (or alternatives) and wireguard-tools, the makepkg automatically installs these otherwise you need to ensure you have them when building with cargo manually.

## Licensing
Rust-VPN-Handler utilises the CC BY-NC-SA 4.0 license, where the share-alike term refers to also sharing the source code. Please see [REPO NAME] for more details.

