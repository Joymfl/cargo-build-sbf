//! Tests for the checks `cargo-build-sbf` runs on the built program before
//! writing the final `.so`.

use {
    assert_cmd::cargo::cargo_bin_cmd,
    predicates::prelude::*,
    std::{fs, path::Path},
};

#[macro_use]
extern crate serial_test;

#[test]
#[serial]
fn test_sol_alloc_free_rejected_on_sbpfv3() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("crates")
        .join("alloc-free");
    let target = crate_dir.join("target");
    let program_so = target.join("deploy").join("alloc_free.so");
    // Start clean so a stale `.so` can't mask a regression.
    let _ = fs::remove_dir_all(&target);

    cargo_bin_cmd!("cargo-build-sbf")
        .arg("--manifest-path")
        .arg(crate_dir.join("Cargo.toml"))
        .args(["--arch", "v3"])
        .assert()
        .failure()
        // 2213547663 is the murmur3 hash of `sol_alloc_free_`.
        .stderr(predicate::str::contains(
            "Invalid syscall code 2213547663, Attempted to allocate to heap, but no allocator \
             found. Define your own with: \
             https://doc.rust-lang.org/beta/std/alloc/trait.GlobalAlloc.html",
        ));
    assert!(!program_so.exists());

    fs::remove_dir_all(target).expect("Failed to remove target dir");
}
