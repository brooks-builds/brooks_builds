import { LoadBalancer } from "@pulumi/aws/lb"
import BBAwsSecurityGroups from "./securityGroups";
import BBAwsVpc from "./vpc";

const ENABLE_DELETION_PROTECTION = false;

export default class BBAwsLoadBalancer {
  loadBalancer: LoadBalancer;
  securityGroups: BBAwsSecurityGroups;
  vpc: BBAwsVpc;

  constructor(securityGroups: BBAwsSecurityGroups, vpc: BBAwsVpc) {
    this.securityGroups = securityGroups;
    this.vpc = vpc;
    this.loadBalancer = this.createLoadBalancer(ENABLE_DELETION_PROTECTION);
  }

  createLoadBalancer(enableDeletionProtection: boolean): LoadBalancer {
    return new LoadBalancer("LoadBalancer", {
      internal: false,
      loadBalancerType: "application",
      securityGroups: this.securityGroups.getInternetToLoadBalancerSecurityGroupIds,
      subnets: this.vpc.getAllSPublicSubnetIds,
      enableDeletionProtection,
      name: 'BrooksBuildsLoadBalancer',
    })
  }
}