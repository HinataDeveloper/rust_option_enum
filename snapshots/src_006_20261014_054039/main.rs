// main.rs
// Date: Wed Oct 14 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning Chapter 15
// Goal: Reference Cycle causes Memory Leak
// Dependency: Without dependency

// rustc 1.101.0-nightly (21b707e3f 2026-09-30)
// binary: rustc
// commit-hash: 21b707e3f97e0b522ebd2f277a862339625ad83f
// commit-date: 2026-09-30
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.1

// cargo 1.101.0-nightly (f3865b2a4 2026-09-29)
// release: 1.101.0-nightly
// commit-hash: f3865b2a4d1acc5276f6b3c67d0e057f4dab3928
// commit-date: 2026-09-29
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.9-201.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use using_option::message;
use using_option::util;

fn main() {
    println!("\n");

    let message_one: Option<&str> = Some("I am a Rust Developer ...");
    let message_two: Option<&str> = Some("I am a Zig Developer ...");

    let message_three: Option<String> = Some(String::from("I am C/C++ Developer ..."));
    let message_four: Option<String> = Some(String::from("I am Python Developer ..."));

    message::display_message_one(&message_one, &message_two, "A");
    util::separator32();

    message::display_message_one(&message_three, &message_four, "B");
    util::separator32();

    util::separator64();

    message::display_message_one(&message_one, &message_two, "C");
    util::separator32();

    message::display_message_one(&message_three, &message_four, "D");
    util::separator32();

    util::separator64();

    message::display_message_two(message_one, message_two, "E");
    util::separator32();

    message::display_message_two(message_three, message_four, "F");
    util::separator32();

    util::separator64();

    // program run without any problem. because message_one and message_two are
    // Option<&str> So They had been borrowed to the display_message_two
    message::display_message_two(message_one, message_two, "E");
    util::separator32();

    // encounter with compile time error. because message_three and message_four are
    // Option<String> data type So they was moved to display_message_two
    // message::display_message_two(message_three, message_four, "F");
    util::separator32();

    println!("\nThe End ...\n");
}
