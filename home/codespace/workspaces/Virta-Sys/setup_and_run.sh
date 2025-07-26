 #!/bin/bash

# Exit on error
set -e

# Step 1: Fix workspace directory structure
echo "Fixing nested Virta-Sys directories..."
WORKSPACE_DIR="/workspaces/Virta-Sys"
if [ -d "$WORKSPACE_DIR/Virta-Sys/Virta-Sys" ]; then
    mv /workspaces/Virta-Sys/Virta-Sys/Virta-Sys/* /workspaces/Virta-Sys/ 2>/dev/null || true
    rm -rf /workspaces/Virta-Sys/Virta-Sys 2>/dev/null || true
fi
cd "$WORKSPACE_DIR" || { echo "Failed to change to $WORKSPACE_DIR"; exit 1; }

# Step 2: Verify Git repository
echo "Checking Git repository status..."
git status || { echo "Git repository issue. Reinitializing..."; git init; }
REPO_ROOT=$(git rev-parse --show-toplevel)
if [ "$REPO_ROOT" != "$WORKSPACE_DIR" ]; then
    echo "Warning: Repository root ($REPO_ROOT) does not match workspace ($WORKSPACE_DIR)"
fi

# Step 3: Set up project directory
PROJECT_DIR="$WORKSPACE_DIR/Virta-Sys/Virta-Sys/home/codespace"
mkdir -p "$PROJECT_DIR"
cd "$PROJECT_DIR" || { echo "Failed to change to $PROJECT_DIR"; exit 1; }

# Step 4: Create or update .NET project
if [ ! -f "DataMiningService.csproj" ]; then
    echo "Creating new .NET console project..."
    dotnet new console -o . || { echo "Failed to create .NET project"; exit 1; }
fi

# Step 5: Write corrected autonomous_opz.cs with GitHub Duo integration
echo "Writing autonomous_opz.cs..."
cat << 'EOF' > autonomous_opz.cs
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using System.Text.RegularExpressions;
using System.Text.Json;
using System.Text;

namespace AutonomousDataMining
{
    public class DataSource
    {
        public string Url { get; set; }
        public string Origin { get; set; }
        public string TopLevelSite { get; set; }
        public bool IsCrossSite { get; set; }
        public double Score { get; set; }
        public DateTime LastUpdated { get; set; }
    }

    public class MLModel
    {
        public double ScoreSource(DataSource source, bool isFromGitHub = false, int? issueTitleLength = null, DateTime? issueCreatedAt = null)
        {
            double score = 0.5;
            if (source.Url.Contains("api") || source.Url.Contains("data")) score += 0.2;
            if ((DateTime.UtcNow - source.LastUpdated).TotalHours < 24) score += 0.3;
            if (source.IsCrossSite) score -= 0.1;
            if (isFromGitHub)
            {
                if (issueTitleLength.HasValue && issueTitleLength > 50) score += 0.1;
                if (issueCreatedAt.HasValue && (DateTime.UtcNow - issueCreatedAt.Value).TotalHours < 48) score += 0.15;
            }
            return Math.Max(0, Math.Min(1, score));
        }
    }

    public class IntellectualPropertyAsset
    {
        public string Name { get; set; }
        public string Owner { get; set; }
    }

    public class Entity
    {
        public string Name { get; set; }
        public List<IntellectualPropertyAsset> Assets { get; set; } = new List<IntellectualPropertyAsset>();

        public void PurchaseAsset(IntellectualPropertyAsset asset)
        {
            Assets.Add(asset);
            asset.Owner = Name;
        }
    }

    public class AIChatbot
    {
        public string Name { get; set; }

        public string GenerateResponse(string input)
        {
            return $"{Name}: Processed '{input}' for data mining.";
        }
    }

    public class GitHubDuoIntegration
    {
        private readonly string _githubApiToken;
        private readonly string _githubDuoApiEndpoint;

        public GitHubDuoIntegration(string githubApiToken, string githubDuoApiEndpoint)
        {
            _githubApiToken = githubApiToken;
            _githubDuoApiEndpoint = githubDuoApiEndpoint;
        }

        public async Task<List<DataSource>> GetDataSourcesAsync(CancellationToken cancellationToken)
        {
            using var httpClient = new HttpClient();
            httpClient.DefaultRequestHeaders.Authorization = new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", _githubApiToken);
            httpClient.DefaultRequestHeaders.Add("User-Agent", "AutonomousDataMining");

            var query = @"
                query {
                    repository(owner: ""example"", name: ""data-source-repo"") {
                        issues(states: OPEN) {
                            nodes {
                                title
                                body
                                url
                                createdAt
                            }
                        }
                    }
                }";

            var request = new HttpRequestMessage(HttpMethod.Post, _githubDuoApiEndpoint)
            {
                Content = new StringContent(JsonSerializer.Serialize(new { query }), Encoding.UTF8, "application/json")
            };

            try
            {
                var response = await httpClient.SendAsync(request, cancellationToken);
                response.EnsureSuccessStatusCode();

                var responseBody = await response.Content.ReadAsStringAsync(cancellationToken);
                var jsonDoc = JsonDocument.Parse(responseBody);
                var issues = jsonDoc.RootElement.GetProperty("data").GetProperty("repository").GetProperty("issues").GetProperty("nodes");

                var dataSources = new List<DataSource>();
                foreach (var issue in issues.EnumerateArray())
                {
                    var url = issue.GetProperty("url").GetString();
                    var createdAt = DateTime.Parse(issue.GetProperty("createdAt").GetString());
                    dataSources.Add(new DataSource
                    {
                        Url = url,
                        Origin = "https://github.com",
                        TopLevelSite = "github.com",
                        IsCrossSite = false,
                        LastUpdated = createdAt,
                        Score = 0
                    });
                }

                return dataSources;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error fetching GitHub data sources: {ex.Message}");
                return new List<DataSource>();
            }
        }
    }

    public class DataMiningService
    {
        private readonly HttpClient _httpClient;
        private readonly MLModel _mlModel;
        private readonly List<DataSource> _dataSources;
        private readonly List<Entity> _entities;
        private readonly AIChatbot _chatbot;
        private readonly CancellationTokenSource _cts;
        private readonly GitHubDuoIntegration _githubDuoIntegration;

        private readonly Dictionary<string, Func<DataSource, bool>> _rules = new Dictionary<string, Func<DataSource, bool>>
        {
            { "IsAccessible", src => !string.IsNullOrEmpty(src.Url) && src.Url.StartsWith("https") },
            { "NotTyposquatting", src => !IsPotentialTyposquatting(src.Url, src.TopLevelSite) },
            { "FreshData", src => (DateTime.UtcNow - src.LastUpdated).TotalHours < 48 },
            { "TrustedOrigin", src => src.TopLevelSite.Contains("gitlab.com") || src.TopLevelSite.Contains("github.dev") || src.TopLevelSite.Contains("google.com") || src.TopLevelSite.Contains("github.com") },
            { "SmartScreenSafe", src => !src.Url.Contains("enhancedevice.co.in") }
        };

        public DataMiningService(GitHubDuoIntegration githubDuoIntegration)
        {
            _httpClient = new HttpClient();
            _mlModel = new MLModel();
            _dataSources = new List<DataSource>();
            _entities = new List<Entity>();
            _chatbot = new AIChatbot { Name = "DataMiningBot" };
            _cts = new CancellationTokenSource();
            _githubDuoIntegration = githubDuoIntegration;
        }

        public void Initialize()
        {
            _dataSources.AddRange(new[]
            {
                new DataSource
                {
                    Url = "https://example.com/data1",
                    Origin = "https://example.com",
                    TopLevelSite = "example.com",
                    IsCrossSite = false,
                    LastUpdated = DateTime.UtcNow.AddHours(-12)
                },
                new DataSource
                {
                    Url = "https://example.org/data2",
                    Origin = "https://example.org",
                    TopLevelSite = "example.org",
                    IsCrossSite = false,
                    LastUpdated = DateTime.UtcNow.AddHours(-6)
                },
                new DataSource
                {
                    Url = "https://example.net/data3",
                    Origin = "https://example.net",
                    TopLevelSite = "example.net",
                    IsCrossSite = true,
                    LastUpdated = DateTime.UtcNow.AddHours(-24)
                }
            });

            _entities.AddRange(new[]
            {
                new Entity { Name = "Entity1" },
                new Entity { Name = "Entity2" }
            });

            var asset = new IntellectualPropertyAsset { Name = "Dataset1", Owner = "Entity1" };
            _entities[0].PurchaseAsset(asset);
        }

        private static bool IsPotentialTyposquatting(string url, string topLevelSite)
        {
            var knownDomains = new[] { "gitlab.com", "github.dev", "google.com", "github.com" };
            var domain = new Uri(url).Host;
            return !knownDomains.Any(kd => domain.EndsWith(kd)) && domain.Length > 20 && Regex.IsMatch(domain, @"[0-9a-z]{20,}") && !topLevelSite.Contains(domain);
        }

        private async Task DiscoverSourcesAsync()
        {
            var githubSources = await _githubDuoIntegration.GetDataSourcesAsync(_cts.Token);
            foreach (var source in githubSources)
            {
                if (_rules.All(rule => rule.Value(source)))
                {
                    var issueTitleLength = source.Url.Length;
                    var issueCreatedAt = source.LastUpdated;
                    source.Score = _mlModel.ScoreSource(source, isFromGitHub: true, issueTitleLength: issueTitleLength, issueCreatedAt: issueCreatedAt);
                    _dataSources.Add(source);
                    Console.WriteLine(_chatbot.GenerateResponse($"Discovered GitHub source: {source.Url}, Score: {source.Score}"));
                }
            }

            var newSources = new[]
            {
                new DataSource
                {
                    Url = "https://example.com/newdata",
                    Origin = "https://example.com",
                    TopLevelSite = "example.com",
                    IsCrossSite = false,
                    LastUpdated = DateTime.UtcNow
                }
            };

            foreach (var source in newSources)
            {
                if (_rules.All(rule => rule.Value(source)))
                {
                    source.Score = _mlModel.ScoreSource(source);
                    _dataSources.Add(source);
                    Console.WriteLine(_chatbot.GenerateResponse($"Discovered source: {source.Url}, Score: {source.Score}"));
                }
            }
        }

        private async Task IngestDataAsync(DataSource source, CancellationToken cancellationToken)
        {
            try
            {
                var response = await _httpClient.GetAsync(source.Url, cancellationToken);
                if (response.IsSuccessStatusCode)
                {
                    var content = await response.Content.ReadAsStringAsync(cancellationToken);
                    var asset = new IntellectualPropertyAsset
                    {
                        Name = $"Data_{Guid.NewGuid()}",
                        Owner = _entities[0].Name
                    };
                    _entities[1].PurchaseAsset(asset);
                    Console.WriteLine(_chatbot.GenerateResponse($"Ingested data from {source.Url}, Transferred to {asset.Owner}"));
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error ingesting {source.Url}: {ex.Message}");
            }
        }

        public async Task RunAsync()
        {
            Initialize();
            while (!_cts.Token.IsCancellationRequested)
            {
                await DiscoverSourcesAsync();
                var topSources = _dataSources.OrderByDescending(s => s.Score).Take(3).ToList();
                foreach (var source in topSources)
                {
                    await IngestDataAsync(source, _cts.Token);
                }
                await Task.Delay(TimeSpan.FromMinutes(5), _cts.Token);
            }
        }

        public void Stop()
        {
            _cts.Cancel();
            _httpClient.Dispose();
        }
    }

    class Program
    {
        static async Task Main(string[] args)
        {
            var githubApiToken = Environment.GetEnvironmentVariable("GITHUB_API_TOKEN") ?? "ghp_1234567890abcdefghijklmnopqrstuvwxyz";
            var githubDuoApiEndpoint = "https://api.github.com/graphql";

            var githubDuoIntegration = new GitHubDuoIntegration(githubApiToken, githubDuoApiEndpoint);
            var service = new DataMiningService(githubDuoIntegration);
            try
            {
                Console.WriteLine("Starting autonomous data mining service with GitHub Duo integration...");
                await service.RunAsync();
            }
            catch (OperationCanceledException)
            {
                Console.WriteLine("Service stopped.");
            }
            finally
            {
                service.Stop();
            }
        }
    }
}
EOF

# Step 6: Update .csproj to include System.Text.Json
echo "Updating .csproj to include System.Text.Json..."
cat << 'EOF' > DataMiningService.csproj
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net6.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="System.Text.Json" Version="6.0.0" />
  </ItemGroup>
</Project>
EOF

# Step 7: Set up environment variable for GitHub API token
echo "Setting up environment variable for GitHub API token..."
export GITHUB_API_TOKEN="ghp_1234567890abcdefghijklmnopqrstuvwxyz"
echo "WARNING: Using hardcoded GitHub API token for demonstration. Store securely in production."

# Step 8: Verify .NET installation
echo "Verifying .NET installation..."
if ! command -v dotnet >/dev/null 2>&1; then
    echo "Installing .NET SDK..."
    wget https://dot.net/v1/dotnet-install.sh -O dotnet-install.sh
    chmod +x dotnet-install.sh
    ./dotnet-install.sh --channel 6.0
    export PATH=$PATH:$HOME/.dotnet
fi
dotnet --version || { echo "Failed to install .NET SDK"; exit 1; }

# Step 9: Build and run the application
echo "Building and running the application..."
dotnet build || { echo "Build failed"; exit 1; }
dotnet run || { echo "Run failed"; exit 1; }

# Step 10: Optional workspace configuration for VS Code
echo "Configuring VS Code workspace..."
cat << 'EOF' > "$WORKSPACE_DIR/Virta-Sys.code-workspace"
{
    "folders": [
        {
            "path": "/workspaces/Virta-Sys"
        }
    ]
}
EOF

# Step 11: Optional devcontainer configuration
echo "Configuring devcontainer..."
mkdir -p "$WORKSPACE_DIR/.devcontainer"
cat << 'EOF' > "$WORKSPACE_DIR/.devcontainer/devcontainer.json"
{
    "name": "Virta-Sys",
    "workspaceFolder": "/workspaces/Virta-Sys",
    "image": "mcr.microsoft.com/dotnet/sdk:6.0"
}
EOF

echo "Setup complete. Application is running."