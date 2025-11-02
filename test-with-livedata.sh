#!/bin/bash
# Test Radarr development build with live data

DATA_DIR="/workspaces/radarr/liveData/config"

echo "Starting Radarr with live data from: $DATA_DIR"
echo "Access at: http://localhost:7878"
echo ""

# Use pre-built binaries to avoid StyleCop errors during development
if [ -f "_output/net8.0/Radarr" ]; then
    echo "Using pre-built binary"
    _output/net8.0/Radarr -data="$DATA_DIR" -nobrowser
else
    echo "Building and running (ignoring StyleCop warnings)"
    dotnet run --project src/NzbDrone.Console/Radarr.Console.csproj -p:EnforceCodeStyleInBuild=false -- -data="$DATA_DIR" -nobrowser
fi
