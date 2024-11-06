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

use keycloak_crd::KeycloakRealm;
use kube::Resource;

#[test]
fn verify_resource() {
    assert_eq!(KeycloakRealm::kind(&()), "KeycloakRealm");
    assert_eq!(KeycloakRealm::group(&()), "keycloak.org");
    assert_eq!(KeycloakRealm::version(&()), "v1alpha1");
    assert_eq!(KeycloakRealm::api_version(&()), "keycloak.org/v1alpha1");
}

#[test]
fn accepts_content() {
    let resource: KeycloakRealm =
        serde_yaml::from_str(include_str!("data/keycloak_realm_1.yaml")).unwrap();

    let status = resource.status.as_ref().unwrap();
    assert_eq!(&status.phase, "reconciling");
    assert_eq!(&status.message, "");
}
