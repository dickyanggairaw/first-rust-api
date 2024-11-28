use redis::{Commands, Connection, RedisError, RedisResult};

pub struct RedisHelper {
  connection: Connection,
}

impl RedisHelper {
    pub fn new() -> RedisResult<Self> {
      let client = redis::Client::open("redis://127.0.0.1/")?;
        let connection = client.get_connection()?;
        Ok(Self { connection })
    }
    pub fn set_value(&mut self, key: &str, value: &str) -> RedisResult<()> {
      self.connection.set(key, value)
  }
  pub fn get_value(&mut self, key: &str) -> RedisResult<Option<String>> {
      self.connection.get(key)
  }
  pub fn del_value(&mut self, key: &str) -> Result<(), RedisError> {
    let result: Result<_, RedisError> = self.connection.del(key);
    result
  }
}