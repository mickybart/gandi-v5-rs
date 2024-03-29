use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use gandi_v5_livedns_api::{records::UpsertRecord, Api, Endpoint};
use serde::Deserialize;

use crate::config::AppConfig;

#[derive(Debug, Deserialize)]
pub(crate) struct Record {
    fqdn: String,
    rrset_name: String,
    rrset_type: String,
    rrset_value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Extra {
    rrset_ttl: Option<u32>,
}

pub(crate) async fn gandi(
    headers: HeaderMap,
    Path(Record {
        fqdn,
        rrset_name,
        rrset_type,
        rrset_value,
    }): Path<Record>,
    Query(Extra { rrset_ttl }): Query<Extra>,
    State(config): State<Arc<AppConfig>>,
) -> (StatusCode, String) {
    let auth_header = match headers.get("authorization") {
        Some(auth_header) => match auth_header.to_str() {
            Ok(auth_header) => auth_header,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()),
        },
        None => {
            return (
                StatusCode::FORBIDDEN,
                "No authorization header provided".to_owned(),
            )
        }
    };

    let rrset_ttl = rrset_ttl.unwrap_or(config.default_rrset_ttl);

    let personal_access_token = match config.get_pat_if_authorized(
        &fqdn,
        &rrset_name,
        &rrset_type,
        rrset_ttl,
        &auth_header,
    ) {
        Ok(personal_access_token) => personal_access_token,
        Err(e) => return (StatusCode::FORBIDDEN, e),
    };

    let api = match Api::build(Endpoint::Prod, &personal_access_token) {
        Ok(api) => api,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.as_ref().to_string()),
    };

    let record = UpsertRecord {
        rrset_values: vec![rrset_value.clone()],
        rrset_ttl: Some(rrset_ttl),
    };

    match api
        .upsert_record_by_name_and_type(&fqdn, &rrset_name, &rrset_type, &record)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Record {}.{} type {} updated with ip {} !",
                rrset_name, fqdn, rrset_type, rrset_value
            ),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.as_ref().to_string()),
    }
}
