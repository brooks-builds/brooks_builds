import * as pulumi from '@pulumi/pulumi';
import BBAwsVpc from './aws/vpc';
import BBAwsEc2 from './aws/ec2';

// export async function main(): Promise<any> {
//   const ec2 = createEc2Instance("brooks-builds-ec2", ami, loadBalancerToEc2SecurityGroups, keyName, subnetId, ec2UserData);
//   const awsVpc = bbAwsVpc(vpcId);
//   const subnetIds = await getAwsSubnetIds(vpcId);
//   return {};
// }

async function main(): Promise<any> {
  const config = new pulumi.Config();
  const bbAwsVpc = new BBAwsVpc(config.require('vpcId'));
  const bbAwsEc2 = await new BBAwsEc2(
    config.require('keyName'), 
    './userdata/ec2.sh',
    bbAwsVpc
  ).load();

  return {
    ami: bbAwsEc2.ami?.id
  }
}

export const output = main();