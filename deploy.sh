#!/bin/bash

PROJECT_DIR=$(pwd)

echo "Start Updating the project..."
git pull

echo "Start building the backend..."
. "$HOME/.cargo/env"
cd $PROJECT_DIR/soulstranded-backend
cargo build --release

echo "Start building the frontend..."
cd $PROJECT_DIR/soulstranded-frontend
pnpm install --frozen-lockfile
pnpm run build

echo "Start deploying the backend..."
systemctl stop soulstranded-backend
rm /opt/soulstranded-backend/soulstranded-backend
cp $PROJECT_DIR/soulstranded-backend/target/release/soulstranded-backend /opt/soulstranded-backend/
systemctl daemon-reload
systemctl restart soulstranded-backend

echo "Start deploying the frontend..."
rm -rf /var/www/soulstranded-frontend
cp -r $PROJECT_DIR/soulstranded-frontend/dist /var/www/soulstranded-frontend
systemctl restart nginx

echo "Deployment completed successfully!"
