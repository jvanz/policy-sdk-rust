use crate::host_capabilities::verification::{KeylessInfo, KeylessPrefixInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod net;
pub mod oci;
pub mod verification;

#[derive(Serialize, Deserialize, Debug)]
pub enum SigstoreVerificationInputV1 {
    /// Require the verification of the manifest digest of an OCI object (be
    /// it an image or anything else that can be stored into an OCI registry)
    /// to be signed by Sigstore, using public keys mode
    SigstorePubKeyVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of PEM encoded keys that must have been used to sign the OCI object
        pub_keys: Vec<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore, using keyless mode
    SigstoreKeylessVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless: Vec<KeylessInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },
}

impl Into<CallbackRequestType> for SigstoreVerificationInputV1{
    fn into(self) -> CallbackRequestType {
        match self {
            SigstoreVerificationInputV1::SigstorePubKeyVerify { image, pub_keys, annotations} => {CallbackRequestType::SigstorePubKeyVerify {
                image,
                pub_keys,
                annotations
            }},
            SigstoreVerificationInputV1::SigstoreKeylessVerify { image, keyless, annotations } => {
                CallbackRequestType::SigstoreKeylessVerify {image, keyless, annotations}
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SigstoreVerificationInputV2 {
    /// Require the verification of the manifest digest of an OCI object (be
    /// it an image or anything else that can be stored into an OCI registry)
    /// to be signed by Sigstore, using public keys mode
    SigstorePubKeyVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of PEM encoded keys that must have been used to sign the OCI object
        pub_keys: Vec<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore, using keyless mode
    SigstoreKeylessVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless: Vec<KeylessInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore using keyless mode, where the passed subject is a URL
    // prefix of the subject to match
    SigstoreKeylessPrefixVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless_prefix: Vec<KeylessPrefixInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore using keyless mode and performed in GitHub Actions
    SigstoreGithubActionsVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// owner of the repository. E.g: octocat
        owner: String,
        /// Optional - Repo of the GH Action workflow that signed the artifact. E.g: example-repo
        repo: Option<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    }
}

impl From<SigstoreVerificationInputV2> for CallbackRequestType{
    fn from(val: SigstoreVerificationInputV2) -> Self {
        match val {
            SigstoreVerificationInputV2::SigstorePubKeyVerify { image, pub_keys, annotations } =>
                CallbackRequestType::SigstorePubKeyVerify {image, pub_keys, annotations},
            SigstoreVerificationInputV2::SigstoreKeylessVerify { image, keyless, annotations } =>
                CallbackRequestType::SigstoreKeylessVerify {image, keyless, annotations},
            SigstoreVerificationInputV2::SigstoreKeylessPrefixVerify { image, keyless_prefix, annotations } =>
                CallbackRequestType::SigstoreKeylessPrefixVerify {image, keyless_prefix, annotations},
            SigstoreVerificationInputV2::SigstoreGithubActionsVerify { image, owner, repo, annotations } =>
                CallbackRequestType::SigstoreGithubActionsVerify {image, owner, repo, annotations},
        }
    }
}

/// Describes the different kinds of request a waPC guest can make to
/// our host.
#[derive(Serialize, Deserialize, Debug)]
pub enum CallbackRequestType {
    /// Require the computation of the manifest digest of an OCI object (be
    /// it an image or anything else that can be stored into an OCI registry)
    OciManifestDigest {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
    },

    /// Require the verification of the manifest digest of an OCI object (be
    /// it an image or anything else that can be stored into an OCI registry)
    /// to be signed by Sigstore, using public keys mode
    SigstorePubKeyVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of PEM encoded keys that must have been used to sign the OCI object
        pub_keys: Vec<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore, using keyless mode
    SigstoreKeylessVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless: Vec<KeylessInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore using keyless mode, where the passed subject is a URL
    // prefix of the subject to match
    SigstoreKeylessPrefixVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless_prefix: Vec<KeylessPrefixInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    // Require the verification of the manifest digest of an OCI object to be
    // signed by Sigstore using keyless mode and performed in GitHub Actions
    SigstoreGithubActionsVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// owner of the repository. E.g: octocat
        owner: String,
        /// Optional - Repo of the GH Action workflow that signed the artifact. E.g: example-repo
        repo: Option<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    /// Lookup the addresses for a given hostname via DNS
    DNSLookupHost { host: String },
}
