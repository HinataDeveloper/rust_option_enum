// main.rs
// Date: Tue Oct 13 2026
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

// Kernel Version: 7.2.8-201.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use using_option::display_message;
use using_option::separator;

fn main() {
    println!("\n");

    // making message_one variable.
    let message_one: Option<String> = Some(String::from("I am a beginner Rustacean."));

    // making message_two variable.
    let message_two: Option<String> = Some(String::from("I am a C++ Developer."));

    // passing a clone of message_one duo to move operation.
    display_message(message_one.clone(), None, "A");
    separator();

    // specifying type is mandatory. because input data must know
    // its data type. duo to both input variable are same type as it
    // was indicated in function syntax as a generic, it is not
    // require to annotate data type for both input variable.
    display_message(None::<String>, None, "B");
    separator();

    display_message(None::<&str>, None, "C");
    separator();

    display_message(None, None::<String>, "D");
    separator();

    display_message(None, None::<&str>, "E");
    separator();

    display_message(message_one, message_two, "F");
    separator();

    // Encounter with compile time error.
    // Because msg_one and msg_two must exactly be same type.
    //display_message(message_one, Some("I am a C++ Developer ..."));

    println!("\nThe End ...\n");
}
