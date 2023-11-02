# rocky
repo to get librocksdb-sys building either via platfrom fixups and cxx_lib or from buildscripts.  If you use the buildscripts, you will get a bindings.rs created and a .a, but if you link the .a via rustc_flags, you'll get a bunch of cxx11 errors, as if something within the dep tree was built with the wrong cxx standard.  If you switch over to just cxx_lib building, skipping the buildscript (and manually yanking the bindings.rs and placing it into the src path), you can get it to build, but it will fail because the .a doesn't get linked in correctly for some reason.  Also, you have to hardcode OPT_LEVEL=0 into the env var for the buildscript if you go down that path.  NB. reindeer buckify will destory this manual change.
## setup
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
## versions
clang
```
ubuntu@librocksdb:~/rust/third-party$ clang --version
Ubuntu clang version 14.0.0-1ubuntu1.1
Target: aarch64-unknown-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
```
ld.lld
```
ubuntu@librocksdb:~/rust/third-party$ ld.lld --version
Ubuntu LLD 14.0.0 (compatible with GNU linkers)
```

Then try to build:

```
buck2 build //rocky:rocky
```

## error if you use the buildscript
```
  = note: ld.lld: error: undefined symbol: rocksdb_free
          >>> referenced by rocky.75efbbde2dcb7026-cgu.0
          >>>               buck-out/v2/gen/root/9f4d83578bb24895/rocky/__rocky__/bin-pic-static_pic-link/extras/rocky/rocky.rocky.75efbbde2dcb7026-cgu.0.rcgu.o:(rocky::main::h9397527e758e9d67)
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

## error if you use the platform fixups to use a cxx_lib
You end up with what appears to be two differen cxx standards mixed at linking (this is just a theory)

```
          ld.lld: error: undefined symbol: std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::assign(char const*, unsigned long)
          >>> referenced by c.cc
          >>>               c.cc.pic.o:(rocksdb_key_may_exist) in archive buck-out/v2/gen/root/9f4d83578bb24895/third-party/__librocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux__/liblibrocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux.pic.a
          >>> referenced by c.cc
          >>>               c.cc.pic.o:(rocksdb_key_may_exist_cf) in archive buck-out/v2/gen/root/9f4d83578bb24895/third-party/__librocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux__/liblibrocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux.pic.a
          >>> referenced by c.cc
          >>>               c.cc.pic.o:(rocksdb_compactionfilter_t::Filter(int, rocksdb::Slice const&, rocksdb::Slice const&, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >*, bool*) const) in archive buck-out/v2/gen/root/9f4d83578bb24895/third-party/__librocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux__/liblibrocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux.pic.a
          >>> referenced 76 more times

          ld.lld: error: undefined symbol: std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::operator=(char const*)
          >>> referenced by c.cc
          >>>               c.cc.pic.o:(rocksdb_set_options) in archive buck-out/v2/gen/root/9f4d83578bb24895/third-party/__librocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux__/liblibrocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux.pic.a
          >>> referenced by c.cc
          >>>               c.cc.pic.o:(rocksdb_set_options_cf) in archive buck-out/v2/gen/root/9f4d83578bb24895/third-party/__librocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux__/liblibrocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux.pic.a
          >>> referenced by c.cc
          >>>               c.cc.pic.o:(rocksdb_options_set_db_log_dir) in archive buck-out/v2/gen/root/9f4d83578bb24895/third-party/__librocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux__/liblibrocksdb-sys-0.11.0+8.1.1-librocksdb-sys-x64_64-linux.pic.a
          >>> referenced 85 more times
```
