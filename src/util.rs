use crate::RedisService;
use redis::{Cmd, RedisResult, ToRedisArgs, Value};
use tower::{Service, ServiceExt};

impl RedisService {
    pub async fn call_service(&self, request: Cmd) -> RedisResult<Value> {
        let mut service = self.clone();
        let service = service.ready().await?;
        let response = service.call(request).await?;

        Ok(response)
    }

    pub async fn del<K>(&self, key: K) -> RedisResult<Value>
    where
        K: ToRedisArgs,
    {
        let request = Cmd::del(key);

        self.call_service(request).await
    }

    pub async fn get<K>(&self, key: K) -> RedisResult<Value>
    where
        K: ToRedisArgs,
    {
        let request = Cmd::get(key);

        self.call_service(request).await
    }

    pub async fn set<K, V>(&self, key: K, value: V) -> RedisResult<Value>
    where
        K: ToRedisArgs,
        V: ToRedisArgs,
    {
        let request = Cmd::set(key, value);

        self.call_service(request).await
    }

    pub async fn set_ex<K, V>(&self, key: K, value: V, seconds: u64) -> RedisResult<Value>
    where
        K: ToRedisArgs,
        V: ToRedisArgs,
    {
        let request = Cmd::set_ex(key, value, seconds);

        self.call_service(request).await
    }
}

#[cfg(test)]
mod test {
    use crate::RedisService;
    use redis::{aio::ConnectionManager, Client, Value};

    const URL: &str = "redis://127.0.0.1:6379";

    #[tokio::test]
    async fn test_redis_util() {
        let client = Client::open(URL).unwrap();
        let connection = ConnectionManager::new(client).await.unwrap();
        let service = RedisService::new(connection);
        let mut join_handles = Vec::new();

        for i in 0..100 {
            let service = service.clone();
            let key = format!("foo-{}", i);
            let value = format!("bar-{}", i);

            join_handles.push(tokio::spawn(async move {
                let res = service.set(key, value).await.unwrap();

                assert_eq!(res, Value::Okay);
            }));
        }

        for i in 0..100 {
            let service = service.clone();
            let key = format!("foo-{}", i);
            let key_clone = key.clone();
            let value = format!("bar-{}", i);

            tokio::spawn(async move {
                let res = service.get(key).await.unwrap();
                assert_eq!(res, Value::Data(value.into()));

                let res = service.del(key_clone).await.unwrap();
                assert_eq!(res, Value::Int(1));
            });
        }
    }
}
