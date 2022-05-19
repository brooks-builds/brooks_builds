import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";
import { BucketArgs } from "@pulumi/aws/s3";
import { Certificate, CertificateValidation } from "@pulumi/aws/acm";
import { Output } from "@pulumi/pulumi";

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

  return new aws.route53.Record("Validation Record", {
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

async function main(): Promise<void> {
  const config = new pulumi.Config();
  let certificateArn = config.get("certificateArn");
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
    certificateArn = validatedCertificate.certificateArn.get();
  }
}

main().then(() => console.log("finished"));
