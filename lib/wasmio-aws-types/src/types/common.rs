use derivative::Derivative;
use derive_builder::Builder;

/// Container for all (if there are any) keys between Prefix and the next
/// occurrence of the string specified by a delimiter. CommonPrefixes lists keys
/// that act like subdirectories in the directory specified by Prefix. For
/// example, if the prefix is notes/ and the delimiter is a slash (/) as in
/// notes/summer/july, the common prefix is notes/summer/.
#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct CommonPrefix {
    /// Container for the specified common prefix.
    pub prefix: Option<String>,
}
