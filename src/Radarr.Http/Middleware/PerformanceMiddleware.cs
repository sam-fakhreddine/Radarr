using System.Diagnostics;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using NzbDrone.Core.Instrumentation;

namespace Radarr.Http.Middleware
{
    public class PerformanceMiddleware
    {
        private readonly RequestDelegate _next;
        private readonly IPerformanceMetrics _metrics;

        public PerformanceMiddleware(RequestDelegate next, IPerformanceMetrics metrics)
        {
            _next = next;
            _metrics = metrics;
        }

        public async Task InvokeAsync(HttpContext context)
        {
            var sw = Stopwatch.StartNew();
            
            await _next(context);
            
            sw.Stop();
            
            var endpoint = context.Request.Path.Value?.TrimStart('/') ?? "unknown";
            _metrics.RecordApiCall(endpoint, sw.ElapsedMilliseconds);
        }
    }
}
