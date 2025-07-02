// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RtlsdrError {
    #[error("RtlSdr error {0}")]
    RtlsdrErr(String),
    #[error("USB error")]
    Usb(#[from] rusb::Error),
}

/// A result of a function that may return a `Error`.
pub type Result<T> = std::result::Result<T, RtlsdrError>;
