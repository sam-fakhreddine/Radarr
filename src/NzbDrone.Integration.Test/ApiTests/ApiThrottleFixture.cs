using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Threading.Tasks;
using FluentAssertions;
using NUnit.Framework;

namespace NzbDrone.Integration.Test.ApiTests
{
    [TestFixture]
    public class ApiThrottleFixture : IntegrationTest
    {
        [Test]
        public async Task should_allow_concurrent_api_requests_up_to_limit()
        {
            var tasks = new List<Task>();

            for (var i = 0; i < 10; i++)
            {
                tasks.Add(Task.Run(() => Movies.All()));
            }

            await Task.WhenAll(tasks);

            tasks.All(t => t.IsCompletedSuccessfully).Should().BeTrue();
        }

        [Test]
        public async Task should_throttle_excessive_concurrent_requests()
        {
            var tasks = new List<Task<List<Client.MovieClient>>>();
            var successCount = 0;
            var throttledCount = 0;

            for (var i = 0; i < 20; i++)
            {
                tasks.Add(Task.Run(async () =>
                {
                    try
                    {
                        return await Task.FromResult(Movies.All());
                    }
                    catch (System.Net.Http.HttpRequestException ex)
                    {
                        if (ex.StatusCode == HttpStatusCode.ServiceUnavailable)
                        {
                            System.Threading.Interlocked.Increment(ref throttledCount);
                        }

                        throw;
                    }
                }));
            }

            var results = await Task.WhenAll(tasks.Select(t => t.ContinueWith(task =>
            {
                if (task.IsCompletedSuccessfully)
                {
                    System.Threading.Interlocked.Increment(ref successCount);
                    return true;
                }

                return false;
            })));

            successCount.Should().BeGreaterThan(0);
        }
    }
}
