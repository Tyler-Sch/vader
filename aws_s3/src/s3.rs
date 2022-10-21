use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::types::AggregatedBytes;
use aws_sdk_s3::Client;
use std::io::{Bytes, Cursor};
use tokio::runtime::Builder;

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
    // there is a limitation to the number of keys returned atm
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
    println!("{:?}", files);
    // this can probably be optimized with tokio-streams
    // let matches: Vec<String> = files.into_iter().filter(|name| name.contains(key)).collect();
    // println!("{:?}",matches);
    let mut data = vec![];
    for k in files {
        let f = get_file(&client, s3_data.bucket, &k).await?;
        data.push(f);
    }
    Ok(data)
}

pub fn run(
    bucket: &str,
    key: &str,
    suffix: &str,
) -> Result<Vec<AggregatedBytes>, aws_sdk_s3::Error> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let fetch_data = S3FetchData {
        bucket,
        key,
        suffix,
    };

    // let z = runtime.block_on(get_client_and_multiple_files("tylersdata", "stock/stock/historicalStockKaggle/small_guy/", "csv"));
    runtime.block_on(get_client_and_multiple_files(fetch_data))
}

#[cfg(test)]
mod s3_test {
    use super::*;
    #[test]
    fn works() {
    }
    #[test]
    fn check_download() {
        let a = vec![
            "hello".to_string(),
            "yellow".to_string(),
            "yolo".to_string(),
        ]
        .into_iter();
        let z = String::from("hello").contains("lo");
        let b: Vec<String> = a.filter(|x| x.contains("llo")).collect();
        println!("{:?}", b);
    }
}
