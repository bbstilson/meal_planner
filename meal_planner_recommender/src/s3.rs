use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_s3::{
    operation::put_object::PutObjectOutput,
    primitives::{ByteStream, SdkBody},
    Client,
};

pub struct S3 {
    client: Client,
}

impl S3 {
    pub async fn init(config: &SdkConfig) -> Self {
        Self {
            client: Client::new(config),
        }
    }

    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<String> {
        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let data = response.body.collect().await?.into_bytes();

        Ok(String::from_utf8_lossy(&data).to_string())
    }

    pub async fn upload_object(
        &self,
        bucket: &str,
        key: &str,
        data: &str,
    ) -> Result<PutObjectOutput> {
        let body = ByteStream::new(SdkBody::from(data));
        let result = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;

        Ok(result)
    }
}
