use super::dtls_fingerprint::*;
use super::dtls_role::*;

use serde::{Deserialize, Serialize};

/// DTLSParameters holds information relating to DTLS configuration.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DTLSParameters {
    pub role: DTLSRole,
    pub fingerprints: Vec<DTLSFingerprint>,
}
