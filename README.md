# rocky
steps to get your env setup to build this
```
sudo apt-get update && sudo apt-get upgrade -y
sudo apt-get install \
build-essential clang cmake pkg-config libssl-dev lld liblz4-dev -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly-2023-07-10
cargo +nightly-2023-07-10 install --git https://github.com/facebook/buck2.git buck2
cargo install --locked --git https://github.com/facebookincubator/reindeer reindeer
```

Then try to build:

```
buck2 build //rocky:rocky
```
