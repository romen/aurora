#![deny(unexpected_cfgs)]

mod common;

#[allow(unused_imports)] // only used if any of the tests are enabled
use common::decode_tests::{generate_all_tests, test_structs, TestParam};
use std::{path::PathBuf, sync::LazyLock};

#[allow(dead_code)] // it is only used if any of the tests are enabled
static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("openssl35")
});

#[cfg(feature = "_mldsa")]
mod mldsa {
    use super::*;
    use test_structs::MLDSA44Tests;
    use test_structs::MLDSA65Tests;
    use test_structs::MLDSA87Tests;

    generate_all_tests!(data_dir=&DATA_DIR; MLDSA44Tests, MLDSA65Tests, MLDSA87Tests);
}

#[cfg(feature = "_slhdsa")]
mod slhdsa {
    use super::*;
    use test_structs::SLHDSASHAKE128fTests;
    use test_structs::SLHDSASHAKE192fTests;
    use test_structs::SLHDSASHAKE256sTests;

    generate_all_tests!(
        data_dir=&DATA_DIR;
        SLHDSASHAKE128fTests,
        SLHDSASHAKE192fTests,
        SLHDSASHAKE256sTests,
    );
}
