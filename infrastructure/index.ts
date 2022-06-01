import * as pulumi from '@pulumi/pulumi';
import BBAwsVpc from './aws/vpc';
import {bbAwsEc2Init} from './aws/ec2';
import BBAwsSecurityGroups from './aws/securityGroups';

// export async function main(): Promise<any> {
//   const awsVpc = bbAwsVpc(vpcId);
//   const subnetIds = await getAwsSubnetIds(vpcId);
//   return {};
// }

async function main(): Promise<any> {
  const config = new pulumi.Config();
  const bbAwsVpc = new BBAwsVpc(config.require('vpcId'), config.require('subnetId'));
  const bbAwsEc2 = await bbAwsEc2Init(
    config.require('keyName'), 
    './userdata/ec2.sh',
    bbAwsVpc,
    new BBAwsSecurityGroups(bbAwsVpc)
  );

  return {
    ami: bbAwsEc2.ami?.id
  }
}

export const output = main();