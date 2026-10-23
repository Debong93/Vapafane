//! Hilfsprogramme für die Vapafane-Entwicklung.

// Kein unwrap/expect und kein unsafe – Fehler werden sauber behandelt.
#![deny(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod pyrandom;
pub mod texturen;
