// Integration test for SigmaOS-godot-rust
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use godot_rust as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
