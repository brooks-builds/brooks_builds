import * as aws from '@pulumi/aws';
import { SecurityGroup } from '@pulumi/aws/ec2';
import { LoadBalancer } from '@pulumi/aws/lb';
import { Output } from '@pulumi/pulumi';

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

function createAwsLoadBalancer(securityGroupIds: Output<string>[], subnetIds: string[], enableDeletionProtection: boolean = true): LoadBalancer {
  return new LoadBalancer("LoadBalancer", {
    internal: false,
    loadBalancerType: "application",
    securityGroups: securityGroupIds,
    subnets: subnetIds,
    enableDeletionProtection,
    name: 'BrooksBuildsLoadBalancer',
})
}

function createAwsSecurityGroup(pulumiName: string, name: string, description: string, port: number, vpcId: string, cidrBlocks: string[] = ['0.0.0.0/0'], ingressDescription: string = 'Allow from everywhere'): SecurityGroup {
  return new SecurityGroup(pulumiName, {
    description,
    egress: [{
      fromPort: 0,
      protocol: 'ALL',
      toPort: 0,
      cidrBlocks: ['0.0.0.0/0'],
      description: 'Allow everything everywhere',
    }],
    ingress: [{
      fromPort: port,
      protocol: 'tcp',
      toPort: port,
      cidrBlocks,
      description: ingressDescription,
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

export default function setupAwsVpc(vpcId: string): BbAwsVpc {
  const subnetInSecondAvailabilityZone = createAwsSubnet(
    "BackupPublicSubnet",
    vpcId,
    'us-east-1b',
    '10.0.2.0/24'
  );

  const securityGroups = {
    sshAccess: createAwsSecurityGroup(
      'SSH Security Group',
      'Allow SSH',
      'Security group allowing SSH into a resource',
      22,
      vpcId
    ),
    adminWeb: createAwsSecurityGroup(
      'GUI Admin Security Group',
      'Allow Admin access to web apps',
      'Security group allowing Admin access',
      8000,
      vpcId
    ),
    normalUsage: createAwsSecurityGroup(
      'Normal usage Security Group',
      'Normal usage for services and apps',
      'Allow traffic for normal usage',
      5341,
      vpcId
    )
  }

  const loadBalancer = createAwsLoadBalancer(internetToLoadBalancerSecurityGroups, subnetIds.ids, false);

  return {
    subnetInSecondAvailabilityZone
  };
}