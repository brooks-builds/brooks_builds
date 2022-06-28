import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";

const stack = pulumi.getStack();

const platformFrontendBucket = new aws.s3.Bucket("platformWebBucket", {
    bucketPrefix: `platform-frontend-${stack}`,
    tags: {
        project: `platform`,
        stack
    }
})
