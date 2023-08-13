// Copyright (c) 2019 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use futures_util::future::{ok, FutureExt};
use pin_project::pin_project;
use tokio::time::{self, Interval};

use std::{
    future::Future,
    sync::{atomic::Ordering, Arc},
};

use super::Inner;
use crate::PoolOpts;
use futures_core::task::{Context, Poll};
use std::pin::Pin;

/// Idling connections TTL check interval.
///
/// The purpose of this interval is to remove idling connections that both:
/// * overflows min bound of the pool;
/// * idles longer then `inactive_connection_ttl`.
#[pin_project]
pub(crate) struct TtlCheckInterval {
    inner: Arc<Inner>,
    #[pin]
    interval: Interval,
    pool_opts: PoolOpts,
}

impl TtlCheckInterval {
    /// Creates new `TtlCheckInterval`.
    pub fn new(pool_opts: PoolOpts, inner: Arc<Inner>) -> Self {
        let interval = time::interval(pool_opts.ttl_check_interval());
        Self {
            inner,
            interval,
            pool_opts,
        }
    }

    /// Perform the check.
    pub fn check_ttl(&self) {
        let mut exchange = self.inner.exchange.lock().unwrap();

        let num_idling = exchange.available.len();
        let num_to_drop = num_idling.saturating_sub(self.pool_opts.constraints().min());

        for _ in 0..num_to_drop {
            let idling_conn = exchange.available.pop_front().unwrap();
            if idling_conn.elapsed() > self.pool_opts.inactive_connection_ttl() {
                assert!(idling_conn.conn.inner.pool.is_none());
                let inner = self.inner.clone();
                tokio::spawn(idling_conn.conn.disconnect().then(move |_| {
                    let mut exchange = inner.exchange.lock().unwrap();
                    exchange.exist -= 1;
                    ok::<_, ()>(())
                }));
            } else {
                exchange.available.push_back(idling_conn);
            }
        }
    }
}

impl Future for TtlCheckInterval {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let _ = futures_core::ready!(self.as_mut().project().interval.poll_tick(cx));
            let close = self.inner.close.load(Ordering::Acquire);

            if !close {
                self.check_ttl();
            } else {
                return Poll::Ready(());
            }
        }
    }
}
