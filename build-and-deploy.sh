#!/bin/bash
# Build and deploy custom Radarr from git branch

BRANCH=${1:-telemetry-baseline}

echo "Building Radarr from branch: $BRANCH"

# Checkout the branch
git checkout $BRANCH

# Build the Docker image
docker build -t radarr-custom:$BRANCH -f Dockerfile.custom .

# Update your compose to use the custom image
cat > docker-compose.override.yml <<EOF
version: '3.8'
services:
  radarr:
    image: radarr-custom:$BRANCH
    build:
      context: .
      dockerfile: Dockerfile.custom
EOF

echo "✅ Built radarr-custom:$BRANCH"
echo ""
echo "To deploy:"
echo "  docker-compose up -d radarr"
echo ""
echo "To switch branches:"
echo "  ./build-and-deploy.sh performance-optimizations"
