use derivative::Derivative;
use derive_builder::Builder;

use super::common::CommonPrefix;
use super::object::Object;

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct ListObjectsRequest {
    /// The name of the bucket containing the objects. When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the AWS SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>. When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com. When using this action using S3 on Outposts through the AWS SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.
    pub bucket: String,
    /// A delimiter is a character you use to group keys.
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// The account ID of the expected bucket owner. If the bucket is owned
    /// by a different account, the request will fail with an HTTP <code>403
    /// (Access Denied)</code> error.
    pub expected_bucket_owner: Option<String>,
    /// Specifies the key to start with when listing objects in a
    /// bucket.
    pub marker: Option<String>,
    /// Sets the maximum number of keys returned in the response. By default
    /// the action returns up to 1,000 key names. The response might contain
    /// fewer keys but will never contain more.
    pub max_keys: Option<i64>,
    /// Limits the response to keys that begin with the specified
    /// prefix.
    pub prefix: Option<String>,
    /// Confirms that the requester knows that she or he will be charged for
    /// the list objects request. Bucket owners need not specify this parameter
    /// in their requests.
    pub request_payer: Option<String>,
}

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct ListObjectsOutput {
    /// All of the keys (up to 1,000) rolled up in a common prefix count as
    /// a single return when calculating the number of returns.  A
    /// response can contain CommonPrefixes only if you specify a
    /// delimiter. CommonPrefixes contains all (if there are any) keys
    /// between Prefix and the next occurrence of the string specified by the
    /// delimiter.  CommonPrefixes lists keys that act like
    /// subdirectories in the directory specified by Prefix. For
    /// example, if the prefix is notes/ and the delimiter is a slash (/) as in
    /// notes/summer/july, the common prefix is notes/summer/. All of the keys
    /// that roll up into a common prefix count as a single return when
    /// calculating the number of returns.
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// Metadata about each object returned.
    pub contents: Option<Vec<Object>>,
    /// Causes keys that contain the same string between the prefix and the
    /// first occurrence of the delimiter to be rolled up into a single result
    /// element in the <code>CommonPrefixes</code> collection. These rolled-up
    /// keys are not returned elsewhere in the response. Each rolled-up result
    /// counts as only one return against the <code>MaxKeys</code> value.
    pub delimiter: Option<String>,
    /// Encoding type used by Amazon S3 to encode object keys in the
    /// response.
    pub encoding_type: Option<String>,
    /// A flag that indicates whether Amazon S3 returned all of the results
    /// that satisfied the search criteria.
    pub is_truncated: Option<bool>,
    /// Indicates where in the bucket listing begins. Marker is included in
    /// the response if it was sent with the request.
    pub marker: Option<String>,
    /// The maximum number of keys returned in the response body.
    pub max_keys: Option<i64>,
    /// The bucket name.
    pub name: Option<String>,
    /// When response is truncated (the IsTruncated element value in the
    /// response is true), you can use the key name in this field as marker in
    /// the subsequent request to get next set of objects. Amazon S3 lists
    /// objects in alphabetical order Note: This element is returned only if
    /// you have delimiter request parameter specified. If response does not
    /// include the NextMarker and it is truncated, you can use the value of
    /// the last Key in the response as the marker in the subsequent request to
    /// get the next set of object keys.
    pub next_marker: Option<String>,
    /// Keys that begin with the indicated prefix.
    pub prefix: Option<String>,
}
