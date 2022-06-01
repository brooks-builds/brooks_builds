import { SecurityGroup } from "@pulumi/aws/ec2";
import { Output } from "@pulumi/pulumi";
import BBAwsVpc from "./vpc";

export default class BBAwsSecurityGroups {
  public sshSecurityGroup: SecurityGroup;
  public webAdminSecurityGroup: SecurityGroup;
  public normalUsageSecurityGroup: SecurityGroup;
  vpc: BBAwsVpc;

  constructor(vpc: BBAwsVpc) {
    this.vpc = vpc;
    this.sshSecurityGroup = this.createSecurityGroup('SSH Security Group', 'SSH Security Group', 22);
    this.webAdminSecurityGroup = this.createSecurityGroup('GUI Admin Security Group', 'Access services through web admin portals', 8000);
    this.normalUsageSecurityGroup = this.createSecurityGroup('Normal usage Security Group', 'Access services as a regular user', 5341);
  }

  createSecurityGroup(pulumiName: string, description: string, port: number): SecurityGroup {
    return new SecurityGroup(pulumiName, {
      description,
      egress: [{
        fromPort: 0,
        protocol: 'ALL',
        toPort: 0,
        cidrBlocks: ['0.0.0.0/0'],
        description: 'Allow everything out to everywhere',
      }],
      ingress: [{
        fromPort: port,
        protocol: 'tcp',
        toPort: port,
        cidrBlocks: ['0.0.0.0/0'],
      }],
      name: pulumiName,
      tags: {
        Name: pulumiName
      },
      vpcId: this.vpc.id
    });
  },

  public get getInternetToLoadBalancerSecurityGroupIds(): Output<string>[] {
    return [
      this.normalUsageSecurityGroup.id,
      this.webAdminSecurityGroup.id,
    ]
  }
}