# rocky
an example setup to demo how you can get rocksdb to build with buck2 and reindeer
## setup
I used multipass with ubuntu 22.04 as an env for testing.  As such, my arch was aarch64-unknown-linux-gnu.  Most of the fixups are targeted for this.  You'd need to modify them for x86_64, which i'll do in a later update to this repo.
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
