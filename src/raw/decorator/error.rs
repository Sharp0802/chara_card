use std::num::ParseIntError;
use std::str::ParseBoolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("insufficient argument")]
    InsufficientArgument,

    #[error("extraneous argument")]
    ExtraneousArgument,

    #[error("invalid depth specified")]
    InvalidDepth,

    #[error("number expected: {0}")]
    ExpectedInt(#[from] ParseIntError),

    #[error("boolean expected: {0}")]
    ExpectedBool(#[from] ParseBoolError),

    #[error("role expected")]
    ExpectedRole,
}
