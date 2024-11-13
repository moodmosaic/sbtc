/*
 * emily-openapi-spec
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateWithdrawalsResponse : Response to update withdrawals request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWithdrawalsResponse {
    /// Updated withdrawals.
    #[serde(rename = "withdrawals")]
    pub withdrawals: Vec<models::Withdrawal>,
}

impl UpdateWithdrawalsResponse {
    /// Response to update withdrawals request.
    pub fn new(withdrawals: Vec<models::Withdrawal>) -> UpdateWithdrawalsResponse {
        UpdateWithdrawalsResponse { withdrawals }
    }
}