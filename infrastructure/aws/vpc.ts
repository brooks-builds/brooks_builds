import { Subnet } from "@pulumi/aws/ec2";
import { Output } from "@pulumi/pulumi";

export default class BBAwsVpc {
  vpcId: string;
  backupSubnet: Subnet;
  primarySubnetId: string;

  constructor(vpcId: string, primarySubnetId: string) {
    this.vpcId = vpcId;
    this.backupSubnet = this.createSubnet('BackupPublicSubnet', 'us-east-1b', '10.0.2.0/24');
    this.primarySubnetId = primarySubnetId;
  }
  
  public get id() : string {
    return this.vpcId;
  }

  public get getPrimarySubnetId(): string {
    return this.primarySubnetId
  }

  public get getAllSPublicSubnetIds(): (string|Output<string>)[] {
    return [
      this.primarySubnetId,
      this.backupSubnet.id,
    ]
  }
  
  createSubnet(name: string, availabilityZone: string, cidrBlock: string): Subnet {
  return new Subnet(name, {
    vpcId: this.id,
    availabilityZone,
    cidrBlock,
    tags: {
      Name: name
    }
  });
}
}