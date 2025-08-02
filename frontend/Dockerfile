# ---- Build stage ----
FROM node:20-alpine AS builder
WORKDIR /web

RUN npm install -g pnpm

COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY frontend/. .
RUN pnpm run build

# ---- Serve static build ----
FROM nginx:stable-alpine

COPY --from=builder /web/dist /usr/share/nginx/html

# Ensure permissions BEFORE switching user
RUN mkdir -p /var/cache/nginx /run \
  && chown -R nginx:nginx /var/cache/nginx /usr/share/nginx/html /run

USER nginx

# Healthcheck to verify Nginx is serving content
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost || exit 1

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]

