import * as aws from '@pulumi/aws';

interface BbAwsVpc {
  subnetInSecondAvailabilityZone: aws.ec2.Subnet
}

function createAwsSubnet(name: string, vpcId: string, availabilityZone: string, cidrBlock: string): aws.ec2.Subnet {
  return new aws.ec2.Subnet(name, {
    vpcId,
    availabilityZone,
    cidrBlock,
    tags: {
      Name: name
    }
  });
}

export default function setupAwsVpc(vpcId: string): BbAwsVpc {
  const subnetInSecondAvailabilityZone = createAwsSubnet(
    "BackupPublicSubnet",
    vpcId,
    'us-east-1b',
    '10.0.2.0/24'
  )

  return {
    subnetInSecondAvailabilityZone
  };
}