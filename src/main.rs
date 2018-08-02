extern crate assert_cmd;
extern crate predicates;

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

fn main() {
    const TOO_MUCH: usize = 1024 * 1024;
    let buf = vec![0; TOO_MUCH];
    Command::new("cat")
        .with_stdin()
        .buffer(buf.clone())
        .assert()
        .stdout(predicate::eq(buf.as_slice()));
}
