//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use napi::bindgen_prelude::*;

pub type CurveError = Error;

pub type Result<T> = std::result::Result<T, CurveError>;