# Brooks Builds Platform

## Deployment

I'm using Pulumi to deploy the web app to AWS using S3 to store the files, and Cloudfront to serve them. I'm using a guide at https://github.com/pulumi/examples/tree/master/aws-ts-static-website to build out the deployment part.

I'm going to be doing things slightly differently as I'll be skipping having Pulumi upload the files and using a script to sync them using the AWS CLI separately.

The general idea is

- [x] Create the S3 buckets
  - [x] Code
  - [x] Logs
- [x] Certificate
  - [x] Get existing if exists in config
  - [x] Create if not
- [ ] Create a Route53 record
- [ ] Create the Cloudfront distribution
- [ ] Build the project for release using `trunk`

left off at https://github.com/pulumi/examples/blob/2ee921d930a40e4be20ce68b12a1e00356888b5f/aws-ts-static-website/index.ts#L156