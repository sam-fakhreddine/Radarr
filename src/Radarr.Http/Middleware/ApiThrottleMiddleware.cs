using System.Threading;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using NLog;
using Radarr.Http.Extensions;

namespace Radarr.Http.Middleware
{
    public class ApiThrottleMiddleware
    {
        private static readonly Logger Logger = LogManager.GetCurrentClassLogger();
        private static readonly SemaphoreSlim Semaphore = new SemaphoreSlim(10, 10);
        private const int TimeoutMs = 30000;

        private readonly RequestDelegate _next;

        public ApiThrottleMiddleware(RequestDelegate next)
        {
            _next = next;
        }

        public async Task InvokeAsync(HttpContext context)
        {
            if (!context.Request.IsApiRequest())
            {
                await _next(context);
                return;
            }

            var acquired = await Semaphore.WaitAsync(TimeoutMs);

            if (!acquired)
            {
                Logger.Warn("API request throttled - too many concurrent requests");
                context.Response.StatusCode = 503;
                await context.Response.WriteAsync("Service temporarily unavailable - too many concurrent requests");
                return;
            }

            try
            {
                await _next(context);
            }
            finally
            {
                Semaphore.Release();
            }
        }
    }
}
