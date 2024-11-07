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

use crate::crd::{Client, User};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[kube(
    group = "legacy.k8s.keycloak.org",
    version = "v1alpha1",
    kind = "KeycloakRealm",
    namespaced,
    derive = "Default",
    derive = "PartialEq",
    status = "KeycloakRealmStatus"
)]
//#[kube(apiextensions = "v1beta1")]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakRealmSpec {
    pub instance_selector: LabelSelector,
    pub realm: Realm,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Realm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_listeners: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_providers: Option<Vec<IdentityProvider>>,
    pub realm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_overrides: Option<Vec<Override>>,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct IdentityProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_read_token_role_on_create: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_broker_login_flow_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_broker_login_flow_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    store_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_email: Option<bool>,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Override {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakRealmStatus {
    #[serde(rename = "loginURL")]
    pub login_url: String,
    pub message: String,
    pub phase: String,
    pub ready: bool,
    pub secondary_resources: Option<BTreeMap<String, Vec<String>>>,
}
