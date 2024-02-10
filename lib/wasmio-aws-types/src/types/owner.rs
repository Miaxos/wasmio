use axum::body::BodyDataStream;
use derivative::Derivative;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Container for the owner's display name and ID.
#[derive(Derivative, Default, Builder, Serialize, Deserialize)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    /// Container for the display name of the owner.
    pub display_name: Option<String>,
    /// Container for the ID of the owner.
    pub id: Option<String>,
}
