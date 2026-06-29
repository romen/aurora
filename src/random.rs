use super::{named, ProviderInstance};
use rand::rand_core::CryptoRng;

// we re-export `rand` so consumers can bring into scope the needed traits from the appropriate
// `rand` crate version
pub(crate) use ::rand;

// This module is for convenience, so consumers can just `use crate::random::prelude::*` instead of
// discovering which traits from `rand` they need.
pub mod prelude {
    pub(crate) use super::rand::TryRng;
}

impl<'a> ProviderInstance<'a> {
    #[named]
    pub fn get_rng(&self) -> impl CryptoRng {
        trace!(target: log_target!(), "Called ");

        // TODO: we should likely build an rng around OpenSSL's rand
        // the core dispatch table has these:
        // https://github.com/openssl/openssl/blob/openssl-4.0.0/doc/internal/man3/ossl_rand_get_entropy.pod

        let rng = rand::rng();

        return rng;
    }
}
