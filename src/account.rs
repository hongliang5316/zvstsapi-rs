use crate::Client;
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountQueryRequest {
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountQueryResponse {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub subtype: String,
    pub currency: String,
    pub currency_rate: i64,
    pub cash_available: i64,
}

impl Client {
    pub async fn account_query(&self, req: AccountQueryRequest) -> Result<AccountQueryResponse> {
        let url = format!("{}/account/query", crate::BASE_URL);
        let res = self.inner.post(&url).json(&req).send().await?;
        crate::check_status_code(res.status())?;
        Ok(res.json().await?)
    }
}
