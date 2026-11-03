// lib.rs
// Date: Wed Oct 14 2026
// Educational and Practice Rust Programming Language Code

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

pub mod message {

    // input variable are &Option<S>
    pub fn display_message_one<S: AsRef<str>>(
        msg_one: &Option<S>,
        msg_two: &Option<S>,
        name: &str,
    ) {
        if let Some(m1) = msg_one {
            println!("{}: message_one: {}", name, m1.as_ref());
        } else {
            println!("{}: there is no message ...", name);
        }

        if let Some(m2) = msg_two {
            println!("{}: message_two: {}", name, m2.as_ref());
        } else {
            println!("{}: there is no message.", name);
        }
    }

    // input variable are Option<S>. there is no reference
    pub fn display_message_two<S: AsRef<str>>(msg_one: Option<S>, msg_two: Option<S>, name: &str) {
        if let Some(m1) = msg_one {
            println!("{}: message_one: {}", name, m1.as_ref());
        } else {
            println!("{}: there is no message ...", name);
        }

        if let Some(m2) = msg_two {
            println!("{}: message_two: {}", name, m2.as_ref());
        } else {
            println!("{}: there is no message.", name);
        }
    }
}

pub mod util {
    pub fn separator32() {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }

    pub fn separator64() {
        println!("----------------------------------------------------------------");
    }
}
