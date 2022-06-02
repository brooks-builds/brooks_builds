import * as pulumi from '@pulumi/pulumi';
import BBAwsVpc from './aws/vpc';
import {bbAwsEc2Init} from './aws/ec2';
import BBAwsSecurityGroups from './aws/securityGroups';
import BBAwsLoadBalancer from './aws/loadBalancer';

async function main(): Promise<any> {
  const config = new pulumi.Config();
  const bbAwsVpc = new BBAwsVpc(config.require('vpcId'), config.require('subnetId'));
  const securityGroups = new BBAwsSecurityGroups(bbAwsVpc);
  const bbAwsEc2 = await bbAwsEc2Init(
    config.require('keyName'), 
    './userdata/ec2.sh',
    bbAwsVpc,
    securityGroups,
  );
  const loadBalancer = new BBAwsLoadBalancer(securityGroups, bbAwsVpc);

  return {
    ami: bbAwsEc2.ami?.id
  }
}

export const output = main();