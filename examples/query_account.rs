use zvstsapi_rs::account::AccountQueryRequest;
use zvstsapi_rs::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let account_query_req = AccountQueryRequest {
        currency: Some("USD".to_string()),
    };
    let account_query_resp = client.account_query(account_query_req).await.unwrap();
    println!("{:#?}", account_query_resp);
}
