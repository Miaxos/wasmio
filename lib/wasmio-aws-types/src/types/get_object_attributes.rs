use std::collections::HashMap;

use axum::body::{Body, BodyDataStream};
use derivative::Derivative;
use derive_builder::Builder;

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct GetObjectAttributesRequest {
    /// <p>Maximum number of parts that were allowed in the response.</p>
    pub max_parts: Option<i64>,
    /// <p>Part number of the object being read. This is a positive integer
    /// between 1 and 10,000. Effectively performs a 'ranged' HEAD request for
    /// the part specified. Useful querying about the size of the part and the
    /// number of parts in this object.</p>
    pub part_number: Option<i64>,
    /// Specifies the algorithm to use to when decrypting the object (for
    /// example, AES256).
    pub sse_customer_algorithm: Option<String>,
    /// Specifies the customer-provided encryption key for Amazon S3 used to
    /// encrypt the data. This value is used to decrypt the object when
    /// recovering it and must match the one used when storing the data. The
    /// key must be appropriate for use with the algorithm specified in the
    /// <code>x-amz-server-side-encryption-customer-algorithm</code> header.
    pub sse_customer_key: Option<String>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC
    /// 1321. Amazon S3 uses this header for a message integrity check to
    /// ensure that the encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<String>,
    pub request_payer: Option<String>,
    /// The account ID of the expected bucket owner. If the bucket is owned by
    /// a different account, the request will fail with an HTTP <code>403
    /// (Access Denied)</code> error.
    pub expected_bucket_owner: Option<String>,
    /// Specifies the fields at the root level that you want returned in the
    /// response. Fields that you do not specify are not returned.
    ///
    /// Valid Values: ETag | Checksum | ObjectParts | StorageClass | ObjectSize
    pub expected_attributes: Vec<String>,
}

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
#[serde(rename = "GetObjectAttributesOutput")]
#[serde(rename_all = "PascalCase")]
pub struct GetObjectAttributesOutput {
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
}
