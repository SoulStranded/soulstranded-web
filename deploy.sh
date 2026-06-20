#!/bin/bash
set -e

username=$1
password=$2

PROJECT_DIR=$(pwd)

echo "Building backend..."
cd $PROJECT_DIR/soulstranded-backend
cargo build --release

echo "Building frontend..."
cd $PROJECT_DIR/soulstranded-frontend
pnpm run build

echo "Deploying backend..."
sshpass -p $password ssh -o StrictHostKeyChecking=no $username@101.201.31.56 "rm -rf /opt/soulstranded-backend/soulstranded-backend"
sshpass -p $password scp $PROJECT_DIR/soulstranded-backend/target/release/soulstranded-backend $username@101.201.31.56:/opt/soulstranded-backend/soulstranded-backend
sshpass -p $password ssh -o StrictHostKeyChecking=no $username@101.201.31.56 "systemctl restart soulstranded-backend"

echo "Deploying frontend..."
sshpass -p $password ssh -o StrictHostKeyChecking=no $username@101.201.31.56 "rm -rf /var/www/soulstranded-frontend"
sshpass -p $password scp -r $PROJECT_DIR/soulstranded-frontend/dist $username@101.201.31.56:/var/www/soulstranded-frontend
sshpass -p $password ssh -o StrictHostKeyChecking=no $username@101.201.31.56 "systemctl restart nginx"

echo "Deployment complete."
