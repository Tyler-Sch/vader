use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::types::AggregatedBytes;
use aws_sdk_s3::Client;
use std::io::{Bytes, Cursor};
use tokio::runtime::Builder;
use futures::{stream::FuturesUnordered, StreamExt};

struct S3FetchData<'a> {
    bucket: &'a str,
    suffix: &'a str,
    key: &'a str,
}

async fn get_client() -> Result<Client, aws_sdk_s3::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

async fn list_files(
    client: &Client,
    s3_data: &S3FetchData<'_>,
) -> Result<Vec<String>, aws_sdk_s3::Error> {
    let resp = client
        .list_objects_v2()
        .bucket(s3_data.bucket)
        .prefix(s3_data.key)
        .send()
        .await?;
    let thing: Vec<String> = resp
        .contents()
        .unwrap_or_default()
        .iter()
        .map(|x| x.key().unwrap().into())
        .filter(|x: &String| x.ends_with(s3_data.suffix))
        .collect();
    Ok(thing)
}

async fn get_file(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<AggregatedBytes, aws_sdk_s3::Error> {
    let data = client.get_object().bucket(bucket).key(key).send().await?;
    let z = data.body.collect().await.unwrap(); // needs better handling of result?
    Ok(z)
}

async fn get_client_and_single_file(
    bucket_name: &str,
    key: &str,
) -> Result<AggregatedBytes, aws_sdk_s3::Error> {
    let client = get_client().await?;
    get_file(&client, bucket_name, key).await
}

async fn get_client_and_multiple_files(
    s3_data: S3FetchData<'_>,
) -> Result<Vec<AggregatedBytes>, aws_sdk_s3::Error> {
    let client = get_client().await?;
    let files = list_files(&client, &s3_data).await?;
    let mut data = FuturesUnordered::new();
    for k in &files {
        data.push(get_file(&client, s3_data.bucket, k));
    }
    let mut result = vec![];
    while let Some(f) = data.next().await {
       result.push(f.unwrap());
    }

    Ok(result)
}

pub fn get_s3_data(
    bucket: &str,
    key: &str,
    suffix: &str,
) -> Result<Vec<AggregatedBytes>, aws_sdk_s3::Error> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(3)
        .enable_all()
        .build()
        .unwrap();

    let fetch_data = S3FetchData {
        bucket,
        key,
        suffix,
    };

    runtime.block_on(get_client_and_multiple_files(fetch_data))
}

#[cfg(test)]
mod s3_test {
    }
