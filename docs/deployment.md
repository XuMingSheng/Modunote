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
| ECR           | CDK-managed staging repo; images are built and pushed automatically by `cdk deploy` via `DockerImageAsset` |
| RDS           | Managed PostgreSQL instance (used by `storage_postgres` crate) |
| Secrets Manager | Stores the RDS credentials (`DB_USER`, `DB_PASSWORD`), injected into App Runner at runtime |

App Runner pulls the image from the CDK-managed ECR repo and routes HTTPS traffic to port `8080` (as declared in `backend/Dockerfile`). RDS is placed in a private subnet and is only reachable from App Runner via a VPC connector.

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

### First-time bootstrap

CDK requires a one-time bootstrap per account/region to create the S3 bucket and ECR repo it needs internally.

> **Important:** Run the bootstrap from [AWS CloudShell](https://console.aws.amazon.com/cloudshell) — not locally. Running it locally may fail for environment-specific reasons.

In AWS CloudShell:

```bash
npx cdk bootstrap
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

Run `cdk deploy` from the `cdk/` directory. CDK builds the Docker image from `backend/` via `DockerImageAsset`, pushes it to the CDK-managed ECR repo, and updates the App Runner service in one step:

```bash
# From cdk/
npx cdk deploy --require-approval never
```

> **Note:** `autoDeploymentsEnabled` is set to `false` on the App Runner service — it will **not** pick up new images automatically. A `cdk deploy` is required for every backend update.

## Deploying a New Frontend Version

1. Install dependencies and build the static assets:
   ```bash
   # From frontend/
   pnpm install --frozen-lockfile
   VITE_API_URL=<APP_RUNNER_URL> pnpm build
   ```
2. Sync the `dist/` output to S3 and invalidate the CloudFront cache:
   ```bash
   aws s3 sync dist/ s3://<BUCKET_NAME>/ --delete
   aws cloudfront create-invalidation --distribution-id <DIST_ID> --paths "/*"
   ```

## Environment Variables

### Backend

In cloud, all variables are injected by App Runner at runtime (configured in the CDK stack). For local dev, set them in `backend/.env`.

| Variable                      | Where set (cloud)       | Description                                                        |
|-------------------------------|-------------------------|--------------------------------------------------------------------|
| `APP_ENV`                     | App Runner env var      | Runtime environment (`dev` / `cloud`)                              |
| `FRONTEND_URL`                | App Runner env var      | Allowed CORS origin (the CloudFront URL)                           |
| `DB_HOST`                     | App Runner env var      | RDS hostname                                                       |
| `DB_NAME`                     | App Runner env var      | RDS database name                                                  |
| `DB_USER`                     | Secrets Manager         | RDS username                                                       |
| `DB_PASSWORD`                 | Secrets Manager         | RDS password                                                       |
| `SQLX_OFFLINE`                | App Runner env var      | Set to `true` to use cached sqlx query metadata                    |
| `RUST_LOG_LEVEL`              | App Runner env var      | Log level (e.g. `info`, `debug`)                                   |
| `OTEL_ENABLED`                | App Runner env var      | Set to `true` to enable OpenTelemetry tracing                      |
| `OTEL_SERVICE_NAME`           | App Runner env var      | Service name reported to the OTLP collector (requires `OTEL_ENABLED=true`) |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | App Runner env var      | OTLP collector endpoint (requires `OTEL_ENABLED=true`)             |

> **Note:** In `dev` mode, `DATABASE_URL` is used directly instead of the individual `DB_*` variables.

### Frontend (`frontend/.env`)

| Variable        | Where set (cloud)          | Description                              |
|-----------------|----------------------------|------------------------------------------|
| `VITE_API_URL`  | Build-time (Vite)          | Base URL of the App Runner API service   |

`VITE_API_URL` is baked into the static bundle at build time. Set it to the App Runner HTTPS service URL before running `pnpm build` for a production deployment.
