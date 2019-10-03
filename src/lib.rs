// lib.rs

// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A crate allowing for repeated retry of a function until it either
//! reports success, fails with an error, or times out.

use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;


/// Wait until an operation yields a result or a deadline is reached,
/// polling every `interval`.
/// On success the function returns `Ok(Some(T))`. When `deadline` is
/// reached without a result being produced the return value is
/// `Ok(None)`. Errors reported by the operation in question result in
/// early return with `Err(E)`.
pub fn wait_until<F, T, E>(deadline: Instant, interval: Duration, mut op: F) -> Result<Option<T>, E>
where
  F: FnMut() -> Result<Option<T>, E>,
{
  'l: loop {
    match op() {
      // No value means we just repeat.
      Ok(None) => sleep(interval),
      // We retrieved our value and are done.
      v @ Ok(Some(_)) => break 'l v,
      // Errors terminate our wait early.
      e @ Err(_) => break 'l e,
    }

    if Instant::now() >= deadline {
      break 'l Ok(None);
    }
  }
}

/// Wait until an operation yields a result or a timeout is reached,
/// polling every `interval`.
/// On success the function returns `Ok(Some(T))`. When `timeout` is
/// exceeded without a result being produced the return value is
/// `Ok(None)`. Errors reported by the operation in question result in
/// early return with `Err(E)`.
pub fn wait_for<F, T, E>(timeout: Duration, interval: Duration, op: F) -> Result<Option<T>, E>
where
  F: FnMut() -> Result<Option<T>, E>,
{
  wait_until(Instant::now() + timeout, interval, op)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wait_for_success() {
    let result = wait_for::<_, _, ()>(Duration::from_secs(5), Duration::from_nanos(1), || {
      static mut COUNTER: u64 = 1;
      if unsafe { COUNTER } == 5 {
        Ok(Some(5))
      } else {
        unsafe { COUNTER += 1 };
        Ok(None)
      }
    });

    assert_eq!(result, Ok(Some(5)))
  }

  #[test]
  fn wait_for_error() {
    let result = wait_for::<_, (), _>(Duration::from_secs(5), Duration::from_nanos(1), || {
      static mut COUNTER: u64 = 1;
      if unsafe { COUNTER } == 5 {
        Err("expected")
      } else {
        unsafe { COUNTER += 1 };
        Ok(None)
      }
    });

    assert_eq!(result, Err("expected"))
  }

  #[test]
  fn wait_for_timeout() {
    let result = wait_for::<_, (), ()>(Duration::from_millis(10), Duration::from_nanos(1), || {
      Ok(None)
    });

    assert_eq!(result, Ok(None))
  }
}
