//! Redis Tower Service

mod service;

pub use service::RedisService;

#[cfg(test)]
mod test {
    use crate::RedisService;
    use redis::{aio::ConnectionManager, Client, Cmd, Value};
    use tower::{Service, ServiceExt};

    const URL: &str = "redis://127.0.0.1:6379";

    macro_rules! call {
        ($service:expr, $cmd:expr) => {
            $service.ready().await.unwrap().call($cmd).await
        };
    }

    #[tokio::test]
    async fn test_redis_service() {
        let client = Client::open(URL).unwrap();
        let connection = ConnectionManager::new(client).await.unwrap();
        let service = RedisService::new(connection);
        let mut join_handles = Vec::new();

        for i in 0..100 {
            let mut service = service.clone();
            let key = format!("foo-{}", i);
            let value = format!("bar-{}", i);

            join_handles.push(tokio::spawn(async move {
                let res = call!(service, Cmd::set(key, value)).unwrap();

                assert_eq!(res, Value::Okay);
            }));
        }

        for i in 0..100 {
            let mut service = service.clone();
            let mut service_clone = service.clone();
            let key = format!("foo-{}", i);
            let key_clone = key.clone();
            let value = format!("bar-{}", i);

            tokio::spawn(async move {
                let res = call!(service, Cmd::get(key)).unwrap();
                assert_eq!(res, Value::Data(value.into()));

                let res = call!(service_clone, Cmd::del(key_clone)).unwrap();
                assert_eq!(res, Value::Int(1));
            });
        }
    }
}
