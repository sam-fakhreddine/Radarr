# Deploying Custom Radarr Branches to Docker

## Quick Start

```bash
# Build PR #1 (telemetry baseline)
./build-and-deploy.sh telemetry-baseline

# Deploy
docker-compose up -d radarr

# After 2 weeks, switch to PR #2
./build-and-deploy.sh performance-optimizations
docker-compose up -d radarr
```

## Manual Build

```bash
# Checkout branch
git checkout telemetry-baseline

# Build image
docker build -t radarr-custom:telemetry -f Dockerfile.custom .

# Update your compose
docker-compose up -d radarr
```

## Your Compose File

Update your existing compose to use custom image:

```yaml
radarr:
    image: radarr-custom:telemetry  # <-- Change this
    container_name: radarr
    restart: always
    ports:
        - 7878:7878
    volumes:
        - /mediapool/docker/docker/plex/radarr/config:/config
        - mediapool:/media
        - usenetsab:/downloads
    environment:
        PUID: 1000
        PGID: 1000
        TZ: ${timezone}
    deploy:
        resources:
            limits:
                cpus: "2"
                memory: 512m
    entrypoint: /bin/sh -c "mkdir -p /mnt/bigshare; ln -s /media/Movies /mnt/bigshare || true; exec /init"
```

## Testing Timeline

### Weeks 1-2: Baseline
```bash
# Build and deploy telemetry branch
./build-and-deploy.sh telemetry-baseline
docker-compose up -d radarr

# Collect metrics
cp /mediapool/docker/docker/plex/radarr/config/logs/performance-metrics.json baseline-week1.json
# Wait 1 week
cp /mediapool/docker/docker/plex/radarr/config/logs/performance-metrics.json baseline-week2.json
```

### Weeks 3-4: Optimized
```bash
# Build and deploy optimizations branch
./build-and-deploy.sh performance-optimizations
docker-compose up -d radarr

# Collect metrics
cp /mediapool/docker/docker/plex/radarr/config/logs/performance-metrics.json optimized-week1.json
# Wait 1 week
cp /mediapool/docker/docker/plex/radarr/config/logs/performance-metrics.json optimized-week2.json
```

### Compare Results
```bash
jq -s '{
  baseline_memory: .[0].process.memory_mb,
  optimized_memory: .[1].process.memory_mb,
  improvement_mb: (.[0].process.memory_mb - .[1].process.memory_mb),
  improvement_percent: ((.[0].process.memory_mb - .[1].process.memory_mb) / .[0].process.memory_mb * 100)
}' baseline-week2.json optimized-week2.json
```

## Metrics Location

Metrics are automatically written to:
```
/mediapool/docker/docker/plex/radarr/config/logs/performance-metrics.json
```

Updated every 5 minutes with:
- API response times
- Database query performance
- Connection pool stats
- Memory usage
- Thread counts

## Rollback

To go back to official LinuxServer image:
```yaml
radarr:
    image: lscr.io/linuxserver/radarr:latest  # <-- Back to official
```

Then:
```bash
docker-compose up -d radarr
```
