import * as cdk from "aws-cdk-lib/core";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as rds from "aws-cdk-lib/aws-rds";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as apprunner from "aws-cdk-lib/aws-apprunner";
import * as iam from "aws-cdk-lib/aws-iam";
import { Construct } from "constructs";

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // ── VPC (no NAT gateway) ──────────────────
    const vpc = new ec2.Vpc(this, "Vpc", {
      maxAzs: 2,
      natGateways: 0,
      subnetConfiguration: [
        {
          name: "Isolated",
          subnetType: ec2.SubnetType.PRIVATE_ISOLATED,
          cidrMask: 24,
        },
      ],
    });

    // ── ECR repository ────────────────────────────────────────────────────
    const ecrRepo = new ecr.Repository(this, "ApiRepository", {
      repositoryName: "modunote-api",
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    // ── Security groups ───────────────────────────────────────────────────
    const appRunnerSg = new ec2.SecurityGroup(this, "AppRunnerSg", {
      vpc,
      description: "Attached to App Runner VPC connector",
      allowAllOutbound: true,
    });

    const dbSg = new ec2.SecurityGroup(this, "DbSg", {
      vpc,
      description: "RDS PostgreSQL",
      allowAllOutbound: false,
    });
    dbSg.addIngressRule(appRunnerSg, ec2.Port.tcp(5432), "App Runner -> RDS");

    // ── RDS PostgreSQL ────────────────────────────────────────────────────
    const db = new rds.DatabaseInstance(this, "Db", {
      engine: rds.DatabaseInstanceEngine.postgres({
        version: rds.PostgresEngineVersion.VER_16,
      }),
      instanceType: ec2.InstanceType.of(
        ec2.InstanceClass.T3,
        ec2.InstanceSize.MICRO,
      ),
      credentials: rds.Credentials.fromGeneratedSecret("modunote"),
      vpc,
      vpcSubnets: { subnetType: ec2.SubnetType.PRIVATE_ISOLATED },
      securityGroups: [dbSg],
      databaseName: "modunote",
      removalPolicy: cdk.RemovalPolicy.SNAPSHOT,
      deletionProtection: true,
    });

    // ── IAM roles for App Runner ──────────────────────────────────────────
    const accessRole = new iam.Role(this, "AppRunnerAccessRole", {
      assumedBy: new iam.ServicePrincipal("build.apprunner.amazonaws.com"),
      managedPolicies: [
        iam.ManagedPolicy.fromAwsManagedPolicyName(
          "service-role/AWSAppRunnerServicePolicyForECRAccess",
        ),
      ],
    });

    const instanceRole = new iam.Role(this, "AppRunnerInstanceRole", {
      assumedBy: new iam.ServicePrincipal("tasks.apprunner.amazonaws.com"),
    });
    db.secret!.grantRead(instanceRole);
    // Allow App Runner to decrypt the secret
    db.secret!.encryptionKey?.grantDecrypt(instanceRole);

    // ── App Runner VPC connector ──────────────────────────────────────────
    const isolatedSubnetIds = vpc.isolatedSubnets.map((s) => s.subnetId);

    const vpcConnector = new apprunner.CfnVpcConnector(this, "VpcConnector", {
      subnets: isolatedSubnetIds,
      securityGroups: [appRunnerSg.securityGroupId],
    });

    // ── App Runner service (L1 to avoid alpha package dependency) ─────────
    const appRunnerService = new apprunner.CfnService(this, "ApiService", {
      serviceName: "modunote-api",
      sourceConfiguration: {
        authenticationConfiguration: {
          accessRoleArn: accessRole.roleArn,
        },
        autoDeploymentsEnabled: true,
        imageRepository: {
          imageIdentifier: `${ecrRepo.repositoryUri}:latest`,
          imageRepositoryType: "ECR",
          imageConfiguration: {
            port: "8080",
            runtimeEnvironmentVariables: [
              { name: "APP_ENV", value: "cloud" },
              { name: "DB_HOST", value: db.dbInstanceEndpointAddress },
              { name: "DB_PORT", value: db.dbInstanceEndpointPort },
              { name: "DB_NAME", value: "modunote" },
            ],
            runtimeEnvironmentSecrets: [
              {
                name: "DB_USER",
                value: `${db.secret!.secretArn}:username::`,
              },
              {
                name: "DB_PASSWORD",
                value: `${db.secret!.secretArn}:password::`,
              },
            ],
          },
        },
      },
      instanceConfiguration: {
        instanceRoleArn: instanceRole.roleArn,
        cpu: "1 vCPU",
        memory: "2 GB",
      },
      networkConfiguration: {
        egressConfiguration: {
          egressType: "VPC",
          vpcConnectorArn: vpcConnector.attrVpcConnectorArn,
        },
      },
    });

    // ── S3 bucket for frontend ────────────────────────────────────────────
    const frontendBucket = new s3.Bucket(this, "FrontendBucket", {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    // ── CloudFront distribution ───────────────────────────────────────────
    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(frontendBucket),
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
      },
      defaultRootObject: "index.html",
      errorResponses: [
        {
          httpStatus: 403,
          responseHttpStatus: 200,
          responsePagePath: "/index.html",
        },
        {
          httpStatus: 404,
          responseHttpStatus: 200,
          responsePagePath: "/index.html",
        },
      ],
    });

    // ── Stack outputs ─────────────────────────────────────────────────────
    new cdk.CfnOutput(this, "FrontendUrl", {
      value: `https://${distribution.distributionDomainName}`,
    });
    new cdk.CfnOutput(this, "ApiUrl", {
      value: `https://${appRunnerService.attrServiceUrl}`,
    });
    new cdk.CfnOutput(this, "EcrRepositoryUri", {
      value: ecrRepo.repositoryUri,
    });
    new cdk.CfnOutput(this, "FrontendBucketName", {
      value: frontendBucket.bucketName,
    });
    new cdk.CfnOutput(this, "CloudFrontDistributionId", {
      value: distribution.distributionId,
    });
  }
}
