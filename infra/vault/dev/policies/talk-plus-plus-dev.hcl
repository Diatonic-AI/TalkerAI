# Talk++ Development Environment Vault Policy
# ============================================
# This policy defines permissions for the Talk++ application in development

# Database secrets for development
path "secret/data/dev/database/*" {
  capabilities = ["read"]
}

# Redis secrets for development
path "secret/data/dev/redis/*" {
  capabilities = ["read"]
}

# Kubernetes configuration for development
path "secret/data/dev/k8s/*" {
  capabilities = ["read"]
}

# API keys for external services (development)
path "secret/data/dev/api-keys/*" {
  capabilities = ["read"]
}

# AI/LLM API keys (development)
path "secret/data/dev/ai/*" {
  capabilities = ["read"]
}

# MCP server configurations
path "secret/data/dev/mcp/*" {
  capabilities = ["read"]
}

# Vector database credentials
path "secret/data/dev/vector-db/*" {
  capabilities = ["read"]
}

# CUDA/GPU configuration
path "secret/data/dev/cuda/*" {
  capabilities = ["read"]
}

# Monitoring and observability
path "secret/data/dev/monitoring/*" {
  capabilities = ["read"]
}

# Development TLS certificates
path "secret/data/dev/tls/*" {
  capabilities = ["read"]
}

# Application configuration
path "secret/data/dev/config/*" {
  capabilities = ["read", "update"]
}

# Temporary secrets (short-lived)
path "secret/data/dev/temp/*" {
  capabilities = ["create", "read", "update", "delete"]
}

# Dynamic database credentials
path "database/creds/talk-plus-plus-dev" {
  capabilities = ["read"]
}

# PKI for service-to-service communication
path "pki_int/issue/talk-plus-plus-dev" {
  capabilities = ["create", "update"]
}

# Transit encryption for sensitive data
path "transit/encrypt/talk-plus-plus-dev" {
  capabilities = ["update"]
}

path "transit/decrypt/talk-plus-plus-dev" {
  capabilities = ["update"]
}

# KV version 2 metadata (for secret lifecycle management)
path "secret/metadata/dev/*" {
  capabilities = ["list", "read"]
} 