using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading;
using NzbDrone.Common.EnvironmentInfo;

namespace NzbDrone.Core.Instrumentation
{
    public interface IPerformanceMetrics
    {
        void RecordApiCall(string endpoint, long durationMs);
        void RecordDatabaseQuery(string operation, long durationMs);
        void RecordDatabaseConnection(bool opened);
        void WriteMetrics();
    }

    public class PerformanceMetrics : IPerformanceMetrics
    {
        private readonly ConcurrentDictionary<string, MetricStats> _apiMetrics = new();
        private readonly ConcurrentDictionary<string, MetricStats> _dbMetrics = new();
        private long _dbConnectionsOpened;
        private long _dbConnectionsClosed;
        private readonly string _metricsPath;
        private readonly Timer _writeTimer;

        public PerformanceMetrics(IAppFolderInfo appFolderInfo)
        {
            _metricsPath = Path.Combine(appFolderInfo.GetLogFolder(), "performance-metrics.json");
            _writeTimer = new Timer(_ => WriteMetrics(), null, TimeSpan.FromMinutes(5), TimeSpan.FromMinutes(5));
        }

        public void RecordApiCall(string endpoint, long durationMs)
        {
            _apiMetrics.AddOrUpdate(endpoint,
                _ => new MetricStats { Count = 1, TotalMs = durationMs, MinMs = durationMs, MaxMs = durationMs },
                (_, stats) => stats.Update(durationMs));
        }

        public void RecordDatabaseQuery(string operation, long durationMs)
        {
            _dbMetrics.AddOrUpdate(operation,
                _ => new MetricStats { Count = 1, TotalMs = durationMs, MinMs = durationMs, MaxMs = durationMs },
                (_, stats) => stats.Update(durationMs));
        }

        public void RecordDatabaseConnection(bool opened)
        {
            if (opened)
            {
                Interlocked.Increment(ref _dbConnectionsOpened);
            }
            else
            {
                Interlocked.Increment(ref _dbConnectionsClosed);
            }
        }

        public void WriteMetrics()
        {
            var metrics = new
            {
                timestamp = DateTime.UtcNow,
                process = new
                {
                    memory_mb = Process.GetCurrentProcess().WorkingSet64 / 1024.0 / 1024.0,
                    threads = Process.GetCurrentProcess().Threads.Count
                },
                database = new
                {
                    connections_opened = _dbConnectionsOpened,
                    connections_closed = _dbConnectionsClosed,
                    active_connections = _dbConnectionsOpened - _dbConnectionsClosed,
                    queries = _dbMetrics.ToDictionary(
                        kvp => kvp.Key,
                        kvp => new
                        {
                            count = kvp.Value.Count,
                            avg_ms = kvp.Value.Count > 0 ? kvp.Value.TotalMs / kvp.Value.Count : 0,
                            min_ms = kvp.Value.MinMs,
                            max_ms = kvp.Value.MaxMs
                        })
                },
                api = new
                {
                    endpoints = _apiMetrics.ToDictionary(
                        kvp => kvp.Key,
                        kvp => new
                        {
                            count = kvp.Value.Count,
                            avg_ms = kvp.Value.Count > 0 ? kvp.Value.TotalMs / kvp.Value.Count : 0,
                            min_ms = kvp.Value.MinMs,
                            max_ms = kvp.Value.MaxMs
                        })
                }
            };

            try
            {
                var json = JsonSerializer.Serialize(metrics, new JsonSerializerOptions { WriteIndented = true });
                File.WriteAllText(_metricsPath, json);
            }
            catch
            {
                // Ignore write errors
            }
        }

        private class MetricStats
        {
            public long Count;
            public long TotalMs;
            public long MinMs;
            public long MaxMs;

            public MetricStats Update(long durationMs)
            {
                Interlocked.Increment(ref Count);
                Interlocked.Add(ref TotalMs, durationMs);
                
                long currentMin;
                do
                {
                    currentMin = MinMs;
                    if (durationMs >= currentMin && currentMin != 0) break;
                } while (Interlocked.CompareExchange(ref MinMs, durationMs, currentMin) != currentMin);

                long currentMax;
                do
                {
                    currentMax = MaxMs;
                    if (durationMs <= currentMax) break;
                } while (Interlocked.CompareExchange(ref MaxMs, durationMs, currentMax) != currentMax);

                return this;
            }
        }
    }
}
