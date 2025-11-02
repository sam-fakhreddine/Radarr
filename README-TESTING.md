# Testing Radarr with Live Data

## Quick Start

```bash
./test-with-livedata.sh
```

Then access: http://localhost:7878

## What's Configured

- **Data Directory**: `liveData/config/`
- **Database**: Restored from backup (v5.28.0)
- **Movies**: All your real movies loaded
- **Settings**: Quality profiles, custom formats, indexers, download clients

## Development Workflow

1. **Make code changes** in `src/` or `frontend/src/`
2. **Rebuild if needed**:
   - Backend: Already built in `_output/net8.0/`
   - Frontend: `cd frontend && yarn build`
3. **Restart**: Stop (Ctrl+C) and run `./test-with-livedata.sh`

## Troubleshooting

### Frontend shows 404
```bash
cd frontend && yarn build
```

### Database corruption
```bash
cd liveData/config
rm -f radarr.db-shm radarr.db-wal
unzip -o Backups/scheduled/radarr_backup_*.zip radarr.db
```

### StyleCop errors during build
Use pre-built binary or disable StyleCop:
```bash
dotnet build -p:EnforceCodeStyleInBuild=false
```

## Notes

- Download client (qBittorrent) won't connect - it's on your home network
- This is expected and doesn't affect most development/testing
- All movie data and settings are fully functional
