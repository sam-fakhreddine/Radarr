using System;
using System.Collections.Generic;
using System.Net;
using System.Threading.Tasks;
using FluentAssertions;
using Moq;
using NUnit.Framework;
using NzbDrone.Common.Cache;
using NzbDrone.Common.Http;
using NzbDrone.Common.Http.Dispatchers;
using NzbDrone.Common.TPL;
using NzbDrone.Test.Common;

namespace NzbDrone.Common.Test.Http
{
    [TestFixture]
    public class HttpClientRetryFixture : TestBase<HttpClient>
    {
        [SetUp]
        public void Setup()
        {
            Mocker.SetConstant<ICacheManager>(Mocker.Resolve<CacheManager>());
            Mocker.SetConstant<IEnumerable<IHttpRequestInterceptor>>(Array.Empty<IHttpRequestInterceptor>());

            Mocker.GetMock<IRateLimitService>()
                .Setup(x => x.WaitAndPulseAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<TimeSpan>()))
                .Returns(Task.CompletedTask);
        }

        [Test]
        public async Task should_retry_on_503_response()
        {
            var callCount = 0;

            Mocker.GetMock<IHttpDispatcher>()
                .Setup(x => x.GetResponseAsync(It.IsAny<HttpRequest>(), It.IsAny<CookieContainer>()))
                .ReturnsAsync(() =>
                {
                    callCount++;
                    if (callCount < 3)
                    {
                        return new HttpResponse(new HttpRequest("http://test.com"), new HttpHeader(), Array.Empty<byte>(), HttpStatusCode.ServiceUnavailable);
                    }

                    return new HttpResponse(new HttpRequest("http://test.com"), new HttpHeader(), Array.Empty<byte>(), HttpStatusCode.OK);
                });

            var request = new HttpRequest("http://test.com");
            request.SuppressHttpErrorStatusCodes = new[] { HttpStatusCode.ServiceUnavailable };

            var response = await Subject.GetAsync(request);

            response.StatusCode.Should().Be(HttpStatusCode.OK);
            callCount.Should().Be(3);
        }

        [Test]
        public async Task should_not_retry_on_non_503_errors()
        {
            var callCount = 0;

            Mocker.GetMock<IHttpDispatcher>()
                .Setup(x => x.GetResponseAsync(It.IsAny<HttpRequest>(), It.IsAny<CookieContainer>()))
                .ReturnsAsync(() =>
                {
                    callCount++;
                    return new HttpResponse(new HttpRequest("http://test.com"), new HttpHeader(), Array.Empty<byte>(), HttpStatusCode.NotFound);
                });

            var request = new HttpRequest("http://test.com");
            request.SuppressHttpErrorStatusCodes = new[] { HttpStatusCode.NotFound };

            var response = await Subject.GetAsync(request);

            response.StatusCode.Should().Be(HttpStatusCode.NotFound);
            callCount.Should().Be(1);
        }

        [Test]
        public async Task should_give_up_after_max_retries()
        {
            var callCount = 0;

            Mocker.GetMock<IHttpDispatcher>()
                .Setup(x => x.GetResponseAsync(It.IsAny<HttpRequest>(), It.IsAny<CookieContainer>()))
                .ReturnsAsync(() =>
                {
                    callCount++;
                    return new HttpResponse(new HttpRequest("http://test.com"), new HttpHeader(), Array.Empty<byte>(), HttpStatusCode.ServiceUnavailable);
                });

            var request = new HttpRequest("http://test.com");
            request.SuppressHttpErrorStatusCodes = new[] { HttpStatusCode.ServiceUnavailable };

            var response = await Subject.GetAsync(request);

            response.StatusCode.Should().Be(HttpStatusCode.ServiceUnavailable);
            callCount.Should().Be(4);
        }
    }
}
