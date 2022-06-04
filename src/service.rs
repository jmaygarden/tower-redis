use redis::{
    aio::{ConnectionLike, ConnectionManager},
    Cmd, RedisError, RedisFuture, Value,
};
use std::task::{Context, Poll};

/// A Tower service for asynchronous Redis request/response performed over a
/// managed, multplexed connection.
#[derive(Clone)]
pub struct RedisService {
    /// Redis connection for queries
    inner: ConnectionManager,
}

impl RedisService {
    // Create a new service with the provided connection manager
    pub fn new(inner: ConnectionManager) -> Self {
        Self { inner }
    }
}

impl tower::Service<Cmd> for RedisService {
    type Response = Value;
    type Error = RedisError;
    type Future = RedisFuture<'static, Value>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Cmd) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move { inner.req_packed_command(&req).await })
    }
}
