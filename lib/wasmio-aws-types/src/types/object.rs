use derivative::Derivative;
use derive_builder::Builder;

use super::Owner;

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
/// An object consists of data and its descriptive metadata.
pub struct Object {
    /// The entity tag is a hash of the object. The ETag reflects changes only
    /// to the contents of an object, not its metadata. The ETag may or may not
    /// be an MD5 digest of the object data. Whether or not it is depends on
    /// how the object was created and how it is encrypted as described below:
    /// - Objects created by the PUT Object, POST Object, or Copy
    /// operation, or through the AWS Management Console, and are encrypted by
    /// SSE-S3 or plaintext, have ETags that are an MD5 digest of their object
    /// data.
    /// - Objects created by the PUT Object, POST Object, or
    /// Copy operation, or through the AWS Management Console, and are
    /// encrypted by SSE-C or SSE-KMS, have ETags that are not an MD5 digest of
    /// their object data.
    /// - If an object is created by either the
    /// Multipart Upload or Part Copy operation, the ETag is not an MD5 digest,
    /// regardless of the method of encryption.
    pub e_tag: Option<String>,
    /// The name that you assign to an object. You use the object key to
    /// retrieve the object.
    pub key: Option<String>,
    /// Creation date of the object.
    pub last_modified: Option<String>,
    /// The owner of the object
    pub owner: Option<Owner>,
    /// Size in bytes of the object
    pub size: Option<i64>,
    /// The class of storage used to store the object.
    pub storage_class: Option<String>,
}
