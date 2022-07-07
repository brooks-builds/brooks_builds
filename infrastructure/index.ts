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

const platformFrontendLogBucket = new aws.s3.Bucket("patformWebLogBucket", {
    bucketPrefix: `platform-frontend-${STACK}-log`,
    tags: {
        project: 'platform',
        stack: STACK
    }
})

const athenaQueryBucket = new aws.s3.Bucket("athenaBucket", {
    bucketPrefix: `athena-query-results`,
})

const platformCloudfrontOriginId = "S3PlatformOrigin";

const brooksBuildsCertificate = new aws.acm.Certificate("brooksbuildsCertificate", {
    domainName: DOMAIN_NAME,
    validationMethod: "DNS",
    subjectAlternativeNames: [`www.${DOMAIN_NAME}`],
    tags: {
        project: 'platform',
        stack: STACK
    }
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
            records: [domainValidationOption.resourceRecordValue],
            allowOverwrite: true
        }));
    });
});

const validatedCertificate = new aws.acm.CertificateValidation(`${DOMAIN_NAME}-Validation`, {
    certificateArn: brooksBuildsCertificate.arn,
    validationRecordFqdns: domainValidationRecords.map(domainValidationRecord => domainValidationRecord.fqdn),
},
{dependsOn: domainValidationRecords});

const originAccessIdentity = new aws.cloudfront.OriginAccessIdentity("CloudfrontOriginAccessIdentity", {
    comment: "access s3"
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
        },
        defaultTtl: 2.628e+6,
        compress: true
    },
    enabled: true,
    origins: [{
        domainName: platformFrontendBucket.bucketDomainName,
        originId: platformCloudfrontOriginId,
        s3OriginConfig: {
            originAccessIdentity: originAccessIdentity.cloudfrontAccessIdentityPath,
        },
    }],
    restrictions: {
        geoRestriction: {
            restrictionType: "none"
        },
    },
    viewerCertificate: {
        cloudfrontDefaultCertificate: false,
        acmCertificateArn: brooksBuildsCertificate.arn,
        sslSupportMethod: "sni-only",
        minimumProtocolVersion: "TLSv1.2_2021"
    },
    comment: "Cloudfront distribution for the Brooks Builds Platform",
    priceClass: "PriceClass_100",
    aliases: ["brooksbuilds.com", "www.brooksbuilds.com"],
    defaultRootObject: "index.html",
    tags: {
        project: 'platform',
        stack: STACK
    },
    loggingConfig: {
        bucket: platformFrontendLogBucket.bucketDomainName,
    },
    customErrorResponses: [{
        errorCode: 404,
        errorCachingMinTtl: 2.628e+6,
        responseCode: 200,
        responsePagePath: '/'
    }]
}, {
    dependsOn: [validatedCertificate, brooksBuildsCertificate]
});

const mainDomainRecord = new aws.route53.Record(`${DOMAIN_NAME}-record`, {
    name: `${DOMAIN_NAME}`,
    type: "A",
    zoneId: zoneId.then(zoneId => zoneId.id),
    aliases: [{
        evaluateTargetHealth: false,
        name: cloudfrontDistribution.domainName,
        zoneId: cloudfrontDistribution.hostedZoneId
    }],
    allowOverwrite: true,
});

const wwwDomainRecord = new aws.route53.Record(`www.${DOMAIN_NAME}-record`, {
    name: `www.${DOMAIN_NAME}`,
    type: "A",
    zoneId: zoneId.then(zoneId => zoneId.id),
    aliases: [{
        evaluateTargetHealth: false,
        name: cloudfrontDistribution.domainName,
        zoneId: cloudfrontDistribution.hostedZoneId
    }],
    allowOverwrite: true,
});
