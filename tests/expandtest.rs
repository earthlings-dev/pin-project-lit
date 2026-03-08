// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(not(miri))]

#[allow(clippy::ignore_without_reason)] // reason cannot be added to rustversion::attr-generated ignore
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn expandtest() {
    let args = &["--all-features"];
    macrotest::expand_args("tests/expand/**/*.rs", args);
}
