# Threshold Rust U2F

A software-only [Universal 2nd Factor](https://www.yubico.com/solutions/fido-u2f/) token. Supports Google Chrome and Firefox on Linux. Written in [Rust](https://www.rust-lang.org/).

## Security

Disclaimer: This is a personal project, I am not a security expert and make no guarantee of security.

Like any U2F authenticator this program provides a degree of protection against phishing and poorly chosen passwords. It does **not** provide the same level of protection against malware that a hardware authenticator does. For some people the protection against phishing and convenience may be worth the security trade-off.

If your machine is compromised by malware, the attacker could steal a copy of the secret keys stored by this authenticator. In this situation you should immediately unregister this authenticator anywhere it is registered in addition to changing the passwords of any potentially compromised accounts. With a hardware authenticator secret keys never leave the device, so in the case of malware you can simply unplug from the infected machine and be confident your accounts are safe from further compromise.

## Installation

### From source (tested on Fedora)

#### Build client
Install rust (tested with version: rustc 1.38.0-nightly (0b680cfce 2019-07-09)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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

#### Build local server
Install node required for local server
```
sudo dnf module install nodejs:10
```
Build server
```
cd local-server
cargo build
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
