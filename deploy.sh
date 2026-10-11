#!/bin/bash
set -e

# Usage: ./deploy.sh <username> <password> [target]
#   target: all | frontend | backend   (default: all)

username=$1
password=$2
target=${3:-all}

if [ -z "$username" ] || [ -z "$password" ]; then
    echo "Usage: $0 <username> <password> [all|frontend|backend]"
    exit 1
fi

case "$target" in
    all|frontend|backend) ;;
    *)
        echo "Invalid target: $target (expected: all|frontend|backend)"
        exit 1
        ;;
esac

PROJECT_DIR=$(pwd)
HOST="101.201.31.56"
SSH_OPTS="-o StrictHostKeyChecking=no"

build_backend() {
    echo "Building backend..."
    cd "$PROJECT_DIR/soulstranded-backend"
    cargo build --release
}

build_frontend() {
    echo "Building frontend..."
    cd "$PROJECT_DIR/soulstranded-frontend"
    pnpm run build
}

deploy_backend() {
    echo "Deploying backend..."
    sshpass -p "$password" ssh $SSH_OPTS "$username@$HOST" "rm -rf /opt/soulstranded-backend/soulstranded-backend"
    sshpass -p "$password" scp "$PROJECT_DIR/soulstranded-backend/target/release/soulstranded-backend" "$username@$HOST:/opt/soulstranded-backend/soulstranded-backend"
    sshpass -p "$password" ssh $SSH_OPTS "$username@$HOST" "systemctl restart soulstranded-backend"
}

deploy_frontend() {
    echo "Deploying frontend..."
    sshpass -p "$password" ssh $SSH_OPTS "$username@$HOST" "rm -rf /var/www/soulstranded-frontend"
    sshpass -p "$password" scp -r "$PROJECT_DIR/soulstranded-frontend/dist" "$username@$HOST:/var/www/soulstranded-frontend"
    sshpass -p "$password" ssh $SSH_OPTS "$username@$HOST" "systemctl restart nginx"
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
