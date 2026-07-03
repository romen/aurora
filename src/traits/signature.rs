#![allow(dead_code)]

use std::error::Error as ErrorTrait;
use std::fmt::Debug;

pub use ::signature::{Error, Keypair, SignatureEncoding, Signer, Verifier};

#[derive(Debug)]
pub enum VerificationError {
    InvalidSignature,
    GenericVerificationError,
}

impl core::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            VerificationError::InvalidSignature => write!(f, "error: verification failed"),
            VerificationError::GenericVerificationError => {
                write!(f, "error: generic internal failure")
            }
        }
    }
}

impl ErrorTrait for VerificationError {}

impl From<Error> for VerificationError {
    fn from(value: Error) -> Self {
        value
            .source()
            .map_or(VerificationError::GenericVerificationError, |e| {
                if let Some(ver_err) = e.downcast_ref::<VerificationError>() {
                    match ver_err {
                        VerificationError::InvalidSignature => VerificationError::InvalidSignature,
                        VerificationError::GenericVerificationError => {
                            VerificationError::GenericVerificationError
                        }
                    }
                } else {
                    VerificationError::GenericVerificationError
                }
            })
    }
}
