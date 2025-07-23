# Talk++ Production Environment Vault Policy
# ===========================================
# This policy defines RESTRICTED permissions for the Talk++ application in production

# Database secrets for production (READ ONLY)
path "secret/data/prod/database/*" {
  capabilities = ["read"]
}

# Redis secrets for production (READ ONLY)
path "secret/data/prod/redis/*" {
  capabilities = ["read"]
}

# Kubernetes configuration for production (READ ONLY)
path "secret/data/prod/k8s/*" {
  capabilities = ["read"]
}

# API keys for external services (production) - READ ONLY
path "secret/data/prod/api-keys/*" {
  capabilities = ["read"]
}

# AI/LLM API keys (production) - READ ONLY
path "secret/data/prod/ai/*" {
  capabilities = ["read"]
}

# MCP server configurations (production) - READ ONLY
path "secret/data/prod/mcp/*" {
  capabilities = ["read"]
}

# Vector database credentials (production) - READ ONLY
path "secret/data/prod/vector-db/*" {
  capabilities = ["read"]
}

# CUDA/GPU configuration (production) - READ ONLY
path "secret/data/prod/cuda/*" {
  capabilities = ["read"]
}

# Monitoring and observability (production) - READ ONLY
path "secret/data/prod/monitoring/*" {
  capabilities = ["read"]
}

# Production TLS certificates (READ ONLY)
path "secret/data/prod/tls/*" {
  capabilities = ["read"]
}

# Application configuration (LIMITED UPDATE for hot-reload)
path "secret/data/prod/config/app-settings" {
  capabilities = ["read"]
}

path "secret/data/prod/config/feature-flags" {
  capabilities = ["read", "update"]
}

# NO access to temp secrets in production
# Production should use short-lived dynamic secrets only

# Dynamic database credentials (short-lived)
path "database/creds/talk-plus-plus-prod" {
  capabilities = ["read"]
}

# PKI for service-to-service communication (short-lived certs)
path "pki_int/issue/talk-plus-plus-prod" {
  capabilities = ["create", "update"]
}

# Transit encryption for sensitive data in production
path "transit/encrypt/talk-plus-plus-prod" {
  capabilities = ["update"]
}

path "transit/decrypt/talk-plus-plus-prod" {
  capabilities = ["update"]
}

# KV version 2 metadata (for secret lifecycle management) - LIST/READ ONLY
path "secret/metadata/prod/*" {
  capabilities = ["list", "read"]
}

# Audit log access (READ ONLY for compliance)
path "sys/audit" {
  capabilities = ["read"]
}

# Health check endpoint
path "sys/health" {
  capabilities = ["read"]
} 