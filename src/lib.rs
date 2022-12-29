// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2021 isis lovecruft
// Copyright (c) 2016-2019 Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

#![no_std]
#![cfg_attr(feature = "nightly", feature(test))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "simd_backend", feature(stdsimd))]
// Refuse to compile if documentation is missing.
#![deny(missing_docs)]

//! # curve25519-dalek
//!
//! **A pure-Rust implementation of group operations on Ristretto and Curve25519.**
//!
//! `curve25519-dalek` is a library providing group operations on the Edwards and
//! Montgomery forms of Curve25519, and on the prime-order Ristretto group.

//------------------------------------------------------------------------
// External dependencies:
//------------------------------------------------------------------------

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(all(feature = "nightly", feature = "packed_simd"))]
extern crate packed_simd;

extern crate byteorder;
pub extern crate digest;
extern crate rand_core;
extern crate zeroize;

#[cfg(any(feature = "fiat_u64_backend", feature = "fiat_u32_backend"))]
extern crate fiat_crypto;

// Used for traits related to constant-time code.
extern crate subtle;

#[cfg(all(test, feature = "serde"))]
extern crate bincode;
#[cfg(feature = "serde")]
extern crate serde;

// Internal macros. Must come first!
#[macro_use]
pub(crate) mod macros;

//------------------------------------------------------------------------
// curve25519-dalek public modules
//------------------------------------------------------------------------

// Scalar arithmetic mod l = 2^252 + ..., the order of the Ristretto group
pub mod scalar;

// Point operations on the Montgomery form of Curve25519
pub mod montgomery;

// Point operations on the Edwards form of Curve25519
pub mod edwards;

// Group operations on the Ristretto group
pub mod ristretto;

// Useful constants, like the Ed25519 basepoint
pub mod constants;

// External (and internal) traits.
pub mod traits;

// Finite field arithmetic mod p = 2^255 - 19
pub mod field;

// Arithmetic backends (using u32, u64, etc) live here
pub mod backend;

//------------------------------------------------------------------------
// curve25519-dalek internal modules
//------------------------------------------------------------------------

// Crate-local prelude (for alloc-dependent features like `Vec`)
pub(crate) mod prelude;

// Generic code for window lookups
pub(crate) mod window;
