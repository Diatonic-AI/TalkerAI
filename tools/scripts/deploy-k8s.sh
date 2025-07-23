#!/bin/bash
# deploy-k8s.sh - Deploy Talk++ to Kubernetes cluster

set -euo pipefail

# Configuration
NAMESPACE="talkpp"
KUBECTL_TIMEOUT="300s"
ENVIRONMENT="${1:-development}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check kubectl
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl is not installed or not in PATH"
        exit 1
    fi
    
    # Check cluster connection
    if ! kubectl cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster"
        log_info "Please ensure your kubeconfig is set up correctly"
        exit 1
    fi
    
    # Check if Podman image exists
    if ! podman images localhost/talkpp:latest --quiet &> /dev/null; then
        log_warning "Container image localhost/talkpp:latest not found"
        log_info "Building image with Podman..."
        ./scripts/build-podman.sh
    fi
    
    log_success "Prerequisites check passed"
}

# Create namespace and basic resources
deploy_infrastructure() {
    log_info "Deploying infrastructure components..."
    
    # Create namespace
    kubectl apply -f k8s/namespace.yaml
    
    # Create ConfigMaps and Secrets
    kubectl apply -f k8s/configmap.yaml
    
    # Check if secrets exist, create template if not
    if ! kubectl get secret talkpp-secrets -n "$NAMESPACE" &> /dev/null; then
        log_warning "Secrets not found. Creating from template..."
        log_info "Please edit k8s/secrets-local.yaml with your actual values"
        
        # Create a local secrets file if it doesn't exist
        if [ ! -f "k8s/secrets-local.yaml" ]; then
            cp k8s/secrets.yaml k8s/secrets-local.yaml
            log_info "Created k8s/secrets-local.yaml - please edit with actual values"
            
            # Generate some random values
            JWT_SECRET=$(openssl rand -base64 32)
            DB_PASSWORD=$(openssl rand -base64 16)
            
            sed -i "s/CHANGE_ME_TO_RANDOM_STRING/$JWT_SECRET/g" k8s/secrets-local.yaml
            sed -i "s/database-password: \"CHANGE_ME\"/database-password: \"$DB_PASSWORD\"/g" k8s/secrets-local.yaml
            sed -i "s|postgresql://talkpp:CHANGE_ME@|postgresql://talkpp:$DB_PASSWORD@|g" k8s/secrets-local.yaml
            
            log_warning "Generated random JWT secret and database password"
            log_warning "Please edit k8s/secrets-local.yaml to add your service API keys"
        fi
        
        kubectl apply -f k8s/secrets-local.yaml
    fi
    
    log_success "Infrastructure components deployed"
}

# Deploy database and cache
deploy_data_services() {
    log_info "Deploying data services..."
    
    # Deploy PostgreSQL
    kubectl apply -f k8s/postgresql.yaml
    
    # Deploy Redis
    kubectl apply -f k8s/redis.yaml
    
    # Wait for data services to be ready
    log_info "Waiting for PostgreSQL to be ready..."
    kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=postgresql -n "$NAMESPACE" --timeout="$KUBECTL_TIMEOUT"
    
    log_info "Waiting for Redis to be ready..."
    kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=redis -n "$NAMESPACE" --timeout="$KUBECTL_TIMEOUT"
    
    log_success "Data services deployed and ready"
}

# Load container image into cluster
load_image_to_cluster() {
    log_info "Loading container image to cluster..."
    
    # For Kind cluster (if detected)
    if kubectl config current-context | grep -q "kind"; then
        log_info "Detected Kind cluster, loading image..."
        kind load docker-image localhost/talkpp:latest --name "$(kubectl config current-context | cut -d'-' -f2-)"
    fi
    
    # For k3s/k3d cluster (if detected)
    if kubectl config current-context | grep -q "k3"; then
        log_info "Detected k3s/k3d cluster, importing image..."
        podman save localhost/talkpp:latest | docker load
    fi
    
    log_success "Image loaded to cluster"
}

# Deploy main application
deploy_application() {
    log_info "Deploying Talk++ application..."
    
    # Deploy the main application
    kubectl apply -f k8s/talkpp-deployment.yaml
    
    # Wait for deployment to be ready
    log_info "Waiting for application to be ready..."
    kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=talkpp -n "$NAMESPACE" --timeout="$KUBECTL_TIMEOUT"
    
    log_success "Application deployed and ready"
}

# Setup ingress and external access
setup_ingress() {
    log_info "Setting up ingress..."
    
    # Check if ingress controller is available
    if kubectl get ingressclass nginx &> /dev/null; then
        log_info "Nginx ingress controller detected, deploying ingress..."
        kubectl apply -f k8s/ingress.yaml
        
        # Add local hosts entries
        log_info "Add these entries to your /etc/hosts file for local access:"
        echo "127.0.0.1 talkpp.local api.talkpp.local"
        
    else
        log_warning "No ingress controller detected, using NodePort service"
        log_info "Application will be available at: http://localhost:30080"
    fi
    
    log_success "Ingress configuration applied"
}

# Display deployment status
show_status() {
    log_info "Deployment Status:"
    echo "===================="
    
    # Show pods
    echo "Pods:"
    kubectl get pods -n "$NAMESPACE" -o wide
    echo ""
    
    # Show services
    echo "Services:"
    kubectl get services -n "$NAMESPACE" -o wide
    echo ""
    
    # Show ingress
    echo "Ingress:"
    kubectl get ingress -n "$NAMESPACE" -o wide 2>/dev/null || echo "No ingress configured"
    echo ""
    
    # Show persistent volumes
    echo "Persistent Volumes:"
    kubectl get pvc -n "$NAMESPACE"
    echo ""
    
    # Show application logs (last 10 lines)
    echo "Recent Application Logs:"
    kubectl logs -l app.kubernetes.io/name=talkpp -n "$NAMESPACE" --tail=10 2>/dev/null || echo "No logs available yet"
}

# Health check
health_check() {
    log_info "Performing health check..."
    
    # Port forward for health check
    kubectl port-forward service/talkpp-api 8080:8080 -n "$NAMESPACE" &
    PORT_FORWARD_PID=$!
    
    # Wait a moment for port forward to establish
    sleep 5
    
    # Health check
    if curl -f http://localhost:8080/health 2>/dev/null; then
        log_success "Health check passed"
    else
        log_warning "Health check failed - application may still be starting"
    fi
    
    # Kill port forward
    kill $PORT_FORWARD_PID 2>/dev/null || true
}

# Main deployment process
main() {
    log_info "Starting Talk++ deployment to Kubernetes ($ENVIRONMENT environment)"
    log_info "Target namespace: $NAMESPACE"
    
    check_prerequisites
    deploy_infrastructure
    deploy_data_services
    load_image_to_cluster
    deploy_application
    setup_ingress
    
    log_success "Deployment completed successfully!"
    
    show_status
    health_check
    
    log_info "Useful commands:"
    log_info "  View logs: kubectl logs -f deployment/talkpp-api -n $NAMESPACE"
    log_info "  Port forward: kubectl port-forward service/talkpp-api 8080:8080 -n $NAMESPACE"
    log_info "  Shell into pod: kubectl exec -it deployment/talkpp-api -n $NAMESPACE -- /bin/sh"
    log_info "  Delete deployment: kubectl delete namespace $NAMESPACE"
}

# Handle script arguments
case "${1:-deploy}" in
    "deploy")
        main
        ;;
    "status")
        show_status
        ;;
    "health")
        health_check
        ;;
    "clean")
        log_warning "Cleaning up Talk++ deployment..."
        kubectl delete namespace "$NAMESPACE" --ignore-not-found
        log_success "Cleanup completed"
        ;;
    *)
        echo "Usage: $0 [deploy|status|health|clean]"
        echo "  deploy: Deploy Talk++ to Kubernetes (default)"
        echo "  status: Show deployment status"
        echo "  health: Run health check"
        echo "  clean: Remove all Talk++ resources"
        exit 1
        ;;
esac 