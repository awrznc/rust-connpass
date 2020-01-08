# rust-connpass

[![Actions Status](https://github.com/awrznc/rust-connpass/workflows/Build/badge.svg)](https://github.com/awrznc/rust-connpass/actions)

[Connpass API](https://connpass.com/about/api/) library in Rust.

## Install

Install openssl through your favourite package. You might also need a C compiler (gcc).

```bash
# Ubuntu, Alpine 
apt-get install libssl-dev pkg-config

# Arch Linux
pacman -S openssl

# Fedora, CentOS8
dnf install openssl-devel

# Amazon Linux
yum install openssl-devel
```

Put the following in your project's Cargo.toml file:

```toml
[dependencies]
connpass = "0.0.1"
```

## Example

```bash
cargo run --example get
```

## Contribute

Any PR is welcomed!

## License

https://github.com/awrznc/rust-connpass/blob/master/LICENSE
