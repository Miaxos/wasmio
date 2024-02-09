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
