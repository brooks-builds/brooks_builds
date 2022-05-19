import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";

function createBucket(): aws.s3.Bucket {
  return new aws.s3.Bucket("Code S3 Bucket", {
    bucketPrefix: "brooks_builds_platform_frontend",
    website: {
      indexDocument: "index.html",
      errorDocument: "index.html",
    }
  })
} 

const bucket = createBucket();