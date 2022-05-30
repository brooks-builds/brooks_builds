import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";
import { Ami, getAmi, SecurityGroup, Vpc } from "@pulumi/aws/ec2";
import { Input, Output } from "@pulumi/pulumi";
import * as fs from 'fs';
import { LoadBalancer } from "@pulumi/aws/lb";
import bbAwsVpc from './aws_vpc';

function createEc2Instance(name: string, ami: aws.ec2.GetAmiResult, securityGroupIds: Output<string>[], keyName: string, subnetId: string, userData: string): aws.ec2.Instance {
  return new aws.ec2.Instance(name, {
    ami: ami.id,
    associatePublicIpAddress: true,
    vpcSecurityGroupIds: securityGroupIds,
    instanceType: 't2.micro',
    keyName,
    subnetId,
    userData
  });
}

function getAwsAmi(): Promise<aws.ec2.GetAmiResult> {
  return getAmi({
    owners: ['amazon'],
    filters: [{
      name: 'image-id',
      values: ['ami-0022f774911c1d690']
    }],
    mostRecent: true
  });
}

function createNormalUsageSecurityGroup(vpcId: string): SecurityGroup {
  const name = 'Allow normal access to hosted services';
  return new SecurityGroup('Usage Security Group', {
    description: 'Security group allowing Normal usage',
    egress: [{
      fromPort: 0,
      protocol: 'ALL',
      toPort: 0,
      cidrBlocks: ['0.0.0.0/0'],
      description: 'Allow everything everywhere',
    }],
    ingress: [{
      fromPort: 5341,
      protocol: 'tcp',
      toPort: 5341,
      cidrBlocks: ['0.0.0.0/0'],
      description: 'Allow access to SEQ injest from anywhere in!',
    }],
    name,
    vpcId,
    tags: {
      Name: name
    }
  })
}

function getEc2UserdataScript(): string {
  return fs.readFileSync("./userdata/ec2.sh", {encoding: 'utf8'});
}

function getAwsSubnetIds(vpcId: string):Promise<aws.ec2.GetSubnetIdsResult> {
  return aws.ec2.getSubnetIds({
    vpcId
  });
}

export async function main(): Promise<any> {
  const config = new pulumi.Config();
  const vpcId = config.require('vpcId');
  const keyName = config.require('keyName');
  const subnetId = config.require('subnetId');
  const ami = await getAwsAmi();
  const sshSecurityGroup = createSSHSecurityGroup(vpcId);
  const adminSecurityGroup = createAdminGuiSecurityGroup(vpcId);
  const normalUsageSecurityGroup = createNormalUsageSecurityGroup(vpcId);
  const internetToLoadBalancerSecurityGroups = [
    adminSecurityGroup.id,
    normalUsageSecurityGroup.id,
  ];
  const loadBalancerToEc2SecurityGroups = [
    sshSecurityGroup.id
  ];
  const ec2UserData = getEc2UserdataScript();
  const ec2 = createEc2Instance("brooks-builds-ec2", ami, loadBalancerToEc2SecurityGroups, keyName, subnetId, ec2UserData);
  const awsVpc = bbAwsVpc(vpcId);
  const subnetIds = await getAwsSubnetIds(vpcId);
  return {};
}

export const output = main();