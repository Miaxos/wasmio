use derivative::Derivative;
use derive_builder::Builder;

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct DeleteObjectRequest {
    /// The bucket name of the bucket containing the object.  When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form *AccessPointName*-*AccountId*.s3-accesspoint.*Region*.amazonaws.com. When using this action with an access point through the AWS SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the *Amazon S3 User Guide*. When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form *AccessPointName*-*AccountId*.*outpostID*.s3-outposts.*Region*.amazonaws.com. When using this action using S3 on Outposts through the AWS SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using S3 on Outposts</a> in the *Amazon S3 User Guide*.
    pub bucket: String,
    /// Indicates whether S3 Object Lock should bypass Governance-mode
    /// restrictions to process this operation.
    pub bypass_governance_retention: Option<bool>,
    /// The account ID of the expected bucket owner. If the bucket is owned by
    /// a different account, the request will fail with an HTTP `403 (Access
    /// Denied)` error.
    pub expected_bucket_owner: Option<String>,
    /// Key name of the object to delete.
    pub key: String,
    /// The concatenation of the authentication device's serial number, a
    /// space, and the value that is displayed on your authentication device.
    /// Required to permanently delete a versioned object if versioning is
    /// configured with MFA delete enabled.
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
    /// VersionId used to reference a specific version of the object.
    pub version_id: Option<String>,
}

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct DeleteObjectOutput {
    /// Specifies whether the versioned object that was permanently deleted was
    /// (true) or was not (false) a delete marker.
    pub delete_marker: Option<bool>,
    pub request_charged: Option<String>,
    /// Returns the version ID of the delete marker created as a result of the
    /// DELETE operation.
    pub version_id: Option<String>,
}
