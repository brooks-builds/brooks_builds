import * as fs from 'fs';
import * as aws from '@pulumi/aws';
import { Output } from '@pulumi/pulumi';
import BBAwsVpc from './vpc';
import { getAmi } from '@pulumi/aws/ec2/getAmi';
import { SecurityGroup } from '@pulumi/aws/ec2';
import BBAwsSecurityGroup from './securityGroups';
import BBAwsSecurityGroups from './securityGroups';

class BBAwsEc2 {
  keyName: string;
  userdata: string;
  instance: aws.ec2.Instance | null = null;
  ami: aws.ec2.GetAmiResult | null = null;
  vpc: BBAwsVpc;
  securityGroups: BBAwsSecurityGroup;

  constructor(keyName: string, userdataPath: string, vpc: BBAwsVpc, securityGroups: BBAwsSecurityGroup) {
    this.keyName = keyName;
    this.vpc = vpc;

    this.userdata = this.loadEc2Userdata(userdataPath);
    this.securityGroups = securityGroups;
  }
  
  async load(): Promise<this> {
    this.ami = await this.loadAmi();
    this.instance = this.createInstance('brooks-builds-ec2');

    return this;
  }

  loadEc2Userdata(path: string): string {
    return fs.readFileSync(path, {encoding: 'utf8'});
  }

  async loadAmi(): Promise<aws.ec2.GetAmiResult> {
    const awsLinuxImageId = 'ami-0022f774911c1d690';

    return getAmi({
      owners: ['amazon'],
      filters: [{
        name: 'image-id',
        values: [awsLinuxImageId]
      }],
      mostRecent: true
    });
  }

  createInstance(pulumiName: string): aws.ec2.Instance {
    return new aws.ec2.Instance(pulumiName, {
      ami: this.ami?.id,
      associatePublicIpAddress: true,
      vpcSecurityGroupIds: [this.securityGroups.sshSecurityGroup.id],
      instanceType: 't2.micro',
      keyName: this.keyName,
      subnetId: this.vpc.getPrimarySubnetId,
      userData: this.userdata
    });
  }
}

export async function bbAwsEc2Init(keyname: string, userdataPath: string, vpc: BBAwsVpc, securityGroups: BBAwsSecurityGroups): Promise<BBAwsEc2> {
  return new BBAwsEc2(keyname, userdataPath, vpc, securityGroups);
}
