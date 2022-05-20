import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";
import { Bucket, BucketArgs, BucketPolicy } from "@pulumi/aws/s3";
import { Certificate, CertificateValidation } from "@pulumi/aws/acm";
import { Output } from "@pulumi/pulumi";
import { Distribution, OriginAccessIdentity } from "@pulumi/aws/cloudfront";

function createWebBucket(name: string, website: boolean = false): aws.s3.Bucket {
  const options: BucketArgs = {
    bucketPrefix: name,
  };

  if(website) {
    options.website = {
      indexDocument: "index.html",
      errorDocument: "index.html",
    };
  }

  return new aws.s3.Bucket(name, options);
}

function createCertificate(domainName: string): Certificate {
  return new Certificate("Certificate", {
    domainName,
    validationMethod: "DNS",
    subjectAlternativeNames: [`www.${domainName}`]
  });
}

async function getRoute53HostedZoneId(domainName: string): Promise<string> {
  const zone = await aws.route53.getZone({name: domainName});
  return zone.zoneId
}

function createRoute53ValidationRecord(certificate: Certificate, validationOptionIndex: number, zoneId: string, ttl: number): aws.route53.Record {
  if(!certificate.domainValidationOptions[validationOptionIndex]) throw new Error(`domain validation options doesn't have index ${validationOptionIndex}`);

  return new aws.route53.Record(`Validation Record - ${validationOptionIndex}`, {
    name: certificate.domainValidationOptions[validationOptionIndex].resourceRecordName,
    type: certificate.domainValidationOptions[validationOptionIndex].resourceRecordType,
    zoneId,
    records: [certificate.domainValidationOptions[validationOptionIndex].resourceRecordValue],
    ttl
  })
}

function validateCertificate(certificate: Certificate, validationRecordFqdns: Output<string>[]): CertificateValidation {
  return new CertificateValidation("Validate created certificate", {
    certificateArn: certificate.arn,
    validationRecordFqdns

  })
}

function createAccessIdentityForConnectingToS3(): OriginAccessIdentity {
  return new OriginAccessIdentity("Origin Access Identity", {
    comment: "Allowing Cloudfront to access the s3 bucket without making it public",
  });
}

function createCloudfrontDistribution(bucket: Bucket, accessIdentity: OriginAccessIdentity, certificateArn: string | Output<string>, aliases: string[], loggingBucket: Bucket, logPrefix: string): Distribution {
  return new Distribution("Cloudfront distribution", {
    defaultCacheBehavior: {
      allowedMethods: ['GET', 'HEAD', 'OPTIONS'],
      cachedMethods: ['GET', 'HEAD', 'OPTIONS'],
      targetOriginId: bucket.arn,
      viewerProtocolPolicy: 'redirect-to-https',
      forwardedValues: {
        cookies: {
          forward: 'none',
        },
        queryString: false,
      },
      defaultTtl: 600,
      maxTtl: 600,
    },
    enabled: true,
    origins: [{
      domainName: bucket.websiteEndpoint,
      originId: bucket.arn,
      s3OriginConfig: {originAccessIdentity: accessIdentity.cloudfrontAccessIdentityPath}
    }],
    restrictions: {
      geoRestriction: {
        restrictionType: 'none'
      }
    },
    viewerCertificate: {
      acmCertificateArn: certificateArn,
      sslSupportMethod: 'sni-only'
    },
    aliases,
    defaultRootObject: "index.html",
    priceClass: 'PriceClass_100',
    loggingConfig: {
      bucket: loggingBucket.bucketDomainName,
      includeCookies: false,
      prefix: logPrefix
    }
  })
}

function createBucketPolicy(bucket: Bucket, originAccessIdentity: OriginAccessIdentity): BucketPolicy {
  const policy = {
    "Id": "Policy1653055266419",
    "Version": "2012-10-17",
    "Statement": [
      {
        "Sid": "Stmt1653055262390",
        "Action": [
          "s3:GetObject"
        ],
        "Effect": "Allow",
        "Resource": [`${bucket.arn}/*`],
        "Principal": {
          "AWS": [
            originAccessIdentity.iamArn
          ]
        }
      }
    ]
  };
  return new BucketPolicy("Bucket policy", {
    bucket: bucket.id,
    policy: JSON.stringify(policy),
  })
}

function createRoute53Record(name: string, domainName: string, zoneId: string, cloudfrontDistribution: Distribution): aws.route53.Record {
  return new aws.route53.Record(name, {
    name: domainName,
    type: 'A',
    zoneId,
    aliases: [{
      evaluateTargetHealth: true,
      name: cloudfrontDistribution.domainName,
      zoneId: cloudfrontDistribution.hostedZoneId
    }]
  });
}

async function main(): Promise<void> {
  const config = new pulumi.Config();
  let certificateArn: string | undefined | Output<string> = config.get("certificateArn");
  const domainName = config.get("domainName");
  
  if(!domainName) throw new Error("missing config 'domainName'");
  
  const bucketName = `brooks_builds_platform_${pulumi.getStack()}`;
  const logBucketName = `${bucketName}_logs`;
  const bucket = createWebBucket(bucketName, true);
  const logBucket = createWebBucket(logBucketName);
  
  if(!certificateArn) {
    const certificate = createCertificate(domainName);
    const hostedZoneId = await getRoute53HostedZoneId(domainName);
    const timeToLiveInSeconds = 60 * 10;
    const validationRecord = createRoute53ValidationRecord(certificate, 0, hostedZoneId, timeToLiveInSeconds);
    const wwwValidationRecord = createRoute53ValidationRecord(certificate, 1, hostedZoneId, timeToLiveInSeconds);
    const validationRecordFqdns = [
      validationRecord.fqdn,
      wwwValidationRecord.fqdn
    ];
    const validatedCertificate = validateCertificate(certificate, validationRecordFqdns);
    certificateArn = validatedCertificate.certificateArn;
  }
  
  const originAccessIdentity = createAccessIdentityForConnectingToS3();
  const distributionAliases = [domainName, `www.${domainName}`];
  const cloudfrontDistribution = createCloudfrontDistribution(
    bucket,
    originAccessIdentity,
    certificateArn,
    distributionAliases,
    logBucket,
    "platform-"
  );
  const bucketPolicy = createBucketPolicy(bucket, originAccessIdentity);
  const route53Record = createRoute53Record(
    "route53 record",
    domainName,
    await getRoute53HostedZoneId(domainName),
    cloudfrontDistribution
  );
  const wwwRoute53Record = createRoute53Record(
    "www route53 record",
    domainName,
    await getRoute53HostedZoneId(domainName),
    cloudfrontDistribution
  );
}

main().then(() => console.log("finished"));
