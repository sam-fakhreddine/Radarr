using System.Data.SQLite;
using FluentAssertions;
using Npgsql;
using NUnit.Framework;
using NzbDrone.Common.EnvironmentInfo;
using NzbDrone.Core.Configuration;
using NzbDrone.Core.Datastore;
using NzbDrone.Test.Common;

namespace NzbDrone.Core.Test.Datastore
{
    [TestFixture]
    public class ConnectionStringFactoryFixture : TestBase<ConnectionStringFactory>
    {
        [Test]
        public void should_set_busy_timeout_to_5000ms_for_sqlite()
        {
            var appFolderInfo = Mocker.GetMock<IAppFolderInfo>();
            appFolderInfo.Setup(x => x.AppDataFolder).Returns("/test");

            var configFileProvider = Mocker.GetMock<IConfigFileProvider>();
            configFileProvider.Setup(x => x.PostgresHost).Returns(string.Empty);

            var factory = Subject;
            var connectionInfo = factory.MainDbConnection;

            connectionInfo.DatabaseType.Should().Be(DatabaseType.SQLite);

            var builder = new SQLiteConnectionStringBuilder(connectionInfo.ConnectionString);
            builder.BusyTimeout.Should().Be(5000);
        }

        [Test]
        public void should_enable_pooling_for_sqlite()
        {
            var appFolderInfo = Mocker.GetMock<IAppFolderInfo>();
            appFolderInfo.Setup(x => x.AppDataFolder).Returns("/test");

            var configFileProvider = Mocker.GetMock<IConfigFileProvider>();
            configFileProvider.Setup(x => x.PostgresHost).Returns(string.Empty);

            var factory = Subject;
            var connectionInfo = factory.MainDbConnection;

            var builder = new SQLiteConnectionStringBuilder(connectionInfo.ConnectionString);
            builder.Pooling.Should().BeTrue();
        }

        [Test]
        public void should_enable_pooling_for_postgres()
        {
            var appFolderInfo = Mocker.GetMock<IAppFolderInfo>();
            var configFileProvider = Mocker.GetMock<IConfigFileProvider>();
            configFileProvider.Setup(x => x.PostgresHost).Returns("localhost");
            configFileProvider.Setup(x => x.PostgresPort).Returns(5432);
            configFileProvider.Setup(x => x.PostgresUser).Returns("radarr");
            configFileProvider.Setup(x => x.PostgresPassword).Returns("password");
            configFileProvider.Setup(x => x.PostgresMainDb).Returns("radarr-main");

            var factory = Subject;
            var connectionInfo = factory.MainDbConnection;

            connectionInfo.DatabaseType.Should().Be(DatabaseType.PostgreSQL);

            var builder = new NpgsqlConnectionStringBuilder(connectionInfo.ConnectionString);
            builder.Pooling.Should().BeTrue();
            builder.MinPoolSize.Should().Be(2);
            builder.MaxPoolSize.Should().Be(20);
        }
    }
}
