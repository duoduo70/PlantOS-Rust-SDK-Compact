# How to use
This is an template project.

You must installed "Make" 、"cargo" 、"QEMU" and "xargo".
Your cargo version must be Nightly.

How to install "xargo":
First, use `cargo install xargo`.
After that, set your `PATH` environment variable. Generally, edit your `~/.bashrc`:
```
# vi ~/.bashrc
export PATH=$PATH:~/.cargo/bin
# :eq
```

If you want to run the template program, use `sh ./run.sh`.
It should show a QEMU window, use command `C:` and `my-prog` in it.
You should saw `hello, rust!`.

If you want to create your Rust program, change `Cargo.toml` and `Makefile` in "my-prog" to the name of you want to.