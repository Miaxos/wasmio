use std::collections::HashMap;

use axum::body::Body;
use derivative::Derivative;
use derive_builder::Builder;

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct GetObjectRequest {
    /// The bucket name containing the object.  When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form *AccessPointName*-*AccountId*.s3-accesspoint.*Region*.amazonaws.com. When using this action with an access point through the AWS SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the *Amazon S3 User Guide*. When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form *AccessPointName*-*AccountId*.*outpostID*.s3-outposts.*Region*.amazonaws.com. When using this action using S3 on Outposts through the AWS SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using S3 on Outposts</a> in the *Amazon S3 User Guide*.
    pub bucket: String,
    /// The account ID of the expected bucket owner. If the bucket is owned by
    /// a different account, the request will fail with an HTTP <code>403
    /// (Access Denied)</code> error.
    pub expected_bucket_owner: Option<String>,
    /// Return the object only if its entity tag (ETag) is the same as the one
    /// specified, otherwise return a 412 (precondition failed).
    pub if_match: Option<String>,
    /// Return the object only if it has been modified since the specified
    /// time, otherwise return a 304 (not modified).
    pub if_modified_since: Option<String>,
    /// Return the object only if its entity tag (ETag) is different from the
    /// one specified, otherwise return a 304 (not modified).
    pub if_none_match: Option<String>,
    /// Return the object only if it has not been modified since the specified
    /// time, otherwise return a 412 (precondition failed).
    pub if_unmodified_since: Option<String>,
    /// Key of the object to get.
    pub key: String,
    /// Part number of the object being read. This is a positive integer
    /// between 1 and 10,000. Effectively performs a 'ranged' GET request for
    /// the part specified. Useful for downloading just a part of an object.
    pub part_number: Option<i64>,
    /// Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>. <note> Amazon S3 doesn&#39;t support retrieving multiple ranges of data per <code>GET</code> request. </note>
    pub range: Option<String>,
    pub request_payer: Option<String>,
    /// Sets the <code>Cache-Control</code> header of the response.
    pub response_cache_control: Option<String>,
    /// Sets the <code>Content-Disposition</code> header of the response
    pub response_content_disposition: Option<String>,
    /// Sets the <code>Content-Encoding</code> header of the response.
    pub response_content_encoding: Option<String>,
    /// Sets the <code>Content-Language</code> header of the response.
    pub response_content_language: Option<String>,
    /// Sets the <code>Content-Type</code> header of the response.
    pub response_content_type: Option<String>,
    /// Sets the <code>Expires</code> header of the response.
    pub response_expires: Option<String>,
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
    /// VersionId used to reference a specific version of the object.
    pub version_id: Option<String>,
}

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct GetObjectOutput {
    /// Indicates that a range of bytes was specified.
    pub accept_ranges: Option<String>,
    /// Object data.
    #[derivative(Debug = "ignore")]
    pub body: Option<Body>,
    /// Indicates whether the object uses an S3 Bucket Key for server-side
    /// encryption with AWS KMS (SSE-KMS).
    pub bucket_key_enabled: Option<bool>,
    /// Specifies caching behavior along the request/reply chain.
    pub cache_control: Option<String>,
    /// Specifies presentational information for the object.
    pub content_disposition: Option<String>,
    /// Specifies what content encodings have been applied to the object and
    /// thus what decoding mechanisms must be applied to obtain the media-type
    /// referenced by the Content-Type header field.
    pub content_encoding: Option<String>,
    /// The language the content is in.
    pub content_language: Option<String>,
    /// Size of the body in bytes.
    pub content_length: Option<i64>,
    /// The portion of the object returned in the response.
    pub content_range: Option<String>,
    /// A standard MIME type describing the format of the object data.
    pub content_type: Option<String>,
    /// Specifies whether the object retrieved was (true) or was not (false) a
    /// Delete Marker. If false, this response header does not appear in the
    /// response.
    pub delete_marker: Option<bool>,
    /// An ETag is an opaque identifier assigned by a web server to a specific
    /// version of a resource found at a URL.
    pub e_tag: Option<String>,
    /// If the object expiration is configured (see PUT Bucket lifecycle), the
    /// response includes this header. It includes the expiry-date and rule-id
    /// key-value pairs providing object expiration information. The value of
    /// the rule-id is URL encoded.
    pub expiration: Option<String>,
    /// The date and time at which the object is no longer cacheable.
    pub expires: Option<String>,
    /// Creation date of the object.
    pub last_modified: Option<String>,
    /// A map of metadata to store with the object in S3.
    pub metadata: Option<HashMap<String, String>>,
    /// This is set to the number of metadata entries not returned in
    /// <code>x-amz-meta</code> headers. This can happen if you create metadata
    /// using an API like SOAP that supports more flexible metadata than the
    /// REST API. For example, using SOAP, you can create metadata whose values
    /// are not legal HTTP headers.
    pub missing_meta: Option<i64>,
    /// Indicates whether this object has an active legal hold. This field is
    /// only returned if you have permission to view an object's legal hold
    /// status.
    pub object_lock_legal_hold_status: Option<String>,
    /// The Object Lock mode currently in place for this object.
    pub object_lock_mode: Option<String>,
    /// The date and time when this object's Object Lock will expire.
    pub object_lock_retain_until_date: Option<String>,
    /// The count of parts this object has.
    pub parts_count: Option<i64>,
    /// Amazon S3 can return this if your request involves a bucket that is
    /// either a source or destination in a replication rule.
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    /// Provides information about object restoration action and expiration
    /// time of the restored object copy.
    pub restore: Option<String>,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the
    /// encryption algorithm used.
    pub sse_customer_algorithm: Option<String>,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round-trip
    /// message integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: Option<String>,
    /// If present, specifies the ID of the AWS Key Management Service (AWS
    /// KMS) symmetric customer managed customer master key (CMK) that was used
    /// for the object.
    pub ssekms_key_id: Option<String>,
    /// The server-side encryption algorithm used when storing this object in
    /// Amazon S3 (for example, AES256, aws:kms).
    pub server_side_encryption: Option<String>,
    /// Provides storage class information of the object. Amazon S3 returns
    /// this header for all objects except for S3 Standard storage class
    /// objects.
    pub storage_class: Option<String>,
    /// The number of tags, if any, on the object.
    pub tag_count: Option<i64>,
    /// Version of the object.
    pub version_id: Option<String>,
    /// If the bucket is configured as a website, redirects requests for this
    /// object to another object in the same bucket or to an external URL.
    /// Amazon S3 stores the value of this header in the object metadata.
    pub website_redirect_location: Option<String>,
}
