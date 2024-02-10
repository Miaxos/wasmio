use derivative::Derivative;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBucketConfiguration {
    /*
    /// Specifies the Region where the bucket will be created. You might choose a Region to optimize latency, minimize costs, or address regulatory requirements. For example, if you reside in Europe, you will probably find it advantageous to create buckets in the Europe (Ireland) Region. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingBucket.html#access-bucket-intro">Accessing a bucket</a> in the Amazon S3 User Guide.
    /// If you don't specify a Region, the bucket is created in the US East (N. Virginia) Region (us-east-1) by default.
    /// This functionality is not supported for directory buckets.
    ///
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: Option<BucketLocationConstraint>,
    /// Specifies the location where the bucket will be created.
    /// For directory buckets, the location type is Availability Zone.
    /// This functionality is only supported by directory buckets.
    ///
    pub location: Option<LocationInfo>,
    /// Specifies the information about the bucket that will be created.
    /// This functionality is only supported by directory buckets.
    ///
    pub bucket: Option<BucketInfo>,
    */
}

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct CreateBucketOutput {
    /// Specifies the Region where the bucket will be created.
    pub location: Option<String>,
}

#[derive(Derivative, Default, Builder)]
#[derivative(Debug)]
#[builder(pattern = "owned", setter(into), default)]
pub struct CreateBucketRequest {
    /// The canned ACL to apply to the bucket.
    pub acl: Option<String>,
    /// The name of the bucket to create.
    pub bucket: String,
    /// The configuration information for the bucket.
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    /// Allows grantee the read, write, read ACP, and write ACP permissions
    /// on the bucket.
    pub grant_full_control: Option<String>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<String>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<String>,
    /// Allows grantee to create new objects in the bucket. For the
    /// bucket and object owners of existing objects, also allows deletions and
    /// overwrites of those objects.
    pub grant_write: Option<String>,
    /// Allows grantee to write the ACL for the applicable bucket.
    pub grant_write_acp: Option<String>,
    /// Specifies whether you want S3 Object Lock to be enabled for the new
    /// bucket.
    pub object_lock_enabled_for_bucket: Option<bool>,
}
