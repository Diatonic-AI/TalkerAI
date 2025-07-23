#!/bin/bash
# build-podman.sh - Build Talk++ container image with Podman

set -euo pipefail

# Configuration
IMAGE_NAME="talkpp"
IMAGE_TAG="${1:-latest}"
FULL_IMAGE="${IMAGE_NAME}:${IMAGE_TAG}"
CONTAINERFILE="docker/Containerfile"

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

# Check if Podman is available
if ! command -v podman &> /dev/null; then
    log_error "Podman is not installed or not in PATH"
    log_info "Please install Podman Desktop or Podman CLI"
    exit 1
fi

# Check if Containerfile exists
if [ ! -f "$CONTAINERFILE" ]; then
    log_error "Containerfile not found at $CONTAINERFILE"
    exit 1
fi

log_info "Building Talk++ container image..."
log_info "Image: $FULL_IMAGE"
log_info "Containerfile: $CONTAINERFILE"

# Build the container image
log_info "Starting Podman build..."
if podman build \
    --file "$CONTAINERFILE" \
    --tag "$FULL_IMAGE" \
    --layers \
    --format docker \
    --pull=newer \
    --progress=plain \
    . ; then
    
    log_success "Container image built successfully: $FULL_IMAGE"
else
    log_error "Container build failed"
    exit 1
fi

# Display image information
log_info "Container image details:"
podman images "$FULL_IMAGE" --format "table {{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.Size}}\t{{.Created}}"

# Test the container
log_info "Testing container startup..."
if podman run --rm "$FULL_IMAGE" talkppc info; then
    log_success "Container test passed"
else
    log_warning "Container test failed - image built but may have runtime issues"
fi

# Optional: Push to local registry or save as tar
read -p "Do you want to save the image as a tar file? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    TAR_FILE="${IMAGE_NAME}-${IMAGE_TAG}.tar"
    log_info "Saving image to $TAR_FILE..."
    podman save -o "$TAR_FILE" "$FULL_IMAGE"
    log_success "Image saved to $TAR_FILE"
fi

log_success "Build process completed!"
log_info "To run the container:"
log_info "  podman run --rm -p 8080:8080 $FULL_IMAGE"
log_info ""
log_info "To deploy to Kubernetes:"
log_info "  kubectl apply -f k8s/"
log_info ""
log_info "To load in Podman Desktop, the image is now available in your local registry." 