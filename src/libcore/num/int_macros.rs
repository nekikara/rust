// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(hidden)]

macro_rules! int_module { ($T:ident) => (

/// The smallest value that can be represented by this integer type.
#[stable(feature = "rust1", since = "1.0.0")]
pub const MIN: $T = $T::min_value();
/// The largest value that can be represented by this integer type.
#[stable(feature = "rust1", since = "1.0.0")]
pub const MAX: $T = $T::max_value();

) }
