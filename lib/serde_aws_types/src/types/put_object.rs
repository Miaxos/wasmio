use axum::body::BodyDataStream;
use std::collections::HashMap;

use derivative::Derivative;
use derive_builder::Builder;
use tokio::io::AsyncBufRead;

pub type StreamingBody = Box<dyn AsyncBufRead + Unpin + Send>;

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct PutObjectRequest {
    /// The canned ACL to apply to the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>. This action is not supported by Amazon S3 on Outposts.
    pub acl: Option<String>,
    /// Object data.
    #[derivative(Debug = "ignore")]
    pub body: Option<BodyDataStream>,
    /// The bucket name to which the PUT action was initiated.  When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the AWS SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>. When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com. When using this action using S3 on Outposts through the AWS SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.
    pub bucket: String,
    /// Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS. Specifying this header with a PUT action doesn’t affect bucket-level settings for S3 Bucket Key.
    pub bucket_key_enabled: Option<bool>,
    ///  Can be used to specify caching behavior along the request/reply chain. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.
    pub cache_control: Option<String>,
    /// Specifies presentational information for the object. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1">http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1</a>.
    pub content_disposition: Option<String>,
    /// Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11</a>.
    pub content_encoding: Option<String>,
    /// The language the content is in.
    pub content_language: Option<String>,
    /// Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13</a>.
    pub content_length: Option<i64>,
    /// The base64-encoded 128-bit MD5 digest of the message (without the headers) according to RFC 1864. This header can be used as a message integrity check to verify that the data is the same data that was originally sent. Although it is optional, we recommend using the Content-MD5 mechanism as an end-to-end integrity check. For more information about REST request authentication, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html">REST Authentication</a>.
    pub content_md5: Option<String>,
    /// A standard MIME type describing the format of the contents. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17</a>.
    pub content_type: Option<String>,
    /// The account ID of the expected bucket owner. If the bucket is owned by a different account, the request will fail with an HTTP <code>403 (Access Denied)</code> error.
    pub expected_bucket_owner: Option<String>,
    /// The date and time at which the object is no longer cacheable. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21</a>.
    pub expires: Option<String>,
    /// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object. This action is not supported by Amazon S3 on Outposts.
    pub grant_full_control: Option<String>,
    /// Allows grantee to read the object data and its metadata. This action is not supported by Amazon S3 on Outposts.
    pub grant_read: Option<String>,
    /// Allows grantee to read the object ACL. This action is not supported by Amazon S3 on Outposts.
    pub grant_read_acp: Option<String>,
    /// Allows grantee to write the ACL for the applicable object. This action is not supported by Amazon S3 on Outposts.
    pub grant_write_acp: Option<String>,
    /// Object key for which the PUT action was initiated.
    pub key: String,
    /// A map of metadata to store with the object in S3.
    pub metadata: Option<HashMap<String, String>>,
    /// Specifies whether a legal hold will be applied to this object. For more information about S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Object Lock</a>.
    pub object_lock_legal_hold_status: Option<String>,
    /// The Object Lock mode that you want to apply to this object.
    pub object_lock_mode: Option<String>,
    /// The date and time when you want this object's Object Lock to expire. Must be formatted as a timestamp parameter.
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// Specifies the algorithm to use to when encrypting the object (for example, AES256).
    pub sse_customer_algorithm: Option<String>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.
    pub sse_customer_key: Option<String>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<String>,
    /// Specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.
    pub ssekms_encryption_context: Option<String>,
    /// If <code>x-amz-server-side-encryption</code> is present and has the value of <code>aws:kms</code>, this header specifies the ID of the AWS Key Management Service (AWS KMS) symmetrical customer managed customer master key (CMK) that was used for the object. If you specify <code>x-amz-server-side-encryption:aws:kms</code>, but do not provide<code> x-amz-server-side-encryption-aws-kms-key-id</code>, Amazon S3 uses the AWS managed CMK in AWS to protect the data. If the KMS key does not exist in the same account issuing the command, you must use the full ARN and not just the ID.
    pub ssekms_key_id: Option<String>,
    /// The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).
    pub server_side_encryption: Option<String>,
    /// By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The STANDARD storage class provides high durability and high availability. Depending on performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses the OUTPOSTS Storage Class. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage Classes</a> in the <i>Amazon S3 User Guide</i>.
    pub storage_class: Option<String>,
    /// The tag-set for the object. The tag-set must be encoded as URL Query parameters. (For example, "Key1=Value1")
    pub tagging: Option<String>,
    /// If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata. For information about object metadata, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a>. In the following example, the request header sets the redirect to an object (anotherPage.html) in the same bucket:  <code>x-amz-website-redirect-location: /anotherPage.html</code>  In the following example, the request header sets the object redirect to another website:  <code>x-amz-website-redirect-location: http://www.example.com/</code>  For more information about website hosting in Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on Amazon S3</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html">How to Configure Website Page Redirects</a>.
    pub website_redirect_location: Option<String>,
}
