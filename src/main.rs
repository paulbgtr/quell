#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos::hello;

fn main() {
    hello();
}
