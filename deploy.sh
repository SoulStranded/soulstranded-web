#!/bin/bash
set -e

# Usage: ./deploy.sh [target]
#   target: all | frontend | backend   (default: all)
#
# Authentication is handled by SSH keys. Configure a host alias named
# "soulstranded" in ~/.ssh/config pointing at the server (see README.md).

target=${1:-all}

case "$target" in
    all|frontend|backend) ;;
    *)
        echo "Invalid target: $target (expected: all|frontend|backend)"
        exit 1
        ;;
esac

PROJECT_DIR=$(pwd)
HOST="soulstranded"

build_backend() {
    echo "Building backend..."
    (cd "$PROJECT_DIR/soulstranded-backend" && cargo build --release)
}

build_frontend() {
    echo "Building frontend..."
    (cd "$PROJECT_DIR/soulstranded-frontend" && pnpm run build)
}

deploy_backend() {
    echo "Deploying backend..."
    ssh "$HOST" "rm -rf /opt/soulstranded-backend/soulstranded-backend"
    scp "$PROJECT_DIR/soulstranded-backend/target/release/soulstranded-backend" "$HOST:/opt/soulstranded-backend/soulstranded-backend"
    ssh "$HOST" "systemctl restart soulstranded-backend"
}

deploy_frontend() {
    echo "Deploying frontend..."
    ssh "$HOST" "rm -rf /var/www/soulstranded-frontend"
    scp -r "$PROJECT_DIR/soulstranded-frontend/dist" "$HOST:/var/www/soulstranded-frontend"
    ssh "$HOST" "systemctl restart nginx"
}

case "$target" in
    backend)
        build_backend
        deploy_backend
        ;;
    frontend)
        build_frontend
        deploy_frontend
        ;;
    all)
        build_backend
        build_frontend
        deploy_backend
        deploy_frontend
        ;;
esac

echo "Deployment complete ($target)."
