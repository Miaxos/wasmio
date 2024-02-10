use derivative::Derivative;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::common::CommonPrefix;
use super::object::Object;

#[derive(Derivative, Default, Builder, Serialize, Deserialize)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct ListObjectsV2Request {
    /// Bucket name to list.  When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form *AccessPointName*-*AccountId*.s3-accesspoint.*Region*.amazonaws.com. When using this action with an access point through the AWS SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the *Amazon S3 User Guide*. When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form *AccessPointName*-*AccountId*.*outpostID*.s3-outposts.*Region*.amazonaws.com. When using this action using S3 on Outposts through the AWS SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using S3 on Outposts</a> in the *Amazon S3 User Guide*.
    pub bucket: String,
    /// ContinuationToken indicates Amazon S3 that the list is being
    /// continued on this bucket with a token. ContinuationToken is obfuscated
    /// and is not a real key.
    pub continuation_token: Option<String>,
    /// A delimiter is a character you use to group keys.
    pub delimiter: Option<String>,
    /// Encoding type used by Amazon S3 to encode object keys in the
    /// response.
    pub encoding_type: Option<String>,
    /// The account ID of the expected bucket owner. If the bucket is owned
    /// by a different account, the request will fail with an HTTP <code>403
    /// (Access Denied)</code> error.
    pub expected_bucket_owner: Option<String>,
    /// The owner field is not present in listV2 by default, if you want to
    /// return owner field with each key in the result then set the fetch owner
    /// field to true.
    pub fetch_owner: Option<bool>,
    /// Sets the maximum number of keys returned in the response. By default
    /// the action returns up to 1,000 key names. The response might contain
    /// fewer keys but will never contain more.
    pub max_keys: Option<i64>,
    /// Limits the response to keys that begin with the specified
    /// prefix.
    pub prefix: Option<String>,
    /// Confirms that the requester knows that she or he will be charged for
    /// the list objects request in V2 style. Bucket owners need not specify
    /// this parameter in their requests.
    pub request_payer: Option<String>,
    /// StartAfter is where you want Amazon S3 to start listing from. Amazon
    /// S3 starts listing after this specified key. StartAfter can be any key
    /// in the bucket.
    pub start_after: Option<String>,
}

#[derive(Derivative, Default, Builder, Serialize, Deserialize)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
#[serde(rename = "ListBucketResult")]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsV2Output {
    /// All of the keys (up to 1,000) rolled up into a common prefix count
    /// as a single return when calculating the number of returns. A
    /// response can contain <code>CommonPrefixes</code> only if you specify a
    /// delimiter.  <code>CommonPrefixes</code> contains all (if there
    /// are any) keys between <code>Prefix</code> and the next occurrence of
    /// the string specified by a delimiter.
    /// <code>CommonPrefixes</code> lists keys that act like subdirectories in
    /// the directory specified by <code>Prefix</code>. For example, if
    /// the prefix is <code>notes/</code> and the delimiter is a slash
    /// (<code>/</code>) as in <code>notes/summer/july</code>, the common
    /// prefix is <code>notes/summer/</code>. All of the keys that roll up into
    /// a common prefix count as a single return when calculating the number of
    /// returns.
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// Metadata about each object returned.
    pub contents: Option<Vec<Object>>,
    ///  If ContinuationToken was sent with the request, it is included in
    /// the response.
    pub continuation_token: Option<String>,
    /// Causes keys that contain the same string between the prefix and the
    /// first occurrence of the delimiter to be rolled up into a single result
    /// element in the CommonPrefixes collection. These rolled-up keys are not
    /// returned elsewhere in the response. Each rolled-up result counts as
    /// only one return against the <code>MaxKeys</code> value.
    pub delimiter: Option<String>,
    /// Encoding type used by Amazon S3 to encode object key names in the
    /// XML response. If you specify the encoding-type request
    /// parameter, Amazon S3 includes this element in the response, and returns
    /// encoded key name values in the following response elements:
    /// <code>Delimiter, Prefix, Key,</code> and <code>StartAfter</code>.
    pub encoding_type: Option<String>,
    /// Set to false if all of the results were returned. Set to true if
    /// more keys are available to return. If the number of results exceeds
    /// that specified by MaxKeys, all of the results might not be
    /// returned.
    pub is_truncated: Option<bool>,
    /// KeyCount is the number of keys returned with this request. KeyCount
    /// will always be less than or equals to MaxKeys field. Say you ask for 50
    /// keys, your result will include less than equals 50 keys
    pub key_count: Option<i64>,
    /// Sets the maximum number of keys returned in the response. By default
    /// the action returns up to 1,000 key names. The response might contain
    /// fewer keys but will never contain more.
    pub max_keys: Option<i64>,
    /// The bucket name. When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form *AccessPointName*-*AccountId*.s3-accesspoint.*Region*.amazonaws.com. When using this action with an access point through the AWS SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the *Amazon S3 User Guide*. When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form *AccessPointName*-*AccountId*.*outpostID*.s3-outposts.*Region*.amazonaws.com. When using this action using S3 on Outposts through the AWS SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using S3 on Outposts</a> in the *Amazon S3 User Guide*.
    pub name: Option<String>,
    ///  <code>NextContinuationToken</code> is sent when
    /// <code>isTruncated</code> is true, which means there are more keys in
    /// the bucket that can be listed. The next list requests to Amazon S3 can
    /// be continued with this <code>NextContinuationToken</code>.
    /// <code>NextContinuationToken</code> is obfuscated and is not a real
    /// key
    pub next_continuation_token: Option<String>,
    ///  Keys that begin with the indicated prefix.
    pub prefix: Option<String>,
    /// If StartAfter was sent with the request, it is included in the
    /// response.
    pub start_after: Option<String>,
}
