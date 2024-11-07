/*
 * Copyright (c) 2020 Jens Reimann and others.
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0
 *
 * SPDX-License-Identifier: EPL-2.0
 */

use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, JsonSchema, Serialize, Default, Deserialize, Debug, Clone, PartialEq)]
#[kube(
    group = "legacy.k8s.keycloak.org",
    version = "v1alpha1",
    kind = "KeycloakClient",
    namespaced,
    derive = "Default",
    derive = "PartialEq",
    status = "KeycloakClientStatus"
)]
//#[kube(apiextensions = "v1beta1")]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakClientSpec {
    pub client: Client,
    pub realm_selector: LabelSelector,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Client {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<BTreeMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authenticator_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_access_grants_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontchannel_logout: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_scope_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    pub implicit_flow_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_registration_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mappers: Option<Vec<ProtocolMapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_client: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_accounts_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_flow_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surrogate_auth_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template_config: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template_mappers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_template_scope: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_origins: Option<Vec<String>>,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ProtocolMapper {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mapper: Option<String>,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakClientStatus {
    pub phase: String,
    pub message: String,
    pub ready: bool,
    pub secondary_resources: Option<BTreeMap<String, Vec<String>>>,
}
