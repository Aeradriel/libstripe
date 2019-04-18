use crate::resources::common::address::Address;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::Client;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CardHolders {
    pub id: String,
    pub object: Object,
    pub billing: Billing,
    pub created: i64,
    pub email: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub phone_number: String,
    pub status: CardHolderStatus,
    pub cardholder_type: CardHolderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationControls {
    pub allowed_categories: Vec<String>,
    pub blocked_categories: Vec<String>,
    pub spending_limits: Vec<SpendingLimits>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpendingLimits {
    pub amount: Option<i64>,
    pub categories: Option<Vec<String>>,
    pub interval: Option<SpendingLimitsInterval>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SpendingLimitsInterval {
    PerAuthorization,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    AllTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Billing {
    pub address: Address,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CardHolderStatus {
    Active,
    Inactive,
    Pending,
    Blocked,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CardHolderType {
    Individual,
    BusinessEntity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardHolderParam<'a> {
    pub billing: Option<Billing>,
    pub name: Option<&'a str>,
    pub cardholder_type: Option<CardHolderType>,
    pub authorization_controls: Option<AuthorizationControls>,
    pub email: Option<&'a str>,
    pub is_default: Option<bool>,
    pub metadata: Option<HashMap<String, String>>,
    pub phone_number: Option<&'a str>,
    pub status: Option<CardHolderStatus>,
}

impl CardHolders {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::CardHolders, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::CardHolders, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::CardHolders, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::CardHolders, vec![], param)
    }
}
