#!/bin/bash

echo "Building channels service..."
docker build -f channels/Dockerfile -t service-channels:latest .

echo "Building posts service..."
docker build -f posts/Dockerfile -t service-posts:latest .

echo "Building subscriptions service..."
docker build -f subscriptions/Dockerfile -t service-subscriptions:latest .

echo "Building users service..."
docker build -f users/Dockerfile -t service-users:latest .

echo "All builds completed"
