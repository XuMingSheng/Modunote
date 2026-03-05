import * as cdk from "aws-cdk-lib/core";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as rds from "aws-cdk-lib/aws-rds";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as apprunner from "aws-cdk-lib/aws-apprunner";
import * as ecr_assets from "aws-cdk-lib/aws-ecr-assets";
import * as iam from "aws-cdk-lib/aws-iam";
import { Construct } from "constructs";
import * as path from "path";

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const prefix = this.stackName;
    const lowerPrefix = prefix.toLowerCase();

    // ── VPC ──────────────────
    const vpc = new ec2.Vpc(this, `${prefix}Vpc`, {
      maxAzs: 2,
      natGateways: 0, // Cost-saving for small apps; uses isolated subnets
      subnetConfiguration: [
        {
          name: "Isolated",
          subnetType: ec2.SubnetType.PRIVATE_ISOLATED,
          cidrMask: 24,
        },
      ],
    });

    // ── Docker Image Asset  ──────────────────────────
    // This replaces manual ECR repository management.
    // CDK will build the Dockerfile in ../backend/ and push it to a
    // CDK-managed staging repository automatically.
    const backendAsset = new ecr_assets.DockerImageAsset(
      this,
      `${prefix}BackendAsset`,
      {
        directory: path.join(__dirname, "../../backend"),
      },
    );

    // ── Security groups ───────────────────────────────────────────────────
    const appRunnerSg = new ec2.SecurityGroup(this, `${prefix}AppRunnerSg`, {
      vpc,
      description: `${prefix} AppRunner Connector SG`,
      allowAllOutbound: true,
    });

    const dbSg = new ec2.SecurityGroup(this, `${prefix}DbSg`, {
      vpc,
      description: `${prefix} RDS Database SG`,
      allowAllOutbound: false,
    });
    dbSg.addIngressRule(
      appRunnerSg,
      ec2.Port.tcp(5432),
      "AllowAppRunnerPostgres",
    );

    // ── RDS PostgreSQL ────────────────────────────────────────────────────
    const db = new rds.DatabaseInstance(this, `${prefix}DbInstance`, {
      engine: rds.DatabaseInstanceEngine.postgres({
        version: rds.PostgresEngineVersion.VER_16,
      }),
      instanceType: ec2.InstanceType.of(
        ec2.InstanceClass.T3,
        ec2.InstanceSize.MICRO,
      ),
      credentials: rds.Credentials.fromGeneratedSecret(`${lowerPrefix}_admin`),
      vpc,
      vpcSubnets: { subnetType: ec2.SubnetType.PRIVATE_ISOLATED },
      securityGroups: [dbSg],
      databaseName: "modunote",
      // Use DESTROY for dev/learning, RETAIN for production
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      deletionProtection: false,
    });

    // ── IAM roles for App Runner ──────────────────────────────────────────
    // The Access Role allows App Runner to pull the image from the CDK staging ECR
    const accessRole = new iam.Role(this, `${prefix}AppRunnerAccessRole`, {
      assumedBy: new iam.ServicePrincipal("build.apprunner.amazonaws.com"),
    });
    // Grant the role permission to pull the specific asset image
    backendAsset.repository.grantPull(accessRole);

    const instanceRole = new iam.Role(this, "AppRunnerInstanceRole", {
      assumedBy: new iam.ServicePrincipal("tasks.apprunner.amazonaws.com"),
    });
    db.secret!.grantRead(instanceRole);
    // Allow App Runner to decrypt the secret
    if (db.secret!.encryptionKey) {
      db.secret!.encryptionKey.grantDecrypt(instanceRole);
    }

    // ── App Runner VPC connector ──────────────────────────────────────────
    const vpcConnector = new apprunner.CfnVpcConnector(this, "VpcConnector", {
      subnets: vpc.isolatedSubnets.map((s) => s.subnetId),
      securityGroups: [appRunnerSg.securityGroupId],
    });

    // ── App Runner service (L1 to avoid alpha package dependency) ─────────
    const appRunnerService = new apprunner.CfnService(
      this,
      `${prefix}ApiService`,
      {
        serviceName: `${lowerPrefix}-api`,
        sourceConfiguration: {
          authenticationConfiguration: {
            accessRoleArn: accessRole.roleArn,
          },
          autoDeploymentsEnabled: false, // Usually set to true only for code-based services
          imageRepository: {
            imageIdentifier: backendAsset.imageUri, // Uses the auto-built Asset URI
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
      },
    );

    // Ensure App Runner doesn't try to create until the image is definitely built/pushed
    appRunnerService.node.addDependency(backendAsset);

    // ── S3 bucket for frontend ────────────────────────────────────────────
    const frontendBucket = new s3.Bucket(this, `${prefix}FrontendBucket`, {
      bucketName: `${lowerPrefix}-frontend-${this.account}`,
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
    new cdk.CfnOutput(this, "FrontendBucketName", {
      value: frontendBucket.bucketName,
    });
    new cdk.CfnOutput(this, "CloudFrontDistributionId", {
      value: distribution.distributionId,
    });
  }
}
