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

const platformCloudfrontOriginId = "S3PlatformOrigin";

let brooksBuildsCertificate = new aws.acm.Certificate("brooksbuildsCertificate", {
    
});

const cloudfrontDistribution = new aws.cloudfront.Distribution("platformCloudfront", {
    defaultCacheBehavior: {
        allowedMethods: ["GET", "HEAD"],
        cachedMethods: ["GET", "HEAD"],
        targetOriginId: platformCloudfrontOriginId,
        viewerProtocolPolicy: "redirect-to-https",
        forwardedValues: {
            cookies: {
                forward: "all"
            },
            queryString: true
        }
    },
    enabled: false,
    origins: [{
        domainName: platformFrontendBucket.bucketDomainName,
        originId: platformCloudfrontOriginId,
    }],
    restrictions: {
        geoRestriction: {
            restrictionType: "none"
        },
    },
    viewerCertificate: {
        cloudfrontDefaultCertificate: false,
        acmCertificateArn: brooksBuildsCertificate.arn
    },
    comment: "Cloudfront distribution for the Brooks Builds Platform",
    priceClass: "PriceClass_100",
    aliases: ["brooksbuilds.com", "www.brooksbuilds.com"]
})
