# HashiCorp Vault Production Configuration
# ========================================

# Storage backend - Consul for production HA
storage "consul" {
  address = "consul:8500"
  path    = "vault/"
  
  # Consul token for authentication
  token   = "vault-consul-token"
  
  # TLS configuration for Consul
  scheme  = "https"
  tls_ca_file   = "/vault/tls/consul-ca.pem"
  tls_cert_file = "/vault/tls/consul-client.pem"
  tls_key_file  = "/vault/tls/consul-client-key.pem"
}

# High Availability configuration
ha_storage "consul" {
  address = "consul:8500"
  path    = "vault/"
  token   = "vault-consul-token"
  
  # Consul cluster configuration
  redirect_addr = "https://vault.talk-plus-plus.com:8200"
  cluster_addr  = "https://vault.talk-plus-plus.com:8201"
}

# Listener configuration - TLS only in production
listener "tcp" {
  address       = "0.0.0.0:8200"
  tls_disable   = 0
  
  # TLS certificate configuration
  tls_cert_file = "/vault/tls/vault-server.pem"
  tls_key_file  = "/vault/tls/vault-server-key.pem"
  
  # Client certificate authentication
  tls_client_ca_file = "/vault/tls/ca.pem"
  tls_require_and_verify_client_cert = true
  
  # TLS configuration
  tls_min_version = "tls12"
  tls_cipher_suites = "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"
  
  # Performance tuning
  tls_prefer_server_cipher_suites = true
}

# API and cluster addresses
api_addr = "https://vault.talk-plus-plus.com:8200"
cluster_addr = "https://vault.talk-plus-plus.com:8201"

# Auto-unseal using AWS KMS (or Azure Key Vault/GCP KMS)
seal "awskms" {
  region     = "us-west-2"
  kms_key_id = "vault-unseal-key"
  endpoint   = "https://kms.us-west-2.amazonaws.com"
}

# UI configuration - Enabled but restricted
ui = true

# Production logging
log_level = "INFO"
log_format = "json"
log_file = "/vault/logs/vault.log"
log_rotate_duration = "24h"
log_rotate_max_files = 30

# Security settings
disable_mlock = false
raw_storage_endpoint = false
disable_sealwrap = false

# Plugin directory
plugin_directory = "/vault/plugins"

# Lease configuration
default_lease_ttl = "1h"
max_lease_ttl = "24h"

# Performance tuning for production
default_max_request_duration = "30s"
max_request_size = "33554432"

# Rate limiting
api_rate_limit = 1000
api_rate_limit_exempt_paths = ["sys/health", "sys/seal-status"]

# Audit logging
audit_log_file = "/vault/audit/audit.log"

# Telemetry for monitoring
telemetry {
  prometheus_retention_time = "30s"
  disable_hostname = false
  
  # StatsD for metrics collection
  statsd_address = "statsd:8125"
}

# Entropy Augmentation for additional randomness
entropy "seal" {
  mode = "augmentation"
}

# Service registration for load balancing
service_registration "consul" {
  address      = "consul:8500"
  token        = "vault-consul-token"
  
  # Service configuration
  service      = "vault"
  service_tags = ["production", "secure"]
  
  # Health check configuration
  check_timeout = "5s"
  address       = "https://vault.talk-plus-plus.com:8200/v1/sys/health?standbyok=true"
} 