//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Extra metadata about the error, may be empty. Usually depends on the error type.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Details {}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Error {
    /**
     * Extra metadata about the error, may be empty. Usually depends on the error type.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct TaskResponse {
    /**
     * ID of the job started.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Role {
    #[serde(rename = "BUSINESS_ADMIN")]
    BusinessAdmin,
    #[serde(rename = "BUSINESS_BOOKKEEPER")]
    BusinessBookkeeper,
    #[serde(rename = "BUSINESS_OWNER")]
    BusinessOwner,
    #[serde(rename = "BUSINESS_USER")]
    BusinessUser,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Role::BusinessAdmin => "BUSINESS_ADMIN",
            Role::BusinessBookkeeper => "BUSINESS_BOOKKEEPER",
            Role::BusinessOwner => "BUSINESS_OWNER",
            Role::BusinessUser => "BUSINESS_USER",
            Role::Noop => "",
            Role::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Role {
    fn default() -> Role {
        Role::Noop
    }
}
impl Role {
    pub fn is_noop(&self) -> bool {
        matches!(self, Role::Noop)
    }
}

/// Ramp User
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct User {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub amount_limit: String,
    /**
     * Ramp User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business_id: String,
    /**
     * Ramp User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * Ramp User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location_id: String,
    /**
     * Ramp User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub manager_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    #[serde(default, skip_serializing_if = "Role::is_noop")]
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PatchUsersRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub direct_manager_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Page {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CardHolder {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct AccountingCategories {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Type {
    #[serde(rename = "POLICY_VIOLATION_FROM_ADMIN")]
    PolicyViolationFromAdmin,
    #[serde(rename = "POLICY_VIOLATION_FROM_USER")]
    PolicyViolationFromUser,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Type::PolicyViolationFromAdmin => "POLICY_VIOLATION_FROM_ADMIN",
            Type::PolicyViolationFromUser => "POLICY_VIOLATION_FROM_USER",
            Type::Noop => "",
            Type::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}
impl Type {
    pub fn is_noop(&self) -> bool {
        matches!(self, Type::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PolicyViolations {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Free form text regarding the policy violation.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub memo: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum GetTransactionResponseDataDisputesType {
    #[serde(rename = "DISPUTE_CANCELLED")]
    DisputeCancelled,
    #[serde(rename = "MERCHANT_ERROR")]
    MerchantError,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UNRECOGNIZED_CHARGE")]
    UnrecognizedCharge,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for GetTransactionResponseDataDisputesType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetTransactionResponseDataDisputesType::DisputeCancelled => "DISPUTE_CANCELLED",
            GetTransactionResponseDataDisputesType::MerchantError => "MERCHANT_ERROR",
            GetTransactionResponseDataDisputesType::Unknown => "UNKNOWN",
            GetTransactionResponseDataDisputesType::UnrecognizedCharge => "UNRECOGNIZED_CHARGE",
            GetTransactionResponseDataDisputesType::Noop => "",
            GetTransactionResponseDataDisputesType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for GetTransactionResponseDataDisputesType {
    fn default() -> GetTransactionResponseDataDisputesType {
        GetTransactionResponseDataDisputesType::Noop
    }
}
impl GetTransactionResponseDataDisputesType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetTransactionResponseDataDisputesType::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Disputes {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Free form text provided by the dispute initiator.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub memo: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GetTransactionResponseDataDisputesType>,
}

/// Ramp transaction
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Data {
    /**
     * Ramp transaction
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounting_categories: Vec<AccountingCategories>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    #[serde()]
    pub card_holder: CardHolder,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub card_id: String,
    /**
     * Ramp transaction
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disputes: Vec<Disputes>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Ramp transaction
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub memo: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merchant_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merchant_name: String,
    /**
     * Ramp transaction
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_violations: Vec<PolicyViolations>,
    /**
     * Ramp transaction
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receipts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub sk_category_id: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sk_category_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub user_transaction_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetTransactionResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Data>,
    #[serde(default)]
    pub page: Page,
}

/// Ramp location
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Location {
    /**
     * Ramp location
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Ramp location
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetLocationResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Location>,
    #[serde(default)]
    pub page: Page,
}

///
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostLocationRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

///
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetUsersResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<User>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PatchLocationRequest {
    /**
     * New name of location
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Ramp Department
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Department {
    /**
     * Ramp Department
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Ramp Department
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetDepartmentsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Department>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PatchDepartmentRequest {
    /**
     * New department name
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct RecipientAddress {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Shipping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address: Option<RecipientAddress>,
}

/// Details for shipping physical cards
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Fulfillment {
    /**
     * Details for shipping physical cards
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
}

/**
 * Time interval to apply limit to.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Interval {
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "TOTAL")]
    Total,
    #[serde(rename = "YEARLY")]
    Yearly,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Interval::Daily => "DAILY",
            Interval::Monthly => "MONTHLY",
            Interval::Total => "TOTAL",
            Interval::Yearly => "YEARLY",
            Interval::Noop => "",
            Interval::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Interval {
    fn default() -> Interval {
        Interval::Noop
    }
}
impl Interval {
    pub fn is_noop(&self) -> bool {
        matches!(self, Interval::Noop)
    }
}

/// Specifies the spend restrictions on a Ramp card.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct SpendingRestrictions {
    /**
     * Amount limit total per interval.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_categories: Vec<f64>,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<f64>,
    /**
     * Time interval to apply limit to.
     */
    #[serde(default, skip_serializing_if = "Interval::is_noop")]
    pub interval: Interval,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub suspended: bool,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub transaction_amount_limit: f64,
}

/// Card data that holds mostly static information about a card.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Card {
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub card_program_id: String,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cardholder_id: String,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cardholder_name: String,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<Fulfillment>,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_physical: bool,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_four: String,
    /**
     * Card data that holds mostly static information about a card.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<SpendingRestrictions>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetCardsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cards: Vec<Card>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PatchResourcesCardsCardRequest {
    /**
     * Set to link card with a card program, or set to null to detach a card from a card program. If the card is already linked with a card program, it will detach from original card program before linking with the new one.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub card_program_id: String,
    /**
     * Cosmetic display name of the card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * Flag to set to enable or disable notifications.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_notifications_enabled: Option<bool>,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<SpendingRestrictions>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetCustomProviderResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_id_provider: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostcustomProviderResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub provider_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BillingAddress {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/// Mostly static information about a business that doesn't change often.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Business {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde()]
    pub billing_address: BillingAddress,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub business_memo_required_threshold: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business_name_legal: String,
    /**
     * Mostly static information about a business that doesn't change often.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business_name_on_card: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub business_receipt_required_threshold: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_time: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enforce_sso: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub initial_approved_limit: f64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_integrated_with_slack: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_reimbursements_enabled: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub limit_locked: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostResourcesCardPhysicalRequest {
    /**
     * Alternative method to create card using a card program. Card program's is_physical must be true. If this value is given, no other attributes (other than idempotency_key) may be given.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub card_program_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * Details for shipping physical cards
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<Fulfillment>,
    /**
     * Idempotency key
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub idempotency_key: String,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<SpendingRestrictions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostResourcesCardVirtualRequest {
    /**
     * Alternative method to create card using a card program. Card program's is_physical must be false. If this value is given, no other attributes (other than idempotency_key) may be given.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub card_program_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * Idempotency key
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub idempotency_key: String,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<SpendingRestrictions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum TokenType {
    #[serde(rename = "Bearer")]
    Bearer,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TokenType::Bearer => "Bearer",
            TokenType::Noop => "",
            TokenType::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType::Noop
    }
}
impl TokenType {
    pub fn is_noop(&self) -> bool {
        matches!(self, TokenType::Noop)
    }
}

///
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct OAuth2Token {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    /**
     * Expiration time for access token in seconds
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub expires_in: i64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub refresh_token_expires_in: i64,
    /**
     * Space-separated set of strings representing accessible resources
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope: String,
    #[serde(default, skip_serializing_if = "TokenType::is_noop")]
    pub token_type: TokenType,
}

/// Current data about the business.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BusinessCurrentStatus {
    /**
     * Current data about the business.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub balance_including_pending: f64,
    /**
     * Current data about the business.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub max_balance: f64,
    /**
     * Current data about the business.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<chrono::NaiveDate>,
    /**
     * Current data about the business.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev_billing_date: Option<chrono::NaiveDate>,
    /**
     * Current data about the business.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub statement_balance: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostResourcesCardsCardSuspensionRequest {
    /**
     * Idempotency key
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub idempotency_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetEntityTypeCustomRampResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ramp_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetEntityTypeRampCustomResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetResourcesCardsDeferredResponseData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub card_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub misc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum GetResourcesCardsDeferredResponseStatus {
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for GetResourcesCardsDeferredResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetResourcesCardsDeferredResponseStatus::Error => "ERROR",
            GetResourcesCardsDeferredResponseStatus::InProgress => "IN_PROGRESS",
            GetResourcesCardsDeferredResponseStatus::Started => "STARTED",
            GetResourcesCardsDeferredResponseStatus::Success => "SUCCESS",
            GetResourcesCardsDeferredResponseStatus::Noop => "",
            GetResourcesCardsDeferredResponseStatus::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for GetResourcesCardsDeferredResponseStatus {
    fn default() -> GetResourcesCardsDeferredResponseStatus {
        GetResourcesCardsDeferredResponseStatus::Noop
    }
}
impl GetResourcesCardsDeferredResponseStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetResourcesCardsDeferredResponseStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetResourcesCardsDeferredResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetResourcesCardsDeferredResponseData>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetResourcesCardsDeferredResponseStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Icon {
    #[serde(rename = "AdvertisingIcon")]
    AdvertisingIcon,
    #[serde(rename = "CardIcon")]
    CardIcon,
    #[serde(rename = "EducationStipendIcon")]
    EducationStipendIcon,
    #[serde(rename = "LunchOrderingIcon")]
    LunchOrderingIcon,
    #[serde(rename = "OnboardingIcon")]
    OnboardingIcon,
    #[serde(rename = "PerDiemCardIcon")]
    PerDiemCardIcon,
    #[serde(rename = "SaasSubscriptionIcon")]
    SaasSubscriptionIcon,
    #[serde(rename = "SoftwareTrialIcon")]
    SoftwareTrialIcon,
    #[serde(rename = "TravelExpensesIcon")]
    TravelExpensesIcon,
    #[serde(rename = "WellnessIcon")]
    WellnessIcon,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Icon::AdvertisingIcon => "AdvertisingIcon",
            Icon::CardIcon => "CardIcon",
            Icon::EducationStipendIcon => "EducationStipendIcon",
            Icon::LunchOrderingIcon => "LunchOrderingIcon",
            Icon::OnboardingIcon => "OnboardingIcon",
            Icon::PerDiemCardIcon => "PerDiemCardIcon",
            Icon::SaasSubscriptionIcon => "SaasSubscriptionIcon",
            Icon::SoftwareTrialIcon => "SoftwareTrialIcon",
            Icon::TravelExpensesIcon => "TravelExpensesIcon",
            Icon::WellnessIcon => "WellnessIcon",
            Icon::Noop => "",
            Icon::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for Icon {
    fn default() -> Icon {
        Icon::Noop
    }
}
impl Icon {
    pub fn is_noop(&self) -> bool {
        matches!(self, Icon::Noop)
    }
}

/// Card Program data that serves as a template for creating new cards.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CardProgram {
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_default: bool,
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_physical: bool,
    /**
     * Card Program data that serves as a template for creating new cards.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<SpendingRestrictions>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetCardProgramsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub card_programs: Vec<CardProgram>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostResourcesCardProgramRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Icon::is_noop")]
    pub icon: Icon,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_physical: Option<bool>,
    /**
     * Specifies the spend restrictions on a Ramp card.
     */
    #[serde()]
    pub spending_restrictions: SpendingRestrictions,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostUsersDeferredRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub direct_manager_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    #[serde(default, skip_serializing_if = "Role::is_noop")]
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetUsersDeferredStatusResponseData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub misc: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

///
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetUsersDeferredStatusResponse {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetUsersDeferredStatusResponseData>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

///
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetUsersDeferredStatusResponseDataType {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Receipt {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub receipt_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub transaction_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetReceiptsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Receipt>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Reimbursement {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merchant: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receipts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct GetReimbursementsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Reimbursement>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PostCustomProviderEntityTypeLinkRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ramp_id: String,
}