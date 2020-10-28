use rusoto_core::Region;
use rusoto_s3::HeadObjectRequest;
use rusoto_s3::{S3Client, S3};

#[tokio::main]
async fn main() {
    let region = Region::default();
    let client = S3Client::new(region);
    let head_request = HeadObjectRequest {
        bucket: String::from("s3://s3rename-test-bucket"),
        if_match: None,
        if_modified_since: None,
        if_none_match: None,
        if_unmodified_since: None,
        key: String::from("key"),
        part_number: None,
        range: None,
        request_payer: None,
        sse_customer_algorithm: None,
        sse_customer_key: None,
        sse_customer_key_md5: None,
        version_id: None,
    };
    let head_result = client.head_object(head_request).await;
    println!("{:?}", head_result);
    //Ok(())
}
