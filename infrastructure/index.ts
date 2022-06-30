import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";

const DOMAIN_NAME = "brooksbuilds.com";
const STACK = pulumi.getStack();

const config = new pulumi.Config();

const platformFrontendBucket = new aws.s3.Bucket("platformWebBucket", {
    bucketPrefix: `platform-frontend-${STACK}`,
    tags: {
        project: `platform`,
        stack: STACK
    }
})

const platformCloudfrontOriginId = "S3PlatformOrigin";

const brooksBuildsCertificate = new aws.acm.Certificate("brooksbuildsCertificate", {
    domainName: DOMAIN_NAME,
    validationMethod: "DNS"
});

const zoneId = aws.route53.getZone({name: DOMAIN_NAME});
const domainValidationRecords: aws.route53.Record[] = [];
brooksBuildsCertificate.domainValidationOptions.apply(domainValidationOptions => {
    domainValidationOptions.forEach((domainValidationOption, domainValidationOptionIndex) => {
        domainValidationRecords.push(new aws.route53.Record(`${DOMAIN_NAME}-validation-${domainValidationOptionIndex}`, {
            name: domainValidationOption.resourceRecordName,
            type: domainValidationOption.resourceRecordType,
            zoneId: zoneId.then(zoneId => zoneId.id),
            ttl: config.requireNumber("domainTtl"),
            records: [domainValidationOption.resourceRecordValue]
        }));
    });
});

const validatedCertificate = new aws.acm.CertificateValidation(`${DOMAIN_NAME}-Validation`, {
    certificateArn: brooksBuildsCertificate.arn,
    validationRecordFqdns: domainValidationRecords.map(domainValidationRecord => domainValidationRecord.fqdn),
},
{dependsOn: domainValidationRecords});

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
        acmCertificateArn: brooksBuildsCertificate.arn,
        sslSupportMethod: "sni-only"
    },
    comment: "Cloudfront distribution for the Brooks Builds Platform",
    priceClass: "PriceClass_100",
    aliases: ["brooksbuilds.com", "www.brooksbuilds.com"],
}, {
    dependsOn: validatedCertificate
});
