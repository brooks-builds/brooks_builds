import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as awsx from "@pulumi/awsx";
import { Ami, getAmi, SecurityGroup, Vpc } from "@pulumi/aws/ec2";

function createEc2Instance(name: string, ami: aws.ec2.GetAmiResult, securityGroup: SecurityGroup, vpcId: string): aws.ec2.Instance {
  return new aws.ec2.Instance(name, {
    ami: ami.id,
    associatePublicIpAddress: true,
    vpcSecurityGroupIds: [
      securityGroup.id,
    ],
    instanceType: 't2.micro',
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

function createSSHSecurityGroup(vpcId: string): SecurityGroup {
  const name = 'Allow SSH';
  return new SecurityGroup('SSH Security Group', {
    description: 'Security group allowing SSH into a resource',
    egress: [{
      fromPort: 0,
      protocol: 'ALL',
      toPort: 0,
      cidrBlocks: ['0.0.0.0/0'],
      description: 'Allow everything everywhere',
    }],
    ingress: [{
      fromPort: 22,
      protocol: 'tcp',
      toPort: 22,
      cidrBlocks: ['0.0.0.0/0'],
      description: 'Allow SSH from anywhere in!',
    }],
    name,
    vpcId,
    tags: {
      Name: name
    }
  })
}

export async function main(): Promise<any> {
  const config = new pulumi.Config();
  const vpcId = config.require('vpcId');
  const ami = await getAwsAmi();
  const sshSecurityGroup = createSSHSecurityGroup(vpcId);
  const ec2 = createEc2Instance("brooks-builds-ec2", ami, sshSecurityGroup);
  return {
  };
}

main()