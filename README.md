LiftInstall
===========

[![Build Status](https://travis-ci.org/j-selby/liftinstall.svg?branch=master)](https://travis-ci.org/j-selby/liftinstall)

- Usage Documentation: https://liftinstall.jselby.net/

An installer for your application. Designed to be customisable to the core, hookable from external
 applications, and have a decent UI.

This is designed to be a more modern interpretation of Qt's Installer Framework, which has several issues:
- Hard to develop on and poorly documented
- Hardcoded package listing format, requires very specific setups for packages, packages must be built
    using their tool
- Poorly supported, with rare updates and a large list of bugs

Building
--------

For more detailed instructions, look at the usage documentation above.

There are are few system dependencies depending on your platform:
- For all platforms, `cargo` should be available on your PATH. [Rustup](https://rustup.rs/) is the 
  recommended way to achieve this. Stable or Nightly Rust works fine.
- For Windows (MSVC), you need Visual Studio installed.
- For Windows (Mingw), you need `gcc`/`g++` available on the PATH.
- For Mac, you need Xcode installed, and Clang/etc available on the PATH.
- For Linux, you need `gcc`/`g++`, `webkit2gtk`, and `libssl`. For Ubuntu 18.04 this would look like:

```bash
apt install -y build-essential libwebkit2gtk-4.0-dev libssl-dev
```

In order to build yourself an installer, as a bare minimum, you need to:

- Add your favicon to `static/favicon.ico`
- Modify the bootstrap configuration file as needed (`config.PLATFORM.toml`).
- Have the main configuration file somewhere useful, reachable over HTTP.
- Run:

```bash
cargo build --release
```

Contributing
------------

PRs are very welcome. Code should be run through [Rustfmt](https://github.com/rust-lang-nursery/rustfmt) 
 before submission.

License
-------

LiftInstall is licensed under the Apache 2.0 License, which can be found in [LICENSE](LICENSE).
