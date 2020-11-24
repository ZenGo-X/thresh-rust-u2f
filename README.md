# Threshold Rust U2F

A software U2F [Universal 2nd Factor](https://www.yubico.com/solutions/fido-u2f/) implementation using threshold signatures.

The implementation uses a server side and a client side to create a valid U2F signature using threshold signatures.
It uses [Gotham-city](https://github.com/ZenGo-X/gotham-city) for the server and client sides.
The client side emulates a USB communication and runs as a software daemon.

The project is heavily inspired and based on [rust-u2f](https://github.com/danstiner/rust-u2f.git), with the addition of TSS functionality.

## Installation

### From source (tested on Fedora)

Install rust (tested with version: rustc 1.40.0-nightly (084beb83e 2019-09-27))

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly-2019-09-28-x86_64-unknown-linux-gnu
rustup defualt nightly-2019-09-28-x86_64-unknown-linux-gnu
```

Install dependencies

```
sudo yum install make
sudo yum -y update glibc
sudo yum install gmp
sudo yum install gmp-devel
sudo yum install openssl
sudo yum install openssl-devel
sudo yum install clang
sudo yum install systemd-devel
sudo yum install dbus-devel
sudo yum install selinux-policy-devel
sudo yum install rpm-build
```

#### Build client

```
cd linux
cargo build
make install
```

Change the storage from keyring to file

```
vim ~/.config/rustu2f/config.json
```

Change `SecretService` to `File`

#### Build local-server

Install node required for local server

```
sudo dnf module install nodejs:10
```

Build server

```
cd local-server
cargo build
```

## Running

Once all the dependencies are installed, run the client and server side on two different terminals.

### Running server

```
cd local-server
cargo build
./target/debug/local-server
```

### Running client

```
cd linux
cargo build
./target/debug/softu2f-user-daemon
```

## Testing

Test `u2f-core` functionality

```
cd u2f-core
cargo test
```

### Trying it out

Visit Yubikey [demo](https://demo.yubico.com/webauthn-technical/registration) and see how it works.
