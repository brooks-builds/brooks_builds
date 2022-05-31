import * as fs from 'fs';
import * as aws from '@pulumi/aws';
import { Output } from '@pulumi/pulumi';
import BBAwsVpc from './vpc';
import { getAmi } from '@pulumi/aws/ec2/getAmi';

export default class BBAwsEc2 {
  keyName: string;
  userdata: string;
  instance: aws.ec2.Instance | null = null;
  ami: aws.ec2.GetAmiResult | null = null;
  vpc: BBAwsVpc;

  constructor(keyName: string, userdataPath: string, vpc: BBAwsVpc) {
    this.keyName = keyName;
    this.vpc = vpc;

    this.userdata = this.loadEc2Userdata(userdataPath);
  }
  
  async load(): Promise<this> {
    this.ami = await this.loadAmi();

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

//   createInstance(name: string, securityGroupIds: Output<string>[]): aws.ec2.Instance {
//   return new aws.ec2.Instance(name, {
//     ami: this.ami?.id,
//     associatePublicIpAddress: true,
//     vpcSecurityGroupIds: securityGroupIds,
//     instanceType: 't2.micro',
//     keyName,
//     subnetId,
//     userData
//   });
// }
}