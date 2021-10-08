use redis::{
    aio::{ConnectionLike, ConnectionManager},
    Cmd, RedisError, Value,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A service that performs GET requests on a single owned connection
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

impl tower_service::Service<Cmd> for RedisService {
    type Response = Value;
    type Error = RedisError;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Cmd) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move { inner.req_packed_command(&req).await })
    }
}

pub type ResponseFuture = Pin<Box<dyn Future<Output = Result<Value, RedisError>> + Send>>;
