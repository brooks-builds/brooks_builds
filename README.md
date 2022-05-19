# Brooks Builds Platform

## Deployment

I'm using Pulumi to deploy the web app to AWS using S3 to store the files, and Cloudfront to serve them. I'm using a guide at https://github.com/pulumi/examples/tree/master/aws-ts-static-website to build out the deployment part.

I'm going to be doing things slightly differently as I'll be skipping having Pulumi upload the files and using a script to sync them using the AWS CLI separately.

The general idea is

- [ ] Create the S3 bucket
- [ ] Build the project for release using `trunk`
- [ ] Create the Cloudfront distribution
- [ ] Create a Route53 record