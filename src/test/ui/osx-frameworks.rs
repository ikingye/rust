// ignore-macos this is supposed to succeed on osx

#[link(name = "foo", kind = "framework")]
extern "C" {}
//~^^ ERROR: native frameworks are only available on macOS

fn main() {}
