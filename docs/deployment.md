# Deployment

Modunote is deployed on AWS. Infrastructure is defined as code in the [`cdk/`](../cdk/) directory using [AWS CDK v2](https://docs.aws.amazon.com/cdk/v2/guide/home.html) (TypeScript).

## Architecture Overview

```
                        ┌─────────────────────────────────────────────┐
                        │                    AWS                       │
                        │                                              │
Users ──► CloudFront ──► S3 (static assets)                          │
                        │                                              │
              │         │   App Runner ──► RDS (PostgreSQL)           │
              └─────────►  (REST API :8080)                           │
                        │                                              │
                        └─────────────────────────────────────────────┘
```

### Frontend — S3 + CloudFront

| Resource     | Purpose                                                        |
|--------------|----------------------------------------------------------------|
| S3 Bucket    | Stores the compiled static assets (`pnpm build` output)        |
| CloudFront   | CDN — HTTPS termination, caching, and global edge delivery     |

The React + Vite app is built to a set of static files and uploaded to S3. CloudFront sits in front of it to serve assets over HTTPS with low latency worldwide and handle SPA routing (all paths return `index.html`).

### Backend — App Runner + RDS

| Resource      | Purpose                                                        |
|---------------|----------------------------------------------------------------|
| App Runner    | Runs the containerised Rust/Axum API; auto-scales, no VMs to manage |
| ECR           | Stores Docker images built from `backend/Dockerfile`           |
| RDS           | Managed PostgreSQL instance (used by `storage_postgres` crate) |
| Secrets Manager | Stores the RDS credentials, injected into App Runner at runtime |

App Runner pulls the image from ECR and routes HTTPS traffic to port `8080` (as declared in `backend/Dockerfile`). RDS is placed in a private subnet and is only reachable from App Runner via a VPC connector.

## CDK Stack (`cdk/`)

All AWS resources are managed by the CDK stack defined in `cdk/lib/cdk-stack.ts` and bootstrapped via `cdk/bin/cdk.ts`.

```
cdk/
├── bin/
│   └── cdk.ts          # App entry point — instantiates CdkStack
├── lib/
│   └── cdk-stack.ts    # All resource definitions (App Runner, RDS, S3, CloudFront …)
└── cdk.json            # CDK toolkit config & feature flags
```

### Prerequisites

- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) configured with appropriate credentials
- [Node.js](https://nodejs.org/) and npm
- CDK CLI: `npm install -g aws-cdk`

### First-time bootstrap

CDK requires a one-time bootstrap per account/region to create the S3 bucket and ECR repo it needs internally:

```bash
cd cdk
npm install
npx cdk bootstrap aws://<ACCOUNT_ID>/<REGION>
```

### Deploy

```bash
# From cdk/
npm run build        # compile TypeScript → JavaScript
npx cdk diff         # preview changes against the live stack
npx cdk deploy       # create / update all AWS resources
```

CDK will print the CloudFront distribution URL and the App Runner service URL as stack outputs after a successful deploy.

### Destroy

```bash
npx cdk destroy      # tears down all resources in the stack
```

> **Warning:** This will delete the RDS instance and all data. Take a snapshot first if needed.

## Deploying a New Backend Version

1. Build and push a new Docker image to ECR:
   ```bash
   # From backend/
   aws ecr get-login-password --region <REGION> | \
     docker login --username AWS --password-stdin <ACCOUNT_ID>.dkr.ecr.<REGION>.amazonaws.com

   docker build -t modunote-api .
   docker tag modunote-api:latest <ACCOUNT_ID>.dkr.ecr.<REGION>.amazonaws.com/modunote-api:latest
   docker push <ACCOUNT_ID>.dkr.ecr.<REGION>.amazonaws.com/modunote-api:latest
   ```
2. App Runner detects the new image and deploys it automatically (or trigger manually from the console / CLI).

## Deploying a New Frontend Version

1. Build the static assets:
   ```bash
   # From frontend/
   pnpm build
   ```
2. Sync the `dist/` output to S3 and invalidate the CloudFront cache:
   ```bash
   aws s3 sync dist/ s3://<BUCKET_NAME>/ --delete
   aws cloudfront create-invalidation --distribution-id <DIST_ID> --paths "/*"
   ```

## Environment Variables

### Backend (`backend/.env`)

| Variable                    | Where set (prod)   | Description                                      |
|-----------------------------|--------------------|--------------------------------------------------|
| `APP_ENV`                   | App Runner config  | Runtime environment (`dev` / `prod`)             |
| `DATABASE_URL`              | Secrets Manager    | PostgreSQL connection string for RDS             |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | App Runner config | OTLP endpoint for OpenTelemetry trace export   |
| `SQLX_OFFLINE`              | App Runner config  | Set to `true` to use cached sqlx query metadata  |

### Frontend (`frontend/.env`)

| Variable        | Where set (prod)           | Description                              |
|-----------------|----------------------------|------------------------------------------|
| `VITE_API_URL`  | Build-time (Vite)          | Base URL of the App Runner API service   |

`VITE_API_URL` is baked into the static bundle at build time. Set it to the App Runner HTTPS service URL before running `pnpm build` for a production deployment.
