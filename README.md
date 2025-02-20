# [`dioxide_os`](https://github.com/dioxide-os/dioxide)

Dioxide is an experimental OS built in pure Rust for educational purposes. Right now, it doesn't do a whole lot. It was made by following the excellent material by phil-opp, [Writing an OS in Rust](https://os.phil-opp.com/).

You will find most of the code for the kernel in the `crates/dioxide` directory.

## Building and Running

Since this is purely experimental, this project uses the **nightly** branch of Rust. Breaking changes are expected frequently.

To build the code, just run `cargo build`. This should download any required dependencies.

To run the code, if you have QEMU or VirtualBox, you can run `cargo run --features=["qemu", "uefi"]` with the features chosen. The VirtualBox version assumes some setup which I haven't outlined here yet, so QEMU might be the easiest to get going.
