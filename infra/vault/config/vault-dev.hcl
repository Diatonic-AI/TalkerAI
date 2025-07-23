# HashiCorp Vault Development Configuration
# =========================================

# Storage backend - File storage for development
storage "file" {
  path = "/vault/data"
}

# Listener configuration - HTTP for development (TLS in production)
listener "tcp" {
  address     = "0.0.0.0:8200"
  tls_disable = 1
}

# API address
api_addr = "http://0.0.0.0:8200"

# Cluster address for HA (development single node)
cluster_addr = "http://0.0.0.0:8201"

# UI configuration
ui = true

# Development mode settings
log_level = "DEBUG"
log_format = "json"

# Disable mlock for development (don't use in production)
disable_mlock = true

# Plugin directory
plugin_directory = "/vault/plugins"

# Default lease TTL
default_lease_ttl = "768h"
max_lease_ttl = "8760h"

# Performance tuning for development
default_max_request_duration = "90s"
max_request_size = "33554432"

# Entropy configuration for development
entropy "seal" {
  mode = "augmentation"
} 