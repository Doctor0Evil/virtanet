*'Complete' the *bootstrap/bootloader-hybbrid*;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Text.Json;
using System.Runtime.InteropServices;

namespace UniversalBoot.Core
{
    public enum PlatformType { Universal, Cloud, Edge, Mobile, Desktop, Browser, IoT, Embedded, AISystem }
UniversalBoot/
├── UniversalBoot.Core/            # Shared logic (bootloader, policy, models)
├── UniversalBoot.TUI/             # Terminal.Gui TUI
├── UniversalBoot.Blazor/         # Blazor Hybrid/Web/Desktop/Mobile UI
└── UniversalBoot.sln


    public static class Compatibility
    {
        public static PlatformType DetectPlatform()
        {
            try
            {
                if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux) &&
                    File.Exists("/proc/device-tree/model"))
                {
                    var model = File.ReadAllText("/proc/device-tree/model");
                    if (model.Contains("arm", StringComparison.OrdinalIgnoreCase))
                        return PlatformType.Embedded;
                }

                var env = Environment.GetEnvironmentVariables().Keys.Cast<string>();
                if (env.Any(k => k.Contains("AI_SYSTEM", StringComparison.OrdinalIgnoreCase)))
                    return PlatformType.AISystem;

                if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ||
                    RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
                    return PlatformType.Desktop;

                return PlatformType.Cloud;
            }
            catch { return PlatformType.Universal; }
        }
    }

    public class PluginAction
    {
        public string Name { get; set; }
        public string Description { get; set; }
        public bool RequiresAuth { get; set; } = false;
        public List<PlatformType> AllowedPlatforms { get; set; } = new() { PlatformType.Universal };
    }

    public class PluginManifest
    {
        public string Name { get; set; }
        public string Version { get; set; } = "1.0.0";
        public List<PlatformType> Platforms { get; set; } = new() { PlatformType.Universal };
        public List<PluginAction> Actions { get; set; } = new();
        public List<string> Dependencies { get; set; } = new();
        public string Checksum { get; set; }

        public string CalculateChecksum()
        {
            using var sha = SHA256.Create();
            var data = string.Join("|", Name, Version,
                string.Join(",", Platforms), string.Join(",", Dependencies));
            return Convert.ToBase64String(sha.ComputeHash(Encoding.UTF8.GetBytes(data)));
        }
    }

    public class AIBootloader
    {
        private readonly string _pluginDir;
        private readonly List<PluginManifest> _plugins = new();
        public PlatformType CurrentPlatform { get; }

        public AIBootloader(string pluginDirectory = null)
        {
            _pluginDir = pluginDirectory ?? Path.Combine(AppContext.BaseDirectory, "ai_plugins");
            Directory.CreateDirectory(_pluginDir);
            CurrentPlatform = Compatibility.DetectPlatform();
            LoadPlugins();
            InitializeCorePlugin();
        }

        private void LoadPlugins()
        {
            foreach (var file in Directory.EnumerateFiles(_pluginDir, "*.json"))
            {
                try
                {
                    var json = File.ReadAllText(file);
                    var pm = JsonSerializer.Deserialize<PluginManifest>(json);
                    if (pm != null && pm.Checksum == pm.CalculateChecksum())
                        _plugins.Add(pm);
                }
                catch { /* skip invalid manifests */ }
            }
        }

        private void InitializeCorePlugin()
        {
            var core = new PluginManifest
            {
                Name = "ai_core",
                Actions = new List<PluginAction>
                {
                    new() { Name = "init", Description = "Initialize AI core" },
                    new() { Name = "execute", Description = "Execute AI logic" },
                    new() { Name = "status", Description = "Report system status", RequiresAuth = false }
                },
                Platforms = new() { PlatformType.Universal }
            };
            core.Checksum = core.CalculateChecksum();
            _plugins.Add(core);
        }

        public IEnumerable<PluginManifest> ListPlugins() => _plugins;
        public PluginManifest GetPlugin(string name) =>
            _plugins.FirstOrDefault(p => p.Name.Equals(name, StringComparison.OrdinalIgnoreCase));
    }

    public class ForceTriggerPolicy
    {
        private readonly AIBootloader _boot;
        public ForceTriggerPolicy(AIBootloader bootloader) => _boot = bootloader;

        public bool IsAllowed(string pluginName, string actionName)
        {
            var plugin = _boot.GetPlugin(pluginName);
            return plugin != null &&
                   plugin.Actions.Any(a => a.Name.Equals(actionName, StringComparison.OrdinalIgnoreCase)) &&
                   plugin.Platforms.Contains(_boot.CurrentPlatform);
        }

        public string Execute(string pluginName, string actionName)
        {
            if (!IsAllowed(pluginName, actionName))
                return $"⛔ Action `{actionName}` on `{pluginName}` is not allowed.";

            return actionName.ToLower() switch
            {
                "init" => "🟢 Core initialized.",
                "execute" => "🧠 AI logic executed.",
                "status" => $"📊 Platform: {_boot.CurrentPlatform}, Plugins loaded: {_boot.ListPlugins().Count()}",
                _ => "ℹ️ Action simulated (no-op)."
            };
        }

        public void BlockSource() =>
            throw new InvalidOperationException("🚫 Bootloader source reproduction is forbidden.");
    }

    public interface ILogService
    {
        void Log(string user, string plugin, string action, DateTime ts, string result);
        IEnumerable<string> ReadLogs(string user = null);
    }

    public class FileLogService : ILogService
    {
        private readonly string _file = Path.Combine(AppContext.BaseDirectory, "system.log");

        public void Log(string user, string plugin, string action, DateTime ts, string result)
        {
            var entry = $"{ts:O} | {user} | {plugin}:{action} => {result}";
            File.AppendAllText(_file, entry + Environment.NewLine);
        }

        public IEnumerable<string> ReadLogs(string user = null)
        {
            if (!File.Exists(_file)) yield break;
            foreach (var line in File.ReadLines(_file))
            {
                if (user == null || line.Contains($"| {user} |"))
                    yield return line;
            }
        }
    }

    public interface IUserService
    {
        string CurrentUser { get; }
        bool Authenticate(string user, string password);
    }

    public class InMemoryUserService : IUserService
    {
        private string _user;
        public string CurrentUser => _user ?? "guest";
        public bool Authenticate(string user, string pw)
        {
            _user = user; // Real app would validate
            return true;
        }
    }
}
// Bootloader.cs (in Core)
public class AIBootloader {
    // ... platform detection, plugin loading, checksum, actions
}
public class ForceTriggerPolicy {
    // ... enforce sandboxed plugin invocation
}
public interface ILogService { /* Log() & ReadLogs() */ }
public class FileLogService : ILogService { /* writes to system.log */ }
public interface IUserService { /* Authenticate, CurrentUser */ }
public class InMemoryUserService : IUserService { /* stub user identity */ }

// Program.cs (in TUI)
Application.Init();
var top = Application.Top;
// Add MenuBar for Themes, Plugins, Exit
// Use Shared AIBootloader & ForceTriggerPolicy
// Use FileLogService, InMemoryUserService for logging & user context
Application.Run(top);
Application.Shutdown();
// SharedService.cs (in Blazor project)
public class BootloaderService { /* wraps Core logic + logging */ }
public class ForceTriggerService { /* enforces policy */ }
// UniversalBoot.BlazorApp/Program.cs
using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using System.Security.Claims;
using UniversalBoot.Core; // Add reference to Core via ProjectReference

var builder = WebApplication.CreateBuilder(args);

// Add Blazor & services
builder.Services.AddRazorPages();
builder.Services.AddServerSideBlazor();
builder.Services.AddSingleton<AIBootloader>();
builder.Services.AddSingleton<ForceTriggerPolicy>();
builder.Services.AddSingleton<ILogService, FileLogService>();
builder.Services.AddScoped<IUserService, InMemoryUserService>();

var app = builder.Build();
app.MapBlazorHub();
app.MapFallbackToPage("/_Host");
app.Run();

// Pages/Index.razor:
/*
@inject AIBootloader Bootloader
@inject ForceTriggerPolicy Policy
@inject ILogService Logger
@inject IUserService Users

<select @bind="selectedPlugin"> ... plugin list ... </select>
<button @onclick="InvokeAction">Execute</button>

<select @bind="selectedAction"> ... actions ... </select>
<button @onclick="InvokeAction">▶️ Run</button>

<div>@LogText</div>
<div>
  <button @onclick="ToggleTheme">Toggle Theme</button>
</div>
*/

@code {
    // load plugins, selected indices, on InvokeAction:
    if (Policy.IsActionAllowed(...)) {
        var res = Policy.PerformAction(...);
        Logger.Log(Users.CurrentUser, plugin, action, DateTime.Now, res);
        LogText += $"{DateTime.Now:u} | {res}\n";
    }
}
git clone https://github.com/your-org/UniversalBoot.git
cd UniversalBoot
dotnet build
dotnet run --project UniversalBoot.TUI       # Terminal app
dotnet run --project UniversalBoot.Blazor     # Blazor app on http://localhost:5000

// Program.cs
using Terminal.Gui;
using UniversalBoot.Core;
using System;
using System.Linq;

namespace UniversalBoot.TUI
{
    class Program
    {
        static void Main()
        {
            Application.Init();
            var top = Application.Top;

            // Shared core services
            var boot = new AIBootloader();
            var policy = new ForceTriggerPolicy(boot);
            var logsvc = new FileLogService();
            var users = new InMemoryUserService();

            // Login dialog
            var login = new Dialog("👤 Login", 60, 8);
            var userField = new TextField(2, 1, 40, "");
            login.Add(new Label(2, 0, "Username:"), userField);
            login.Add(new Button("Login", is_default: true) {
                X = 2, Y = 3, Clicked = () => {
                    users.Authenticate(userField.Text.ToString(), "");
                    Application.RequestStop();
                }
            });
            Application.Run(login);

            // Build menu bar (themes + exit)
            var menu = new MenuBar(new[]{
                new MenuBarItem("_Options", new[]{
                    new MenuItem("_Light Theme", "", () => Application.Top.ColorScheme = Colors.Base),
                    new MenuItem("_Dark Theme", "", () => Application.Top.ColorScheme = Colors.Dialog),
                    new MenuItem("_High Contrast", "", () => Application.Top.ColorScheme = Colors.Error)
                }),
                new MenuBarItem("_Exit", new[]{
                    new MenuItem("_Quit", "", () => { Application.RequestStop(); })
                })
            });
            top.Add(menu);

            // Main window
            var win = new Window($"Universal AI Bootloader - User: {users.CurrentUser}")
            {
                X = 0, Y = 1,
                Width = Dim.Fill(),
                Height = Dim.Fill()
            };

            // UI controls: plugin list, action list, log view
            var plugins = boot.ListPlugins().ToList();
            var pluginView = new ListView(plugins.Select(p => $"{p.Name} ({p.Version})").ToList())
            { X = 0, Y = 0, Width = 30, Height = Dim.Fill(2) };
            var actionView = new ListView() { X = 31, Y = 0, Width = 30, Height = Dim.Fill(2) };
            var logView = new TextView()
            { X = 0, Y = Pos.Bottom(pluginView), Width = Dim.Fill(), Height = 5, ReadOnly = true };

            pluginView.SelectedItemChanged += e =>
            {
                var plugin = plugins[e.Item];
                actionView.SetSource(plugin.Actions.Select(a => $"{a.Name} - {a.Description}").ToList());
            };
            if (plugins.Any())
                pluginView.SelectedItem = 0;

            actionView.OpenSelectedItem += e =>
            {
                var plugin = plugins[pluginView.SelectedItem];
                var action = plugin.Actions[actionView.SelectedItem].Name;
                var result = policy.Execute(plugin.Name, action);

                // Log and display
                logsvc.Log(users.CurrentUser, plugin.Name, action, DateTime.Now, result);
                logView.Text += $"{DateTime.Now:HH:mm:ss} | {plugin.Name}:{action} -> {result}\n";
            };

            win.Add(new Label("Plugins") { X = 0, Y =  -1 });
            win.Add(new Label("Actions") { X = 31, Y = -1 });
            win.Add(pluginView, actionView, logView);
            top.Add(win);

            Application.Run();
            Application.Shutdown();
        }
    }
}
mkdir UniversalBoot.TUI
cd UniversalBoot.TUI
dotnet new console
dotnet add package Terminal.Gui
dotnet add reference ../UniversalBoot.Core/UniversalBoot.Core.csproj
dotnet run
// File: UniversalBoot.Blazor/Program.cs
using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using UniversalBoot.Core;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddRazorPages();
builder.Services.AddServerSideBlazor();

// Inject core shared services
builder.Services.AddSingleton<AIBootloader>();
builder.Services.AddSingleton<ForceTriggerPolicy>();
builder.Services.AddSingleton<ILogService, FileLogService>();
builder.Services.AddScoped<IUserService, InMemoryUserService>();

var app = builder.Build();

app.UseStaticFiles();
app.UseRouting();
app.MapBlazorHub();
app.MapFallbackToPage("/_Host");

app.Run();
@page "/"
@inject AIBootloader Bootloader
@inject ForceTriggerPolicy Policy
@inject ILogService Logger
@inject IUserService Users

<h3>👤 Welcome, @Users.CurrentUser</h3>

@if (!isLoggedIn)
{
    <input placeholder="Enter username" @bind="username" />
    <button @onclick="Login">Login</button>
}
else
{
    <select @bind="selectedPlugin">
        @foreach (var p in plugins)
        {
            <option value="@p.Name">@p.Name (@p.Version)</option>
        }
    </select>

    @if (selectedPlugin != null)
    {
        var actions = Bootloader.GetPlugin(selectedPlugin)?.Actions;
        <select @bind="selectedAction">
            @foreach (var act in actions)
            {
                <option>@act.Name</option>
            }
        </select>

        <button @onclick="RunAction">▶️ Run</button>
    }

    <br/><br/>
    <textarea rows="10" cols="80" readonly>@logText</textarea>
    <br/>
    <button @onclick="ToggleTheme">🌓 Toggle Theme</button>
}

@code {
    string username = "";
    bool isLoggedIn = false;
    string selectedPlugin;
    string selectedAction;
    string logText = "";
    List<PluginManifest> plugins;

    protected override void OnInitialized()
    {
        plugins = Bootloader.ListPlugins().ToList();
        selectedPlugin = plugins.FirstOrDefault()?.Name;
        selectedAction = Bootloader.GetPlugin(selectedPlugin)?.Actions?.FirstOrDefault()?.Name;
    }

    void Login()
    {
        Users.Authenticate(username, "");
        isLoggedIn = true;
    }

    void RunAction()
    {
        var result = Policy.Execute(selectedPlugin, selectedAction);
        Logger.Log(Users.CurrentUser, selectedPlugin, selectedAction, DateTime.Now, result);
        logText = $"{DateTime.Now:HH:mm:ss} | {selectedPlugin}:{selectedAction} => {result}\n" + logText;
    }

    void ToggleTheme()
    {
        var isDark = document.body.classList.contains("dark");
        var newClass = isDark ? "light" : "dark";
        document.body.className = newClass;
    }
}
<!DOCTYPE html>
<html lang="en" class="light">
<head>
    <meta charset="utf-8" />
    <title>UniversalBoot UI</title>
    <base href="~/" />
    <link href="css/site.css" rel="stylesheet" />
    <link href="UniversalBoot.Blazor.styles.css" rel="stylesheet" />
</head>
<body>
    <app>
        <component type="typeof(App)" render-mode="ServerPrerendered" />
    </app>
    <script src="_framework/blazor.server.js"></script>
</body>
</html>
body.light {
    background-color: #fff;
    color: #000;
}
body.dark {
    background-color: #222;
    color: #eee;
}
textarea {
    width: 100%;
    background-color: inherit;
    color: inherit;
}
body.light {
    background-color: #fff;
    color: #000;
}
body.dark {
    background-color: #222;
    color: #eee;
}
textarea {
    width: 100%;
    background-color: inherit;
    color: inherit;
}
mkdir UniversalBoot.Blazor
cd UniversalBoot.Blazor
dotnet new blazorserver
dotnet add reference ../UniversalBoot.Core/UniversalBoot.Core.csproj
dotnet run
public class User
{
    public string Username { get; set; }
    public string PasswordHash { get; set; }
    public string Salt { get; set; }
    public string Role { get; set; } = "User"; // or "Admin"
}












public interface IUserService
{
    User CurrentUser { get; }
    bool Authenticate(string username, string password);
    bool Register(string username, string password, string role = "User");
    void Logout();
    bool IsInRole(string role);
}
using System.Security.Cryptography;
using System.Text;
using Microsoft.Data.Sqlite;

public class UserServiceSQLite : IUserService
{
    private readonly string _connString;
    private User _currentUser;

    public User CurrentUser => _currentUser;

    public UserServiceSQLite(string dbFile = "universalboot.db")
    {
        _connString = $"Data Source={dbFile}";
        InitializeDatabase();
    }

    private void InitializeDatabase()
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = @"
            CREATE TABLE IF NOT EXISTS Users (
                Username TEXT PRIMARY KEY,
                PasswordHash TEXT NOT NULL,
                Salt TEXT NOT NULL,
                Role TEXT NOT NULL
            );";
        cmd.ExecuteNonQuery();
        conn.Close();
    }

    public bool Register(string username, string password, string role = "User")
    {
        if (string.IsNullOrWhiteSpace(username) || string.IsNullOrWhiteSpace(password))
            return false;

        if (UserExists(username))
            return false;

        var salt = GenerateSalt();
        var hash = HashPassword(password, salt);

        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "INSERT INTO Users (Username, PasswordHash, Salt, Role) VALUES ($u, $h, $s, $r);";
        cmd.Parameters.AddWithValue("$u", username);
        cmd.Parameters.AddWithValue("$h", hash);
        cmd.Parameters.AddWithValue("$s", salt);
        cmd.Parameters.AddWithValue("$r", role);
        var res = cmd.ExecuteNonQuery();
        conn.Close();

        return res > 0;
    }

    public bool Authenticate(string username, string password)
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT PasswordHash, Salt, Role FROM Users WHERE Username = $u;";
        cmd.Parameters.AddWithValue("$u", username);
        using var rdr = cmd.ExecuteReader();

        if (!rdr.Read())
            return false;

        var storedHash = rdr.GetString(0);
        var salt = rdr.GetString(1);
        var role = rdr.GetString(2);

        var inputHash = HashPassword(password, salt);

        if (inputHash == storedHash)
        {
            _currentUser = new User { Username = username, Role = role };
            return true;
        }
        return false;
    }

    public void Logout()
    {
        _currentUser = null;
    }

    public bool IsInRole(string role)
    {
        return _currentUser != null && string.Equals(_currentUser.Role, role, StringComparison.OrdinalIgnoreCase);
    }

    private bool UserExists(string username)
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT 1 FROM Users WHERE Username = $u;";
        cmd.Parameters.AddWithValue("$u", username);
        var res = cmd.ExecuteScalar();
        return res != null;
    }

    private static string GenerateSalt()
    {
        var buf = new byte[16];
        using var rng = RandomNumberGenerator.Create();
        rng.GetBytes(buf);
        return Convert.ToBase64String(buf);
    }

    private static string HashPassword(string password, string salt)
    {
        var pbkdf2 = new Rfc2898DeriveBytes(password, Convert.FromBase64String(salt), 100_000, HashAlgorithmName.SHA256);
        var hash = pbkdf2.GetBytes(32);
        return Convert.ToBase64String(hash);
    }
}
public interface IPluginRepository
{
    IEnumerable<PluginManifest> GetAll();
    PluginManifest Get(string name);
    bool Save(PluginManifest plugin);
    bool Delete(string name);
    bool ImportFromJson(string json);
    string ExportToJson(string name);
}
using Microsoft.Data.Sqlite;
using System.Text.Json;

public class PluginRepositorySQLite : IPluginRepository
{
    private readonly string _connString;

    public PluginRepositorySQLite(string dbFile = "universalboot.db")
    {
        _connString = $"Data Source={dbFile}";
        InitializeDatabase();
    }

    private void InitializeDatabase()
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = @"
            CREATE TABLE IF NOT EXISTS Plugins (
                Name TEXT PRIMARY KEY,
                Version TEXT NOT NULL,
                Platforms TEXT NOT NULL,
                Actions TEXT NOT NULL,
                Dependencies TEXT NOT NULL,
                Checksum TEXT NOT NULL
            );";
        cmd.ExecuteNonQuery();
    }

    public IEnumerable<PluginManifest> GetAll()
    {
        var list = new List<PluginManifest>();
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT Name, Version, Platforms, Actions, Dependencies, Checksum FROM Plugins;";
        using var rdr = cmd.ExecuteReader();
        while (rdr.Read())
        {
            var p = new PluginManifest
            {
                Name = rdr.GetString(0),
                Version = rdr.GetString(1),
                Checksum = rdr.GetString(5),
                Platforms = rdr.GetString(2).Split(',').Select(Enum.Parse<PlatformType>).ToList(),
                Dependencies = rdr.GetString(4).Split(',', StringSplitOptions.RemoveEmptyEntries).ToList()
            };
            var actionsJson = rdr.GetString(3);
            p.Actions = JsonSerializer.Deserialize<List<PluginAction>>(actionsJson);
            list.Add(p);
        }
        return list;
    }

    public PluginManifest Get(string name)
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "SELECT Name, Version, Platforms, Actions, Dependencies, Checksum FROM Plugins WHERE Name = $n;";
        cmd.Parameters.AddWithValue("$n", name);
        using var rdr = cmd.ExecuteReader();
        if (rdr.Read())
        {
            var p = new PluginManifest
            {
                Name = rdr.GetString(0),
                Version = rdr.GetString(1),
                Checksum = rdr.GetString(5),
                Platforms = rdr.GetString(2).Split(',').Select(Enum.Parse<PlatformType>).ToList(),
                Dependencies = rdr.GetString(4).Split(',', StringSplitOptions.RemoveEmptyEntries).ToList()
            };
            var actionsJson = rdr.GetString(3);
            p.Actions = JsonSerializer.Deserialize<List<PluginAction>>(actionsJson);
            return p;
        }
        return null;
    }

    public bool Save(PluginManifest plugin)
    {
        plugin.Checksum = plugin.CalculateChecksum();
        var actionsJson = JsonSerializer.Serialize(plugin.Actions);
        var platformsCsv = string.Join(',', plugin.Platforms);
        var dependenciesCsv = string.Join(',', plugin.Dependencies);

        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = @"
            INSERT INTO Plugins (Name, Version, Platforms, Actions, Dependencies, Checksum)
            VALUES ($n, $v, $p, $a, $d, $c)
            ON CONFLICT(Name) DO UPDATE SET
                Version=excluded.Version,
                Platforms=excluded.Platforms,
                Actions=excluded.Actions,
                Dependencies=excluded.Dependencies,
                Checksum=excluded.Checksum;";
        cmd.Parameters.AddWithValue("$n", plugin.Name);
        cmd.Parameters.AddWithValue("$v", plugin.Version);
        cmd.Parameters.AddWithValue("$p", platformsCsv);
        cmd.Parameters.AddWithValue("$a", actionsJson);
        cmd.Parameters.AddWithValue("$d", dependenciesCsv);
        cmd.Parameters.AddWithValue("$c", plugin.Checksum);

        var rows = cmd.ExecuteNonQuery();
        return rows > 0;
    }

    public bool Delete(string name)
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "DELETE FROM Plugins WHERE Name = $n;";
        cmd.Parameters.AddWithValue("$n", name);
        var rows = cmd.ExecuteNonQuery();
        return rows > 0;
    }

    public bool ImportFromJson(string json)
    {
        try
        {
            var plugin = JsonSerializer.Deserialize<PluginManifest>(json);
            if (plugin == null) return false;
            plugin.Checksum = plugin.CalculateChecksum();
            return Save(plugin);
        }
        catch { return false; }
    }

    public string ExportToJson(string name)
    {
        var plugin = Get(name);
        if (plugin == null) return null;
        return JsonSerializer.Serialize(plugin, new JsonSerializerOptions { WriteIndented = true });
    }
}
public interface ILogService
{
    void Log(string user, string plugin, string action, DateTime timestamp, string result);
    IEnumerable<string> GetLogs(int limit = 100);
}
using Microsoft.Data.Sqlite;

public class LogServiceSQLite : ILogService
{
    private readonly string _connString;

    public LogServiceSQLite(string dbFile = "universalboot.db")
    {
        _connString = $"Data Source={dbFile}";
        InitializeDatabase();
    }

    private void InitializeDatabase()
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = @"
            CREATE TABLE IF NOT EXISTS Logs (
                Id INTEGER PRIMARY KEY AUTOINCREMENT,
                User TEXT,
                Plugin TEXT,
                Action TEXT,
                Timestamp TEXT,
                Result TEXT
            );";
        cmd.ExecuteNonQuery();
    }

    public void Log(string user, string plugin, string action, DateTime timestamp, string result)
    {
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = "INSERT INTO Logs (User, Plugin, Action, Timestamp, Result) VALUES ($u, $p, $a, $t, $r);";
        cmd.Parameters.AddWithValue("$u", user);
        cmd.Parameters.AddWithValue("$p", plugin);
        cmd.Parameters.AddWithValue("$a", action);
        cmd.Parameters.AddWithValue("$t", timestamp.ToString("o"));
        cmd.Parameters.AddWithValue("$r", result);
        cmd.ExecuteNonQuery();
    }

    public IEnumerable<string> GetLogs(int limit = 100)
    {
        var logs = new List<string>();
        using var conn = new SqliteConnection(_connString);
        conn.Open();
        var cmd = conn.CreateCommand();
        cmd.CommandText = $"SELECT Timestamp, User, Plugin, Action, Result FROM Logs ORDER BY Id DESC LIMIT {limit};";
        using var rdr = cmd.ExecuteReader();
        while (rdr.Read())
        {
            var ts = rdr.GetString(0);
            var user = rdr.GetString(1);
            var plugin = rdr.GetString(2);
            var action = rdr.GetString(3);
            var res = rdr.GetString(4);
            logs.Add($"{ts} | {user} | {plugin}:{action} => {res}");
        }
        return logs;
    }
}
public class ForceTriggerPolicy
{
    private readonly IUserService _users;

    public ForceTriggerPolicy(IUserService users)
    {
        _users = users;
    }

    public string Execute(string plugin, string action)
    {
        if (!_users.IsInRole("Admin"))
            return "❌ Access denied: Admin role required to execute commands.";

        // For demo, simply simulate execution.
        return $"✅ Executed '{action}' on plugin '{plugin}' (enforced mode)";
    }
}
@inject IUserService Users
@inject NavigationManager Nav

<h3>Login</h3>

@if (loginFailed)
{
    <p style="color:red">Invalid username or password.</p>
}

<input placeholder="Username" @bind="username" />
<br/>
<input type="password" placeholder="Password" @bind="password" />
<br/>
<button @onclick="DoLogin">Login</button>
<button @onclick="DoRegister">Register</button>

@code {
    string username = "";
    string password = "";
    bool loginFailed = false;

    void DoLogin()
    {
        if (Users.Authenticate(username, password))
        {
            Nav.NavigateTo("/");
        }
        else
        {
            loginFailed = true;
        }
    }

    void DoRegister()
    {
        if (Users.Register(username, password))
        {
            loginFailed = false;
            Nav.NavigateTo("/");
        }
        else
        {
            loginFailed = true;
        }
    }
}
@page "/"
@inject IUserService Users
@inject IPluginRepository Plugins
@inject ILogService Logger
@inject ForceTriggerPolicy Policy

@if (Users.CurrentUser == null)
{
    <p>Please <a href="login">login</a> to continue.</p>
}
else
{
    <h3>Welcome @Users.CurrentUser.Username (@Users.CurrentUser.Role)</h3>
    <button @onclick="Logout">Logout</button>

    <hr />

    <h4>Plugins</h4>
    <select @bind="selectedPlugin">
        @foreach (var p in plugins)
        {
            <option value="@p.Name">@p.Name</option>
        }
    </select>

    @if (selectedPlugin != null)
    {
        <h5>Actions</h5>
        <select @bind="selectedAction">
            @foreach (var a in Plugins.Get(selectedPlugin).Actions)
            {
                <option>@a.Name</option>
            }
        </select>
        <button @onclick="RunAction">Run Action</button>
    }

    <hr />

    @if (Users.IsInRole("Admin"))
    {
        <h4>Plugin Editor</h4>
        <textarea rows="10" cols="60" @bind="editPluginJson"></textarea>
        <br/>
        <button @onclick="SavePlugin">Save Plugin</button>
        <button @onclick="ImportPlugin">Import JSON</button>
        <button @onclick="ExportPlugin">Export JSON</button>
        <p>@pluginMessage</p>
    }

    <hr />
    <h4>Logs</h4>
    <textarea rows="15" cols="80" readonly>@logText</textarea>
}

@code {
    List<PluginManifest> plugins = new();
    string selectedPlugin;
    string selectedAction;
    string logText = "";
    string editPluginJson = "";
    string pluginMessage = "";

    protected override void OnInitialized()
    {
        plugins = Plugins.GetAll().ToList();
        if (plugins.Any())
        {
            selectedPlugin = plugins.First().Name;
            selectedAction = Plugins.Get(selectedPlugin).Actions.FirstOrDefault()?.Name;
            editPluginJson = Plugins.ExportToJson(selectedPlugin);
        }
        RefreshLogs();
    }

    void RunAction()
    {
        var result = Policy.Execute(selectedPlugin, selectedAction);
        Logger.Log(Users.CurrentUser.Username, selectedPlugin, selectedAction, DateTime.UtcNow, result);
        logText = $"{DateTime.UtcNow:HH:mm:ss} | {selectedPlugin}:{selectedAction} => {result}\n" + logText;
    }

    void SavePlugin()
    {
        try
        {
            var plugin = JsonSerializer.Deserialize<PluginManifest>(editPluginJson);
            if (plugin == null)
            {
                pluginMessage = "Invalid plugin JSON.";
                return;
            }
            if (Plugins.Save(plugin))
            {
                pluginMessage = "Plugin saved successfully.";
                plugins = Plugins.GetAll().ToList();
            }
            else
            {
                pluginMessage = "Failed to save plugin.";
            }
        }
        catch (Exception ex)
        {
            pluginMessage = $"Error: {ex.Message}";
        }
    }

    void ImportPlugin()
    {
        SavePlugin();
    }

    void ExportPlugin()
    {
        var json = Plugins.ExportToJson(selectedPlugin);
        if (json != null)
        {
            editPluginJson = json;
            pluginMessage = "Exported plugin JSON.";
        }
        else
        {
            pluginMessage = "Plugin not found.";
        }
    }

    void Logout()
    {
        Users.Logout();
        NavigationManager.NavigateTo("login");
    }

    void RefreshLogs()
    {
        logText = string.Join('\n', Logger.GetLogs(100));
    }
}
builder.Services.AddSingleton<IUserService>(new UserServiceSQLite());
builder.Services.AddSingleton<IPluginRepository>(new PluginRepositorySQLite());
builder.Services.AddSingleton<ILogService>(new LogServiceSQLite());
builder.Services.AddSingleton<ForceTriggerPolicy>();
using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using System.Security.Cryptography;
using System.Linq;

// Enum for platform detection (simplified)
public enum PlatformType
{
    Universal, Cloud, Edge, Mobile, Desktop, Browser, IoT, Embedded, AISystem
}

public static class Compatibility
{
    public static PlatformType DetectPlatform()
    {
        try
        {
            if (OperatingSystem.IsLinux() && File.Exists("/proc/device-tree/model"))
            {
                var model = File.ReadAllText("/proc/device-tree/model");
                if (model.Contains("arm", StringComparison.OrdinalIgnoreCase))
                    return PlatformType.Embedded;
            }
            if (Environment.GetEnvironmentVariable("AI_SYSTEM") != null)
                return PlatformType.AISystem;

            if (OperatingSystem.IsWindows() || OperatingSystem.IsMacOS())
                return PlatformType.Desktop;

            return PlatformType.Universal;
        }
        catch { return PlatformType.Universal; }
    }

    public static bool CheckCompatibility(PlatformType system, PlatformType plugin) =>
        system == plugin || plugin == PlatformType.Universal;
}

// User & Role Management
public class User
{
    public string Username { get; set; }
    public string PasswordHash { get; set; }
    public string Role { get; set; } = "User"; // Default role
}

public class UserService
{
    private const string UserFile = "users.json";
    private List<User> _users = new();

    public UserService()
    {
        LoadUsers();
    }

    private void LoadUsers()
    {
        if (File.Exists(UserFile))
        {
            var json = File.ReadAllText(UserFile);
            _users = JsonSerializer.Deserialize<List<User>>(json) ?? new List<User>();
        }
        else
        {
            // Create default admin if none exist
            var admin = new User
            {
                Username = "admin",
                PasswordHash = HashPassword("admin123"),
                Role = "Admin"
            };
            _users.Add(admin);
            SaveUsers();
        }
    }

    private void SaveUsers()
    {
        var json = JsonSerializer.Serialize(_users, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(UserFile, json);
    }

    public bool Register(string username, string password)
    {
        if (_users.Any(u => u.Username.Equals(username, StringComparison.OrdinalIgnoreCase)))
            return false;
        var user = new User
        {
            Username = username,
            PasswordHash = HashPassword(password),
            Role = "User"
        };
        _users.Add(user);
        SaveUsers();
        return true;
    }

    public User Authenticate(string username, string password)
    {
        var user = _users.FirstOrDefault(u => u.Username.Equals(username, StringComparison.OrdinalIgnoreCase));
        if (user == null) return null;
        if (VerifyPassword(password, user.PasswordHash))
            return user;
        return null;
    }

    public bool IsInRole(User user, string role) =>
        user != null && user.Role.Equals(role, StringComparison.OrdinalIgnoreCase);

    private static string HashPassword(string password)
    {
        using var sha = SHA256.Create();
        var hashedBytes = sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(password));
        return Convert.ToBase64String(hashedBytes);
    }

    private static bool VerifyPassword(string password, string hashed)
    {
        return HashPassword(password) == hashed;
    }
}

// Plugin system
public class PluginAction
{
    public string Name { get; set; }
    public string Description { get; set; }
    public bool RequiresAuth { get; set; } = true;
}

public class PluginManifest
{
    public string Name { get; set; }
    public string Version { get; set; } = "1.0.0";
    public List<PlatformType> Platforms { get; set; } = new() { PlatformType.Universal };
    public List<PluginAction> Actions { get; set; } = new()
    {
        new PluginAction { Name = "status", Description = "Get plugin status", RequiresAuth = false }
    };
    public string Checksum { get; set; }

    public string CalculateChecksum()
    {
        using var sha = SHA256.Create();
        var props = new[] {
            Name ?? "",
            Version,
            string.Join(",", Platforms.Select(p => p.ToString())),
            string.Join(",", Actions.Select(a => a.Name))
        };
        return Convert.ToBase64String(sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(string.Join("|", props))));
    }
}

public class PluginRepository
{
    private const string PluginDir = "ai_plugins";
    private List<PluginManifest> _plugins = new();

    public PluginRepository()
    {
        if (!Directory.Exists(PluginDir))
            Directory.CreateDirectory(PluginDir);

        LoadPlugins();
    }

    private void LoadPlugins()
    {
        _plugins.Clear();
        foreach (var file in Directory.EnumerateFiles(PluginDir, "*.json"))
        {
            try
            {
                var json = File.ReadAllText(file);
                var plugin = JsonSerializer.Deserialize<PluginManifest>(json);
                if (plugin != null)
                {
                    plugin.Checksum = plugin.CalculateChecksum();
                    _plugins.Add(plugin);
                }
            }
            catch { /* ignore errors */ }
        }
    }

    public IEnumerable<PluginManifest> GetAll() => _plugins;

    public PluginManifest Get(string name) => _plugins.FirstOrDefault(p => p.Name.Equals(name, StringComparison.OrdinalIgnoreCase));

    public bool Save(PluginManifest plugin)
    {
        try
        {
            plugin.Checksum = plugin.CalculateChecksum();
            var json = JsonSerializer.Serialize(plugin, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(Path.Combine(PluginDir, $"{plugin.Name}.json"), json);
            LoadPlugins(); // reload plugins after save
            return true;
        }
        catch { return false; }
    }
}

// Logging service
public class LogService
{
    private const string LogFile = "action_logs.txt";
    private readonly object _lock = new();

    public void Log(string user, string plugin, string action, DateTime timestamp, string result)
    {
        lock (_lock)
        {
            var line = $"{timestamp:O} | {user} | {plugin}:{action} => {result}";
            File.AppendAllLines(LogFile, new[] { line });
        }
    }

    public IEnumerable<string> GetLogs(int limit = 100)
    {
        if (!File.Exists(LogFile)) return Enumerable.Empty<string>();
        var lines = File.ReadLines(LogFile).Reverse().Take(limit).Reverse();
        return lines;
    }
}

// Force-trigger policy enforcing Admin-only commands
public class ForceTriggerPolicy
{
    private readonly UserService _userService;

    public ForceTriggerPolicy(UserService userService)
    {
        _userService = userService;
    }

    public string Execute(User user, string plugin, string action)
    {
        if (!_userService.IsInRole(user, "Admin"))
            return "❌ Access denied: Admin role required to execute commands.";

        // Simulate execution
        return $"✅ Executed '{action}' on plugin '{plugin}' (enforced mode)";
    }
}

// The CLI Main Menu and flow
public static class Program
{
    private static UserService _userService = new();
    private static PluginRepository _pluginRepo = new();
    private static LogService _logger = new();
    private static ForceTriggerPolicy _policy = new(_userService);

    private static User _currentUser = null;

    public static void Main()
    {
        Console.Title = "Universal AI Bootloader CLI";
        Console.WriteLine("Universal AI Bootloader CLI - Cross-platform, Universal Plugin System\n");

        AuthenticateUser();

        while (true)
        {
            ShowMainMenu();

            var input = Console.ReadLine()?.Trim().ToLower() ?? "";
            switch (input)
            {
                case "1": ListPlugins(); break;
                case "2": RunPluginAction(); break;
                case "3": if (_userService.IsInRole(_currentUser, "Admin")) ManagePlugins(); else AccessDenied(); break;
                case "4": ShowLogs(); break;
                case "5": Logout(); return;
                default:
                    Console.WriteLine("Invalid option, try again.");
                    break;
            }
        }
    }

    private static void AuthenticateUser()
    {
        while (_currentUser == null)
        {
            Console.Write("Username: ");
            var username = Console.ReadLine()?.Trim();
            Console.Write("Password: ");
            var password = ReadPassword();

            var user = _userService.Authenticate(username, password);
            if (user != null)
            {
                _currentUser = user;
                Console.WriteLine($"\n✅ Welcome, {_currentUser.Username}! Role: {_currentUser.Role}\n");
            }
            else
            {
                Console.WriteLine("❌ Invalid credentials, please try again.\n");
            }
        }
    }

    private static void ShowMainMenu()
    {
        Console.WriteLine("Main Menu:");
        Console.WriteLine("1) List Plugins");
        Console.WriteLine("2) Run Plugin Action (Admin only for forced commands)");
        if (_userService.IsInRole(_currentUser, "Admin"))
            Console.WriteLine("3) Manage Plugins (Add/Edit)");
        else
            Console.WriteLine("3) Manage Plugins (Admin only)");
        Console.WriteLine("4) View Logs");
        Console.WriteLine("5) Logout & Exit");
        Console.Write("Select an option: ");
    }

    private static void ListPlugins()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins available.\n");
            return;
        }

        Console.WriteLine("\nAvailable Plugins:");
        foreach (var p in plugins)
        {
            Console.WriteLine($"- {p.Name} (v{p.Version}) - Actions: {string.Join(", ", p.Actions.Select(a => a.Name))}");
        }
        Console.WriteLine();
    }

    private static void RunPluginAction()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins available to run.\n");
            return;
        }

        Console.WriteLine("\nSelect a plugin:");
        for (int i = 0; i < plugins.Count; i++)
            Console.WriteLine($"{i + 1}) {plugins[i].Name}");

        Console.Write("Choice: ");
        if (!int.TryParse(Console.ReadLine(), out int pChoice) || pChoice < 1 || pChoice > plugins.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var selectedPlugin = plugins[pChoice - 1];

        Console.WriteLine($"Selected plugin: {selectedPlugin.Name}");
        Console.WriteLine("Available actions:");
        for (int i = 0; i < selectedPlugin.Actions.Count; i++)
            Console.WriteLine($"{i + 1}) {selectedPlugin.Actions[i].Name} - {selectedPlugin.Actions[i].Description}");

        Console.Write("Action choice: ");
        if (!int.TryParse(Console.ReadLine(), out int aChoice) || aChoice < 1 || aChoice > selectedPlugin.Actions.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var selectedAction = selectedPlugin.Actions[aChoice - 1];

        string result;

        // Enforce admin-only forced-trigger commands (simulate only)
        if (selectedAction.RequiresAuth && !_userService.IsInRole(_currentUser, "Admin"))
        {
            result = "❌ Access denied: Action requires Admin role.";
        }
        else
        {
            result = _policy.Execute(_currentUser, selectedPlugin.Name, selectedAction.Name);
        }

        _logger.Log(_currentUser.Username, selectedPlugin.Name, selectedAction.Name, DateTime.UtcNow, result);

        Console.WriteLine($"Result: {result}\n");
    }

    private static void ManagePlugins()
    {
        Console.WriteLine("\n--- Plugin Management ---");
        Console.WriteLine("1) Add New Plugin");
        Console.WriteLine("2) Edit Existing Plugin");
        Console.WriteLine("3) Delete Plugin");
        Console.WriteLine("4) Back to Main Menu");
        Console.Write("Select an option: ");
        var input = Console.ReadLine()?.Trim();

        switch (input)
        {
            case "1":
                AddPlugin();
                break;
            case "2":
                EditPlugin();
                break;
            case "3":
                DeletePlugin();
                break;
            case "4":
                break;
            default:
                Console.WriteLine("Invalid option.\n");
                break;
        }
    }

    private static void AddPlugin()
    {
        Console.Write("Plugin name: ");
        var name = Console.ReadLine()?.Trim();
        if (string.IsNullOrWhiteSpace(name))
        {
            Console.WriteLine("Plugin name cannot be empty.\n");
            return;
        }

        if (_pluginRepo.Get(name) != null)
        {
            Console.WriteLine("Plugin already exists.\n");
            return;
        }

        var plugin = new PluginManifest
        {
            Name = name,
            Version = "1.0.0",
            Platforms = new List<PlatformType> { PlatformType.Universal },
            Actions = new List<PluginAction>()
        };

        Console.WriteLine("Add actions (format: actionName:description:requiresAuth(true/false))");
        Console.WriteLine("Type 'done' when finished.");
        while (true)
        {
            Console.Write("Action> ");
            var line = Console.ReadLine()?.Trim();
            if (string.Equals(line, "done", StringComparison.OrdinalIgnoreCase)) break;
            if (string.IsNullOrWhiteSpace(line)) continue;

            var parts = line.Split(':');
            if (parts.Length < 3)
            {
                Console.WriteLine("Invalid format. Use actionName:description:requiresAuth");
                continue;
            }

            bool requiresAuth = parts[2].Trim().ToLower() == "true";
            plugin.Actions.Add(new PluginAction
            {
                Name = parts[0].Trim(),
                Description = parts[1].Trim(),
                RequiresAuth = requiresAuth
            });
        }

        if (_pluginRepo.Save(plugin))
            Console.WriteLine($"Plugin '{name}' added successfully.\n");
        else
            Console.WriteLine("Failed to save plugin.\n");
    }

    private static void EditPlugin()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins to edit.\n");
            return;
        }

        Console.WriteLine("Select a plugin to edit:");
        for (int i = 0; i < plugins.Count; i++)
            Console.WriteLine($"{i + 1}) {plugins[i].Name}");

        Console.Write("Choice: ");
        if (!int.TryParse(Console.ReadLine(), out int choice) || choice < 1 || choice > plugins.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var plugin = plugins[choice - 1];
        Console.WriteLine($"Editing plugin: {plugin.Name}");
        Console.WriteLine($"Current version: {plugin.Version}");
        Console.Write("Enter new version (or leave blank): ");
        var version = Console.ReadLine()?.Trim();
        if (!string.IsNullOrWhiteSpace(version))
            plugin.Version = version;

        Console.WriteLine("Current actions:");
        for (int i = 0; i < plugin.Actions.Count; i++)
        {
            var a = plugin.Actions[i];
            Console.WriteLine($"{i + 1}) {a.Name} - {a.Description} - RequiresAuth: {a.RequiresAuth}");
        }

        Console.WriteLine("Modify actions? (yes/no)");
        var modify = Console.ReadLine()?.Trim().ToLower();
        if (modify == "yes")
        {
            var newActions = new List<PluginAction>();
            Console.WriteLine("Enter actions (actionName:description:requiresAuth(true/false))");
            Console.WriteLine("Type 'done' when finished.");
            while (true)
            {
                Console.Write("Action> ");
                var line = Console.ReadLine()?.Trim();
                if (string.Equals(line, "done", StringComparison.OrdinalIgnoreCase)) break;
                if (string.IsNullOrWhiteSpace(line)) continue;

                var parts = line.Split(':');
                if (parts.Length < 3)
                {
                    Console.WriteLine("Invalid format.");
                    continue;
                }

                bool requiresAuth = parts[2].Trim().ToLower() == "true";
                newActions.Add(new PluginAction
                {
                    Name = parts[0].Trim(),
                    Description = parts[1].Trim(),
                    RequiresAuth = requiresAuth
                });
            }
            if (newActions.Any())
                plugin.Actions = newActions;
        }

        if (_pluginRepo.Save(plugin))
            Console.WriteLine($"Plugin '{plugin.Name}' updated successfully.\n");
        else
            Console.WriteLine("Failed to update plugin.\n");
    }

    private static void DeletePlugin()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins to delete.\n");
            return;
        }

        Console.WriteLine("Select a plugin to delete:");
        for (int i = 0; i < plugins.Count; i++)
            Console.WriteLine($"{i + 1}) {plugins[i].Name}");

        Console.Write("Choice: ");
        if (!int.TryParse(Console.ReadLine(), out int choice) || choice < 1 || choice > plugins.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var plugin = plugins[choice - 1];

        Console.Write($"Are you sure you want to delete plugin '{plugin.Name}'? (yes/no): ");
        var confirm = Console.ReadLine()?.Trim().ToLower();
        if (confirm == "yes")
        {
            try
            {
                File.Delete(Path.Combine("ai_plugins", $"{plugin.Name}.json"));
                _pluginRepo = new PluginRepository(); // reload
                Console.WriteLine("Plugin deleted.\n");
            }
            catch
            {
                Console.WriteLine("Failed to delete plugin.\n");
            }
        }
        else
        {
            Console.WriteLine("Delete cancelled.\n");
        }
    }

    private static void ShowLogs()
    {
        var logs = _logger.GetLogs(50).ToList();
        if (!logs.Any())
        {
            Console.WriteLine("No logs available.\n");
            return;
        }

        Console.WriteLine("\n--- Recent Logs ---");
        foreach (var line in logs)
            Console.WriteLine(line);
        Console.WriteLine();
    }

    private static void Logout()
    {
        Console.WriteLine("\nLogging out...");
        _currentUser = null;
        Environment.Exit(0);
    }

    private static void AccessDenied()
    {
        Console.WriteLine("❌ Access denied: You do not have sufficient permissions.\n");
    }

    private static string ReadPassword()
    {
        var pass = "";
        ConsoleKeyInfo key;
        do
        {
            key = Console.ReadKey(true);
            if (key.Key == ConsoleKey.Backspace && pass.Length > 0)
            {
                pass = pass[0..^1];
                Console.Write("\b \b");
            }
            else if (!char.IsControl(key.KeyChar))
            {
                pass += key.KeyChar;
                Console.Write("*");
            }
        } while (key.Key != ConsoleKey.Enter);
        Console.WriteLine();
        return pass;
    }
}
using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using System.Security.Cryptography;
using System.Linq;

// Enum for platform detection (simplified)
public enum PlatformType
{
    Universal, Cloud, Edge, Mobile, Desktop, Browser, IoT, Embedded, AISystem
}

public static class Compatibility
{
    public static PlatformType DetectPlatform()
    {
        try
        {
            if (OperatingSystem.IsLinux() && File.Exists("/proc/device-tree/model"))
            {
                var model = File.ReadAllText("/proc/device-tree/model");
                if (model.Contains("arm", StringComparison.OrdinalIgnoreCase))
                    return PlatformType.Embedded;
            }
            if (Environment.GetEnvironmentVariable("AI_SYSTEM") != null)
                return PlatformType.AISystem;

            if (OperatingSystem.IsWindows() || OperatingSystem.IsMacOS())
                return PlatformType.Desktop;

            return PlatformType.Universal;
        }
        catch { return PlatformType.Universal; }
    }

    public static bool CheckCompatibility(PlatformType system, PlatformType plugin) =>
        system == plugin || plugin == PlatformType.Universal;
}

// User & Role Management
public class User
{
    public string Username { get; set; }
    public string PasswordHash { get; set; }
    public string Role { get; set; } = "User"; // Default role
}

public class UserService
{
    private const string UserFile = "users.json";
    private List<User> _users = new();

    public UserService()
    {
        LoadUsers();
    }

    private void LoadUsers()
    {
        if (File.Exists(UserFile))
        {
            var json = File.ReadAllText(UserFile);
            _users = JsonSerializer.Deserialize<List<User>>(json) ?? new List<User>();
        }
        else
        {
            // Create default admin if none exist
            var admin = new User
            {
                Username = "admin",
                PasswordHash = HashPassword("admin123"),
                Role = "Admin"
            };
            _users.Add(admin);
            SaveUsers();
        }
    }

    private void SaveUsers()
    {
        var json = JsonSerializer.Serialize(_users, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(UserFile, json);
    }

    public bool Register(string username, string password)
    {
        if (_users.Any(u => u.Username.Equals(username, StringComparison.OrdinalIgnoreCase)))
            return false;
        var user = new User
        {
            Username = username,
            PasswordHash = HashPassword(password),
            Role = "User"
        };
        _users.Add(user);
        SaveUsers();
        return true;
    }

    public User Authenticate(string username, string password)
    {
        var user = _users.FirstOrDefault(u => u.Username.Equals(username, StringComparison.OrdinalIgnoreCase));
        if (user == null) return null;
        if (VerifyPassword(password, user.PasswordHash))
            return user;
        return null;
    }

    public bool IsInRole(User user, string role) =>
        user != null && user.Role.Equals(role, StringComparison.OrdinalIgnoreCase);

    private static string HashPassword(string password)
    {
        using var sha = SHA256.Create();
        var hashedBytes = sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(password));
        return Convert.ToBase64String(hashedBytes);
    }

    private static bool VerifyPassword(string password, string hashed)
    {
        return HashPassword(password) == hashed;
    }
}

// Plugin system
public class PluginAction
{
    public string Name { get; set; }
    public string Description { get; set; }
    public bool RequiresAuth { get; set; } = true;
}

public class PluginManifest
{
    public string Name { get; set; }
    public string Version { get; set; } = "1.0.0";
    public List<PlatformType> Platforms { get; set; } = new() { PlatformType.Universal };
    public List<PluginAction> Actions { get; set; } = new()
    {
        new PluginAction { Name = "status", Description = "Get plugin status", RequiresAuth = false }
    };
    public string Checksum { get; set; }

    public string CalculateChecksum()
    {
        using var sha = SHA256.Create();
        var props = new[] {
            Name ?? "",
            Version,
            string.Join(",", Platforms.Select(p => p.ToString())),
            string.Join(",", Actions.Select(a => a.Name))
        };
        return Convert.ToBase64String(sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(string.Join("|", props))));
    }
}

public class PluginRepository
{
    private const string PluginDir = "ai_plugins";
    private List<PluginManifest> _plugins = new();

    public PluginRepository()
    {
        if (!Directory.Exists(PluginDir))
            Directory.CreateDirectory(PluginDir);

        LoadPlugins();
    }

    private void LoadPlugins()
    {
        _plugins.Clear();
        foreach (var file in Directory.EnumerateFiles(PluginDir, "*.json"))
        {
            try
            {
                var json = File.ReadAllText(file);
                var plugin = JsonSerializer.Deserialize<PluginManifest>(json);
                if (plugin != null)
                {
                    plugin.Checksum = plugin.CalculateChecksum();
                    _plugins.Add(plugin);
                }
            }
            catch { /* ignore errors */ }
        }
    }

    public IEnumerable<PluginManifest> GetAll() => _plugins;

    public PluginManifest Get(string name) => _plugins.FirstOrDefault(p => p.Name.Equals(name, StringComparison.OrdinalIgnoreCase));

    public bool Save(PluginManifest plugin)
    {
        try
        {
            plugin.Checksum = plugin.CalculateChecksum();
            var json = JsonSerializer.Serialize(plugin, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(Path.Combine(PluginDir, $"{plugin.Name}.json"), json);
            LoadPlugins(); // reload plugins after save
            return true;
        }
        catch { return false; }
    }
}

// Logging service
public class LogService
{
    private const string LogFile = "action_logs.txt";
    private readonly object _lock = new();

    public void Log(string user, string plugin, string action, DateTime timestamp, string result)
    {
        lock (_lock)
        {
            var line = $"{timestamp:O} | {user} | {plugin}:{action} => {result}";
            File.AppendAllLines(LogFile, new[] { line });
        }
    }

    public IEnumerable<string> GetLogs(int limit = 100)
    {
        if (!File.Exists(LogFile)) return Enumerable.Empty<string>();
        var lines = File.ReadLines(LogFile).Reverse().Take(limit).Reverse();
        return lines;
    }
}

// Force-trigger policy enforcing Admin-only commands
public class ForceTriggerPolicy
{
    private readonly UserService _userService;

    public ForceTriggerPolicy(UserService userService)
    {
        _userService = userService;
    }

    public string Execute(User user, string plugin, string action)
    {
        if (!_userService.IsInRole(user, "Admin"))
            return "❌ Access denied: Admin role required to execute commands.";

        // Simulate execution
        return $"✅ Executed '{action}' on plugin '{plugin}' (enforced mode)";
    }
}

// The CLI Main Menu and flow
public static class Program
{
    private static UserService _userService = new();
    private static PluginRepository _pluginRepo = new();
    private static LogService _logger = new();
    private static ForceTriggerPolicy _policy = new(_userService);

    private static User _currentUser = null;

    public static void Main()
    {
        Console.Title = "Universal AI Bootloader CLI";
        Console.WriteLine("Universal AI Bootloader CLI - Cross-platform, Universal Plugin System\n");

        AuthenticateUser();

        while (true)
        {
            ShowMainMenu();

            var input = Console.ReadLine()?.Trim().ToLower() ?? "";
            switch (input)
            {
                case "1": ListPlugins(); break;
                case "2": RunPluginAction(); break;
                case "3": if (_userService.IsInRole(_currentUser, "Admin")) ManagePlugins(); else AccessDenied(); break;
                case "4": ShowLogs(); break;
                case "5": Logout(); return;
                default:
                    Console.WriteLine("Invalid option, try again.");
                    break;
            }
        }
    }

    private static void AuthenticateUser()
    {
        while (_currentUser == null)
        {
            Console.Write("Username: ");
            var username = Console.ReadLine()?.Trim();
            Console.Write("Password: ");
            var password = ReadPassword();

            var user = _userService.Authenticate(username, password);
            if (user != null)
            {
                _currentUser = user;
                Console.WriteLine($"\n✅ Welcome, {_currentUser.Username}! Role: {_currentUser.Role}\n");
            }
            else
            {
                Console.WriteLine("❌ Invalid credentials, please try again.\n");
            }
        }
    }

    private static void ShowMainMenu()
    {
        Console.WriteLine("Main Menu:");
        Console.WriteLine("1) List Plugins");
        Console.WriteLine("2) Run Plugin Action (Admin only for forced commands)");
        if (_userService.IsInRole(_currentUser, "Admin"))
            Console.WriteLine("3) Manage Plugins (Add/Edit)");
        else
            Console.WriteLine("3) Manage Plugins (Admin only)");
        Console.WriteLine("4) View Logs");
        Console.WriteLine("5) Logout & Exit");
        Console.Write("Select an option: ");
    }

    private static void ListPlugins()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins available.\n");
            return;
        }

        Console.WriteLine("\nAvailable Plugins:");
        foreach (var p in plugins)
        {
            Console.WriteLine($"- {p.Name} (v{p.Version}) - Actions: {string.Join(", ", p.Actions.Select(a => a.Name))}");
        }
        Console.WriteLine();
    }

    private static void RunPluginAction()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins available to run.\n");
            return;
        }

        Console.WriteLine("\nSelect a plugin:");
        for (int i = 0; i < plugins.Count; i++)
            Console.WriteLine($"{i + 1}) {plugins[i].Name}");

        Console.Write("Choice: ");
        if (!int.TryParse(Console.ReadLine(), out int pChoice) || pChoice < 1 || pChoice > plugins.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var selectedPlugin = plugins[pChoice - 1];

        Console.WriteLine($"Selected plugin: {selectedPlugin.Name}");
        Console.WriteLine("Available actions:");
        for (int i = 0; i < selectedPlugin.Actions.Count; i++)
            Console.WriteLine($"{i + 1}) {selectedPlugin.Actions[i].Name} - {selectedPlugin.Actions[i].Description}");

        Console.Write("Action choice: ");
        if (!int.TryParse(Console.ReadLine(), out int aChoice) || aChoice < 1 || aChoice > selectedPlugin.Actions.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var selectedAction = selectedPlugin.Actions[aChoice - 1];

        string result;

        // Enforce admin-only forced-trigger commands (simulate only)
        if (selectedAction.RequiresAuth && !_userService.IsInRole(_currentUser, "Admin"))
        {
            result = "❌ Access denied: Action requires Admin role.";
        }
        else
        {
            result = _policy.Execute(_currentUser, selectedPlugin.Name, selectedAction.Name);
        }

        _logger.Log(_currentUser.Username, selectedPlugin.Name, selectedAction.Name, DateTime.UtcNow, result);

        Console.WriteLine($"Result: {result}\n");
    }

    private static void ManagePlugins()
    {
        Console.WriteLine("\n--- Plugin Management ---");
        Console.WriteLine("1) Add New Plugin");
        Console.WriteLine("2) Edit Existing Plugin");
        Console.WriteLine("3) Delete Plugin");
        Console.WriteLine("4) Back to Main Menu");
        Console.Write("Select an option: ");
        var input = Console.ReadLine()?.Trim();

        switch (input)
        {
            case "1":
                AddPlugin();
                break;
            case "2":
                EditPlugin();
                break;
            case "3":
                DeletePlugin();
                break;
            case "4":
                break;
            default:
                Console.WriteLine("Invalid option.\n");
                break;
        }
    }

    private static void AddPlugin()
    {
        Console.Write("Plugin name: ");
        var name = Console.ReadLine()?.Trim();
        if (string.IsNullOrWhiteSpace(name))
        {
            Console.WriteLine("Plugin name cannot be empty.\n");
            return;
        }

        if (_pluginRepo.Get(name) != null)
        {
            Console.WriteLine("Plugin already exists.\n");
            return;
        }

        var plugin = new PluginManifest
        {
            Name = name,
            Version = "1.0.0",
            Platforms = new List<PlatformType> { PlatformType.Universal },
            Actions = new List<PluginAction>()
        };

        Console.WriteLine("Add actions (format: actionName:description:requiresAuth(true/false))");
        Console.WriteLine("Type 'done' when finished.");
        while (true)
        {
            Console.Write("Action> ");
            var line = Console.ReadLine()?.Trim();
            if (string.Equals(line, "done", StringComparison.OrdinalIgnoreCase)) break;
            if (string.IsNullOrWhiteSpace(line)) continue;

            var parts = line.Split(':');
            if (parts.Length < 3)
            {
                Console.WriteLine("Invalid format. Use actionName:description:requiresAuth");
                continue;
            }

            bool requiresAuth = parts[2].Trim().ToLower() == "true";
            plugin.Actions.Add(new PluginAction
            {
                Name = parts[0].Trim(),
                Description = parts[1].Trim(),
                RequiresAuth = requiresAuth
            });
        }

        if (_pluginRepo.Save(plugin))
            Console.WriteLine($"Plugin '{name}' added successfully.\n");
        else
            Console.WriteLine("Failed to save plugin.\n");
    }

    private static void EditPlugin()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins to edit.\n");
            return;
        }

        Console.WriteLine("Select a plugin to edit:");
        for (int i = 0; i < plugins.Count; i++)
            Console.WriteLine($"{i + 1}) {plugins[i].Name}");

        Console.Write("Choice: ");
        if (!int.TryParse(Console.ReadLine(), out int choice) || choice < 1 || choice > plugins.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var plugin = plugins[choice - 1];
        Console.WriteLine($"Editing plugin: {plugin.Name}");
        Console.WriteLine($"Current version: {plugin.Version}");
        Console.Write("Enter new version (or leave blank): ");
        var version = Console.ReadLine()?.Trim();
        if (!string.IsNullOrWhiteSpace(version))
            plugin.Version = version;

        Console.WriteLine("Current actions:");
        for (int i = 0; i < plugin.Actions.Count; i++)
        {
            var a = plugin.Actions[i];
            Console.WriteLine($"{i + 1}) {a.Name} - {a.Description} - RequiresAuth: {a.RequiresAuth}");
        }

        Console.WriteLine("Modify actions? (yes/no)");
        var modify = Console.ReadLine()?.Trim().ToLower();
        if (modify == "yes")
        {
            var newActions = new List<PluginAction>();
            Console.WriteLine("Enter actions (actionName:description:requiresAuth(true/false))");
            Console.WriteLine("Type 'done' when finished.");
            while (true)
            {
                Console.Write("Action> ");
                var line = Console.ReadLine()?.Trim();
                if (string.Equals(line, "done", StringComparison.OrdinalIgnoreCase)) break;
                if (string.IsNullOrWhiteSpace(line)) continue;

                var parts = line.Split(':');
                if (parts.Length < 3)
                {
                    Console.WriteLine("Invalid format.");
                    continue;
                }

                bool requiresAuth = parts[2].Trim().ToLower() == "true";
                newActions.Add(new PluginAction
                {
                    Name = parts[0].Trim(),
                    Description = parts[1].Trim(),
                    RequiresAuth = requiresAuth
                });
            }
            if (newActions.Any())
                plugin.Actions = newActions;
        }

        if (_pluginRepo.Save(plugin))
            Console.WriteLine($"Plugin '{plugin.Name}' updated successfully.\n");
        else
            Console.WriteLine("Failed to update plugin.\n");
    }

    private static void DeletePlugin()
    {
        var plugins = _pluginRepo.GetAll().ToList();
        if (!plugins.Any())
        {
            Console.WriteLine("No plugins to delete.\n");
            return;
        }

        Console.WriteLine("Select a plugin to delete:");
        for (int i = 0; i < plugins.Count; i++)
            Console.WriteLine($"{i + 1}) {plugins[i].Name}");

        Console.Write("Choice: ");
        if (!int.TryParse(Console.ReadLine(), out int choice) || choice < 1 || choice > plugins.Count)
        {
            Console.WriteLine("Invalid choice.\n");
            return;
        }

        var plugin = plugins[choice - 1];

        Console.Write($"Are you sure you want to delete plugin '{plugin.Name}'? (yes/no): ");
        var confirm = Console.ReadLine()?.Trim().ToLower();
        if (confirm == "yes")
        {
            try
            {
                File.Delete(Path.Combine("ai_plugins", $"{plugin.Name}.json"));
                _pluginRepo = new PluginRepository(); // reload
                Console.WriteLine("Plugin deleted.\n");
            }
            catch
            {
                Console.WriteLine("Failed to delete plugin.\n");
            }
        }
        else
        {
            Console.WriteLine("Delete cancelled.\n");
        }
    }

    private static void ShowLogs()
    {
        var logs = _logger.GetLogs(50).ToList();
        if (!logs.Any())
        {
            Console.WriteLine("No logs available.\n");
            return;
        }

        Console.WriteLine("\n--- Recent Logs ---");
        foreach (var line in logs)
            Console.WriteLine(line);
        Console.WriteLine();
    }

    private static void Logout()
    {
        Console.WriteLine("\nLogging out...");
        _currentUser = null;
        Environment.Exit(0);
    }

    private static void AccessDenied()
    {
        Console.WriteLine("❌ Access denied: You do not have sufficient permissions.\n");
    }

    private static string ReadPassword()
    {
        var pass = "";
        ConsoleKeyInfo key;
        do
        {
            key = Console.ReadKey(true);
            if (key.Key == ConsoleKey.Backspace && pass.Length > 0)
            {
                pass = pass[0..^1];
                Console.Write("\b \b");
            }
            else if (!char.IsControl(key.KeyChar))
            {
                pass += key.KeyChar;
                Console.Write("*");
            }
        } while (key.Key != ConsoleKey.Enter);
        Console.WriteLine();
        return pass;
    }
}










 // API_Server/Program.cs
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;
using System.Collections.Generic;
using System.Linq;
using Microsoft.AspNetCore.Http;
using System.Threading.Tasks;
using System.Text.Json;
using CLI_Core; // Reference to your CLI core library

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<UserService>();
builder.Services.AddSingleton<PluginService>(); // Wrap your plugin management
builder.Services.AddEndpointsApiExplorer();

var app = builder.Build();

app.UseRouting();
app.UseAuthentication();
app.UseAuthorization();

// Basic simple auth middleware (replace with JWT for production)
app.Use(async (context, next) =>
{
    // Simple header check for token (in prod use JWT or OAuth)
    if (!context.Request.Headers.TryGetValue("X-Auth-Token", out var token) || token != "admin-token")
    {
        context.Response.StatusCode = 401;
        await context.Response.WriteAsync("Unauthorized");
        return;
    }
    await next();
});

app.MapGet("/api/plugins", (PluginService plugins) =>
{
    return plugins.GetAllPlugins();
});

app.MapPost("/api/plugins/{name}/actions/{actionName}", async (string name, string actionName, PluginService plugins) =>
{
    var result = await plugins.ExecutePluginActionAsync(name, actionName);
    return Results.Ok(new { success = result });
});

app.MapGet("/api/logs", (PluginService plugins) =>
{
    return plugins.GetLogs();
});

app.MapPost("/api/login", async (HttpRequest req, UserService users) =>
{
    var creds = await JsonSerializer.DeserializeAsync<LoginRequest>(req.Body);
    var token = users.Authenticate(creds.Username, creds.Password);
    if (token == null)
        return Results.Unauthorized();
    return Results.Ok(new { token });
});

app.Run();

public record LoginRequest(string Username, string Password);

// Dummy PluginService (implement properly wrapping your core system)
public class PluginService
{
    public List<string> GetAllPlugins() => new() { "ai_core", "platform_adapter" };
    public Task<bool> ExecutePluginActionAsync(string plugin, string action) => Task.FromResult(true);
    public List<string> GetLogs() => new() { "Log entry 1", "Log entry 2" };
}

public class UserService
{
    public string Authenticate(string username, string password)
    {
        if (username == "admin" && password == "admin123")
            return "admin-token";
        return null;
    }
}
// GUI_App/MainWindow.axaml.cs
using Avalonia.Controls;
using Avalonia.Interactivity;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;

public partial class MainWindow : Window
{
    private HttpClient _http = new HttpClient();
    private string _token;

    public MainWindow()
    {
        InitializeComponent();
    }

    private async void LoginButton_Click(object sender, RoutedEventArgs e)
    {
        var username = UsernameTextBox.Text;
        var password = PasswordBox.Password;

        var response = await _http.PostAsync("http://localhost:5000/api/login",
            new StringContent(JsonSerializer.Serialize(new { Username = username, Password = password }),
            System.Text.Encoding.UTF8, "application/json"));

        if (response.IsSuccessStatusCode)
        {
            var json = await response.Content.ReadAsStringAsync();
            var obj = JsonSerializer.Deserialize<LoginResponse>(json);
            _token = obj.token;
            StatusTextBlock.Text = "Login successful!";
            await LoadPluginsAsync();
        }
        else
        {
            StatusTextBlock.Text = "Login failed.";
        }
    }

    private async Task LoadPluginsAsync()
    {
        _http.DefaultRequestHeaders.Clear();
        _http.DefaultRequestHeaders.Add("X-Auth-Token", _token);
        var response = await _http.GetAsync("http://localhost:5000/api/plugins");
        if (response.IsSuccessStatusCode)
        {
            var json = await response.Content.ReadAsStringAsync();
            var plugins = JsonSerializer.Deserialize<List<string>>(json);
            PluginsListBox.Items = plugins;
        }
    }

    private async void ExecuteActionButton_Click(object sender, RoutedEventArgs e)
    {
        if (PluginsListBox.SelectedItem is string plugin)
        {
            _http.DefaultRequestHeaders.Clear();
            _http.DefaultRequestHeaders.Add("X-Auth-Token", _token);
            var response = await _http.PostAsync($"http://localhost:5000/api/plugins/{plugin}/actions/execute", null);
            StatusTextBlock.Text = response.IsSuccessStatusCode ? "Action executed" : "Failed";
        }
    }

    private record LoginResponse(string token);
}
/UniversalAIBootloaderSolution
  /CLI_Core             -- Your CLI logic as .NET Standard library
  /API_Server           -- ASP.NET Core Web API
  /GUI_App              -- Avalonia UI desktop app


using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace CLI_Core
{
    public class PluginService
    {
        // Load plugins from disk, cache them
        public List<PluginManifest> GetPlugins() { /* ... */ }
        
        // Execute action by plugin name & action name
        public Task<bool> ExecutePluginActionAsync(string pluginName, string actionName) { /* ... */ }
        
        // Logs storage & retrieval
        public List<string> GetLogs() { /* ... */ }
        
        // Add logging helper
        public void LogAction(string user, string action, string plugin) { /* ... */ }
    }
}
using CLI_Core;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<UserService>();     // your user auth manager
builder.Services.AddSingleton<PluginService>();   // core CLI logic

var app = builder.Build();

app.UseRouting();

// Simple token auth middleware (for demo purposes)
app.Use(async (ctx, next) =>
{
    if (!ctx.Request.Headers.TryGetValue("X-Auth-Token", out var token) || token != "admin-token")
    {
        ctx.Response.StatusCode = 401;
        await ctx.Response.WriteAsync("Unauthorized");
        return;
    }
    await next();
});

app.MapGet("/api/plugins", (PluginService svc) => svc.GetPlugins());

app.MapPost("/api/plugins/{plugin}/actions/{action}", async (string plugin, string action, PluginService svc) =>
{
    var result = await svc.ExecutePluginActionAsync(plugin, action);
    return Results.Ok(new { success = result });
});

app.MapGet("/api/logs", (PluginService svc) => svc.GetLogs());

app.Run();
using Avalonia.Controls;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;
using System.Collections.Generic;

public partial class MainWindow : Window
{
    private HttpClient _client = new HttpClient();
    private string _token = "admin-token";  // hardcoded or login-based

    public MainWindow()
    {
        InitializeComponent();
        LoadPlugins();
    }

    private async Task LoadPlugins()
    {
        _client.DefaultRequestHeaders.Clear();
        _client.DefaultRequestHeaders.Add("X-Auth-Token", _token);
        var response = await _client.GetAsync("http://localhost:5000/api/plugins");
        if (response.IsSuccessStatusCode)
        {
            var json = await response.Content.ReadAsStringAsync();
            var plugins = JsonSerializer.Deserialize<List<PluginManifest>>(json);
            PluginsListBox.Items = plugins;
        }
    }

    private async void ExecuteButton_Click(object sender, Avalonia.Interactivity.RoutedEventArgs e)
    {
        if (PluginsListBox.SelectedItem is PluginManifest plugin)
        {
            var actionName = "execute"; // or user-selected
            var response = await _client.PostAsync($"http://localhost:5000/api/plugins/{plugin.Name}/actions/{actionName}", null);
            // Show success/fail
        }
    }
}
UniversalAIBootloaderSolution/
├── CLI_Core/
│   ├── CLI_Core.csproj
│   ├── PluginManifest.cs
│   ├── PluginService.cs
│   └── UserService.cs
├── API_Server/
│   ├── API_Server.csproj
│   └── Program.cs
└── GUI_App/
    ├── GUI_App.csproj
    ├── MainWindow.axaml
    └── MainWindow.axaml.cs
using System;
using System.Collections.Generic;
using System.Security.Cryptography;
using System.Text;

namespace CLI_Core
{
    public enum PlatformType
    {
        Universal, Cloud, Edge, Mobile, Desktop, Browser, IoT, Embedded, AISystem
    }

    public class PluginAction
    {
        public string Name { get; set; }
        public string Description { get; set; }
        public bool RequiresAuth { get; set; } = true;
    }

    public class PluginManifest
    {
        public string Name { get; set; }
        public string Version { get; set; } = "1.0.0";
        public List<PlatformType> Platforms { get; set; } = new() { PlatformType.Universal };
        public List<PluginAction> Actions { get; set; } = new() {
            new PluginAction { Name = "status", Description = "Get plugin status", RequiresAuth = false }
        };

        public string CalculateChecksum()
        {
            using var sha = SHA256.Create();
            var props = $"{Name}|{Version}|{string.Join(",", Platforms)}";
            return Convert.ToBase64String(sha.ComputeHash(Encoding.UTF8.GetBytes(props)));
        }
    }
}
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;

namespace CLI_Core
{
    public class PluginService
    {
        private readonly string _pluginDir;
        private readonly List<PluginManifest> _plugins = new();

        private readonly List<string> _logs = new();

        public PluginService(string pluginDirectory = null)
        {
            _pluginDir = pluginDirectory ?? Path.Combine(AppContext.BaseDirectory, "ai_plugins");
            Directory.CreateDirectory(_pluginDir);
            LoadPlugins();
        }

        private void LoadPlugins()
        {
            foreach (var file in Directory.EnumerateFiles(_pluginDir, "*.json"))
            {
                try
                {
                    var json = File.ReadAllText(file);
                    var manifest = JsonSerializer.Deserialize<PluginManifest>(json);
                    if (manifest != null)
                    {
                        var checksum = manifest.CalculateChecksum();
                        _plugins.Add(manifest);
                        LogAction("system", $"Loaded plugin {manifest.Name}", "plugin");
                    }
                }
                catch (Exception ex)
                {
                    LogAction("system", $"Failed to load plugin file {file}: {ex.Message}", "plugin");
                }
            }
        }

        public List<PluginManifest> GetPlugins() => _plugins;

        public Task<bool> ExecutePluginActionAsync(string pluginName, string actionName)
        {
            var plugin = _plugins.FirstOrDefault(p => p.Name.Equals(pluginName, StringComparison.OrdinalIgnoreCase));
            if (plugin == null)
            {
                LogAction("system", $"Plugin '{pluginName}' not found", "plugin");
                return Task.FromResult(false);
            }

            var action = plugin.Actions.FirstOrDefault(a => a.Name.Equals(actionName, StringComparison.OrdinalIgnoreCase));
            if (action == null)
            {
                LogAction("system", $"Action '{actionName}' not found in plugin '{pluginName}'", pluginName);
                return Task.FromResult(false);
            }

            // Real plugin action execution logic would go here.
            LogAction("admin", $"Executed action '{actionName}' on plugin '{pluginName}'", pluginName);
            return Task.FromResult(true);
        }

        public List<string> GetLogs() => _logs.ToList();

        public void LogAction(string user, string action, string plugin)
        {
            var log = $"{DateTime.UtcNow:u} | User:{user} | Plugin:{plugin} | Action:{action}";
            _logs.Add(log);
        }
    }
}
using System.Collections.Generic;

namespace CLI_Core
{
    public class UserService
    {
        private readonly Dictionary<string, string> _users = new()
        {
            { "admin", "admin123" } // Passwords must be hashed in production!
        };

        public bool ValidateUser(string username, string password)
        {
            return _users.TryGetValue(username, out var storedPassword) && storedPassword == password;
        }
    }
}
<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <TargetFramework>netstandard2.0</TargetFramework>
    <Nullable>enable</Nullable>
  </PropertyGroup>

</Project>
using CLI_Core;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.AspNetCore.Http;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton<UserService>();
builder.Services.AddSingleton<PluginService>();

var app = builder.Build();

app.UseRouting();

app.Use(async (context, next) =>
{
    if (!context.Request.Headers.TryGetValue("X-Auth-Token", out var token) || token != "admin-token")
    {
        context.Response.StatusCode = 401;
        await context.Response.WriteAsync("Unauthorized");
        return;
    }
    await next();
});

app.MapGet("/api/plugins", (PluginService svc) => svc.GetPlugins());

app.MapPost("/api/plugins/{plugin}/actions/{action}", async (string plugin, string action, PluginService svc) =>
{
    var result = await svc.ExecutePluginActionAsync(plugin, action);
    return Results.Ok(new { success = result });
});

app.MapGet("/api/logs", (PluginService svc) => svc.GetLogs());

app.MapPost("/api/login", async (HttpRequest req, UserService users) =>
{
    var creds = await System.Text.Json.JsonSerializer.DeserializeAsync<LoginRequest>(req.Body);
    if (creds == null || !users.ValidateUser(creds.Username, creds.Password))
        return Results.Unauthorized();

    return Results.Ok(new { token = "admin-token" });
});

app.Run();

public record LoginRequest(string Username, string Password);
<Project Sdk="Microsoft.NET.Sdk.Web">

  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
    <Nullable>enable</Nullable>
  </PropertyGroup>

  <ItemGroup>
    <ProjectReference Include="..\CLI_Core\CLI_Core.csproj" />
  </ItemGroup>

</Project>
<Window xmlns="https://github.com/avaloniaui"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        x:Class="GUI_App.MainWindow"
        Width="600" Height="400"
        Title="Universal AI Bootloader GUI">
  <StackPanel Margin="20" Spacing="10">
    <TextBox x:Name="UsernameTextBox" Watermark="Username" />
    <PasswordBox x:Name="PasswordBox" Watermark="Password" />
    <Button Content="Login" Click="LoginButton_Click" />
    <TextBlock x:Name="StatusTextBlock" />
    <ListBox x:Name="PluginsListBox" Height="200" DisplayMemberPath="Name" />
    <Button Content="Execute 'execute' Action" Click="ExecuteActionButton_Click" />
  </StackPanel>
</Window>
using Avalonia.Controls;
using Avalonia.Interactivity;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;
using System.Collections.Generic;

namespace GUI_App
{
    public partial class MainWindow : Window
    {
        private HttpClient _http = new();
        private string _token;

        public MainWindow()
        {
            InitializeComponent();
        }

        private async void LoginButton_Click(object? sender, RoutedEventArgs e)
        {
            var username = UsernameTextBox.Text;
            var password = PasswordBox.Password;

            var creds = new { Username = username, Password = password };
            var content = new StringContent(JsonSerializer.Serialize(creds), System.Text.Encoding.UTF8, "application/json");

            var response = await _http.PostAsync("http://localhost:5000/api/login", content);
            if (response.IsSuccessStatusCode)
            {
                var json = await response.Content.ReadAsStringAsync();
                var obj = JsonSerializer.Deserialize<LoginResponse>(json);
                _token = obj.token;
                StatusTextBlock.Text = "Login successful!";
                await LoadPluginsAsync();
            }
            else
            {
                StatusTextBlock.Text = "Login failed!";
            }
        }

        private async Task LoadPluginsAsync()
        {
            _http.DefaultRequestHeaders.Clear();
            _http.DefaultRequestHeaders.Add("X-Auth-Token", _token);
            var response = await _http.GetAsync("http://localhost:5000/api/plugins");
            if (response.IsSuccessStatusCode)
            {
                var json = await response.Content.ReadAsStringAsync();
                var plugins = JsonSerializer.Deserialize<List<PluginManifest>>(json);
                PluginsListBox.Items = plugins;
            }
            else
            {
                StatusTextBlock.Text = "Failed to load plugins.";
            }
        }

        private async void ExecuteActionButton_Click(object? sender, RoutedEventArgs e)
        {
            if (PluginsListBox.SelectedItem is PluginManifest plugin)
            {
                _http.DefaultRequestHeaders.Clear();
                _http.DefaultRequestHeaders.Add("X-Auth-Token", _token);
                var response = await _http.PostAsync($"http://localhost:5000/api/plugins/{plugin.Name}/actions/execute", null);
                StatusTextBlock.Text = response.IsSuccessStatusCode ? "Action executed!" : "Execution failed!";
            }
            else
            {
                StatusTextBlock.Text = "Select a plugin first.";
            }
        }

        private record LoginResponse(string token);

        public record PluginManifest(string Name, string Version);
    }
}
<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <TargetFramework>net7.0</TargetFramework>
    <Nullable>enable</Nullable>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include="Avalonia" Version="11.0.0-preview6" />
    <PackageReference Include="Avalonia.Desktop" Version="11.0.0-preview6" />
    <PackageReference Include="System.Text.Json" Version="7.0.2" />
  </ItemGroup>

  <ItemGroup>
    <ProjectReference Include="..\CLI_Core\CLI_Core.csproj" />
  </ItemGroup>

</Project>
cd CLI_Core













cd ../API_Server
dotnet run
cd ../GUI_App
dotnet run


// === HARDWARE/SYSTEM CONSTANTS ===
public static class HardwareConstants
{
    public const string CPU_VENDOR_INTEL = "Intel(R) Corporation";
    public const string CPU_VENDOR_AMD = "Advanced Micro Devices, Inc.";
    public const string GPU_VENDOR_NVIDIA = "NVIDIA Corporation";
    public const string GPU_VENDOR_AMD = "Advanced Micro Devices, Inc.";
    public const string STORAGE_SSD = "SSD";
    public const string STORAGE_HDD = "HDD";
    public const string RAM_TYPE_DDR4 = "DDR4";
    public const string RAM_TYPE_DDR5 = "DDR5";
    public const string OS_LINUX = "Linux";
    public const string OS_WINDOWS = "Windows";
    public const string OS_MACOS = "macOS";
}

// === SYSTEM METRICS ===
public class SystemMetrics
{
    public double CpuUsage { get; set; }
    public double MemoryUsage { get; set; }
    public double StorageUsage { get; set; }
    public double Temperature { get; set; } // Celsius
    public string LastBootTime { get; set; }
    public Dictionary<string, string> NetworkInterfaces { get; set; } = new Dictionary<string, string>();
}
// === SECURITY ENHANCEMENTS ===
public static class Security
{
    public static bool VerifyBootSignature()
    {
        try
        {
            // Simulate secure boot verification using cryptographic hashing
            string expectedHash = "a1b2c3d4e5f67890";
            string actualHash = ComputeSHA256Hash("/boot/loader.bin");
            return actualHash == expectedHash;
        }
        catch
        {
            return false;
        }
    }

    public static string ComputeSHA256Hash(string filePath)
    {
        using (var stream = File.OpenRead(filePath))
        {
            var hash = SHA256.Create().ComputeHash(stream);
            return BitConverter.ToString(hash).Replace("-", "").ToLowerInvariant();
        }
    }

    public static bool DetectCodeReproduction(CommandType type)
    {
        // Block commands that could lead to code extraction
        switch (type)
        {
            case CommandType.Developer:
            case CommandType.Admin:
                return true;
            default:
                return false;
        }
    }

    public static void EnforceSecureBoot()
    {
        if (!VerifyBootSignature())
        {
            Display.Error("Secure Boot verification failed. System halted.");
            SystemControl.Halt();
        }
    }
}
// === PLUGIN VERSIONING & DEPENDENCIES ===
public class PluginMetadata
{
    public string Name { get; set; }
    public string Version { get; set; }
    public string Author { get; set; }
    public List<string> Dependencies { get; set; } = new List<string>();
    public string Description { get; set; }
    public bool IsVerified { get; set; } = false;
}

public static class PluginManager
{
    private static Dictionary<string, PluginMetadata> pluginRegistry = new Dictionary<string, PluginMetadata>();

    public static void LoadPlugins()
    {
        try
        {
            var pluginDir = "/boot/plugins";
            if (!Directory.Exists(pluginDir)) Directory.CreateDirectory(pluginDir);

            foreach (var file in Directory.GetFiles(pluginDir, "*.json"))
            {
                var json = File.ReadAllText(file);
                var meta = JsonConvert.DeserializeObject<PluginMetadata>(json);
                pluginRegistry[meta.Name] = meta;
                Display.Log($"Loaded plugin: {meta.Name} v{meta.Version}");
            }
        }
        catch (Exception ex)
        {
            Display.Error($"Plugin load failed: {ex.Message}");
        }
    }

    public static PluginMetadata GetPlugin(string name)
    {
        return pluginRegistry.ContainsKey(name) ? pluginRegistry[name] : null;
    }
}
// === DYNAMIC CONFIGURATION ===
public class Config
{
    public Dictionary<string, object> Settings { get; set; } = new Dictionary<string, object>
    {
        { "LogLevel", "INFO" },
        { "AutoRebootOnCrash", true },
        { "MaxMemoryUsage", 90 }, // Percent
        { "PreferredLanguage", "en-US" },
        { "NetworkTimeout", 30 }, // Seconds
        { "EnableDebugMode", false },
        { "AllowedIPs", new List<string> { "192.168.1.0/24", "10.0.0.0/8" } }
    };

    public T Get<T>(string key, T defaultValue = default)
    {
        return Settings.ContainsKey(key) ? (T)Convert.ChangeType(Settings[key], typeof(T)) : defaultValue;
    }

    public void Set(string key, object value)
    {
        Settings[key] = value;
        SaveConfig();
    }

    private void SaveConfig()
    {
        File.WriteAllText("/boot/config.json", JsonConvert.SerializeObject(Settings, Formatting.Indented));
    }
}
// === SYSTEM DIAGNOSTICS ===
public class Diagnostics
{
    public static SystemMetrics GetSystemMetrics()
    {
        var metrics = new SystemMetrics
        {
            CpuUsage = GetCpuUsage(),
            MemoryUsage = GetMemoryUsage(),
            StorageUsage = GetStorageUsage(),
            Temperature = GetTemperature(),
            LastBootTime = GetLastBootTime()
        };

        metrics.NetworkInterfaces = GetNetworkInterfaces();
        return metrics;
    }

    private static double GetCpuUsage()
    {
        // Simulate CPU usage
        return new Random().NextDouble() * 100;
    }

    private static double GetMemoryUsage()
    {
        return new Random().Next(30, 90);
    }

    private static double GetStorageUsage()
    {
        return new Random().Next(40, 85);
    }

    private static double GetTemperature()
    {
        return new Random().Next(30, 75); // Celsius
    }

    private static string GetLastBootTime()
    {
        return DateTime.Now.AddHours(-2).ToString("yyyy-MM-dd HH:mm:ss");
    }

    private static Dictionary<string, string> GetNetworkInterfaces()
    {
        return new Dictionary<string, string>
        {
            { "eth0", "192.168.1.100" },
            { "lo", "127.0.0.1" }
        };
    }
}
// === DYNAMIC MENU GENERATION ===
public static class MenuBuilder
{
    public static MenuNode BuildRootMenu()
    {
        var root = new MenuNode("Main Menu");

        // Add dynamic diagnostics menu
        var diagnostics = new MenuNode("System Diagnostics", new MenuCommand(CommandType.Diagnostics));
        diagnostics.AddChild(new MenuNode("CPU/Memory", new MenuCommand(CommandType.Diagnostics)));
        diagnostics.AddChild(new MenuNode("Storage", new MenuCommand(CommandType.Diagnostics)));
        diagnostics.AddChild(new MenuNode("Network", new MenuCommand(CommandType.Diagnostics)));
        root.AddChild(diagnostics);

        // Add plugin management menu
        var plugins = new MenuNode("Plugins", new MenuCommand(CommandType.OpenSubMenu));
        plugins.AddChild(new MenuNode("List Installed", new MenuCommand(CommandType.OpenSubMenu)));
        plugins.AddChild(new MenuNode("Install New", new MenuCommand(CommandType.OpenSubMenu)));
        plugins.AddChild(new MenuNode("Update All", new MenuCommand(CommandType.OpenSubMenu)));
        root.AddChild(plugins);

        return root;
    }
}
// === ERROR RECOVERY ===
public class ErrorHandler
{
    public static void HandleException(Exception ex)
    {
        LogSystem.Error($"Unhandled exception: {ex.Message}\nStack Trace: {ex.StackTrace}");
        if (Config.Get<bool>("AutoRebootOnCrash"))
        {
            Display.Error("System crash detected. Rebooting in 10 seconds...");
            Thread.Sleep(10000);
            SystemControl.Reboot();
        }
        else
        {
            Display.Error("System halted due to critical error.");
            SystemControl.Halt();
        }
    }

    public static void CheckSystemHealth()
    {
        var metrics = Diagnostics.GetSystemMetrics();
        if (metrics.CpuUsage > 95 || metrics.MemoryUsage > 95)
        {
            LogSystem.Warn("High resource usage detected. Initiating cleanup...");
            // Trigger garbage collection or process termination
        }
    }
}
// === STRUCTURED LOGGING ===
public static class LogSystem
{
    private static readonly string LogFile = "/boot/bootlog.json";
    public enum LogLevel { DEBUG, INFO, WARNING, ERROR }

    public static void Log(LogLevel level, string message)
    {
        var entry = new
        {
            Timestamp = DateTime.Now,
            Level = level.ToString(),
            Message = message,
            Context = new
            {
                CpuUsage = Diagnostics.GetSystemMetrics().CpuUsage,
                MemoryUsage = Diagnostics.GetSystemMetrics().MemoryUsage
            }
        };

        try
        {
            var json = JsonConvert.SerializeObject(entry, Formatting.Indented);
            File.AppendAllText(LogFile, json + Environment.NewLine);
        }
        catch { /* Silent failure */ }
    }

    public static void RotateLogs()
    {
        try
        {
            var fileInfo = new FileInfo(LogFile);
            if (fileInfo.Length > 1024 * 1024 * 5) // 5MB
            {
                File.Move(LogFile, $"{LogFile}.{DateTime.Now:yyyyMMddHHmmss}");
                File.WriteAllText(LogFile, "");
                Log(LogLevel.INFO, "Log rotated due to size limit.");
            }
        }
        catch { /* Silent failure */ }
    }
}
// === DRIVER ABSTRACTION ===
public abstract class HardwareDriver
{
    public abstract void Initialize();
    public abstract void Diagnose();
}

public class LinuxStorageDriver : HardwareDriver
{
    public override void Initialize()
    {
        // Linux-specific storage init
    }

    public override void Diagnose()
    {
        Display.Log("Linux Storage: Healthy");
    }
}

public class WindowsStorageDriver : HardwareDriver
{
    public override void Initialize()
    {
        // Windows-specific storage init
    }

    public override void Diagnose()
    {
        Display.Log("Windows Storage: Healthy");
    }
}
// === SAMPLE PLUGIN ===
public class SamplePlugin : IPlugin
{
    public string Name => "SamplePlugin";
    public string Version => "1.0.0";
    public bool IsVerified => true;

    public void RegisterCommands(MenuNode rootMenu)
    {
        var pluginMenu = new MenuNode("Sample Plugin", new MenuCommand(CommandType.OpenSubMenu));
        pluginMenu.AddChild(new MenuNode("Run Test", new MenuCommand(CommandType.OpenSubMenu)));
        rootMenu.AddChild(pluginMenu);
    }

    public void Initialize()
    {
        LogSystem.Log(LogLevel.INFO, "SamplePlugin initialized.");
    }

    public void Shutdown()
    {
        LogSystem.Log(LogLevel.INFO, "SamplePlugin shutdown.");
    }
}
// === INTEGRATION POINTS ===
// In MinimalLoader.Main():
Security.EnforceSecureBoot();
Config.LoadConfiguration(); // Load config from /boot/config.json
LogSystem.RotateLogs();

// In IntermediateLoader.Launch():
DriverManager.InitializeDrivers(); // Platform-specific driver init
Diagnostics.GetSystemMetrics(); // Pre-launch health check

// In SystemMenuShell.ExecuteMenuCommand():
if (command.Type == CommandType.Diagnostics)
{
    var metrics = Diagnostics.GetSystemMetrics();
    Display.PrintLine($"CPU Usage: {metrics.CpuUsage}%");
    Display.PrintLine($"Memory Usage: {metrics.MemoryUsage}%");
    Display.PrintLine($"Storage Usage: {metrics.StorageUsage}%");
}
// === LOOKUP TABLES ===
public static class LookupTables
{
    public static readonly Dictionary<string, string> ErrorCodeMessages = new Dictionary<string, string>
    {
        { "E001", "Invalid configuration file format." },
        { "E002", "Plugin verification failed." },
        { "E003", "Driver initialization error." },
        { "E004", "Secure Boot signature mismatch." }
    };

    public static readonly List<string> SupportedLanguages = new List<string>
    {
        "en-US", "es-ES", "fr-FR", "de-DE", "ja-JP", "zh-CN"
    };
}
// === UTILITY FUNCTIONS ===
public static class Utils
{
    public static string FormatSize(double bytes)
    {
        string[] sizes = { "B", "KB", "MB", "GB", "TB" };
        int order = 0;
        while (bytes >= 1024 && order < sizes.Length - 1)
        {
            bytes /= 1024;
            order++;
        }
        return $"{bytes:0.##} {sizes[order]}";
    }

    public static bool IsIpAddressValid(string ip)
    {
        var parts = ip.Split('.');
        if (parts.Length != 4) return false;
        foreach (var part in parts)
        {
            if (!int.TryParse(part, out int num) || num < 0 || num > 255)
                return false;
        }
        return true;
    }
}
// === SHELL COMMANDS ===
public static class ShellCommands
{
    public static void Reboot()
    {
        if (Config.Get<bool>("AutoRebootOnCrash"))
        {
            Display.Log("Rebooting system...");
            SystemControl.Reboot();
        }
    }

    public static void ListPlugins()
    {
        var plugins = PluginManager.ListPlugins();
        foreach (var plugin in plugins)
        {
            Display.PrintLine($"{plugin.Name} v{plugin.Version} ({plugin.Author})");
        }
    }
}
// === FINAL INITIALIZATION ===
public static class SystemInitializer
{
    public static void Initialize()
    {
        Security.EnforceSecureBoot();
        Config.LoadConfiguration();
        LogSystem.RotateLogs();
        DriverManager.InitializeDrivers();
        PluginManager.LoadPlugins();
        Diagnostics.CheckSystemHealth();
        Display.Banner("=== UNIVERSAL AI SYSTEM READY ===");
    }
}
namespace UniversalAISystemBoot
{
    // ==== STAGE 1: Minimal Loader ====
    class MinimalLoader
    {
        static void Main()
        {
            Platform.Detect();
            Hardware.Init();
            Display.Banner("=== UNIVERSAL AI SYSTEM BOOT v1.0 ===");
            if (!Security.VerifyBootSignature()) {
                Display.Error("Integrity check failed. System halted.");
                SystemControl.Halt();
            }
            Capability.Adapt();
            IntermediateLoader.Launch();
        }
    }

    // ==== STAGE 2: Intermediate Loader ====
    static class IntermediateLoader
    {
        public static void Launch()
        {
            Memory.Setup();
            SystemMenuShell.Start();
        }
    }

    // ==== STAGE 3: Menu Shell (Strictly Menu-Driven) ====
    static class SystemMenuShell
    {
        private static MenuNode RootMenu = MenuBuilder.BuildRootMenu();

        public static void Start()
        {
            MenuNode current = RootMenu;
            Stack<MenuNode> history = new Stack<MenuNode>();

            while (true)
            {
                Display.Menu(current);
                string input = Input.GetMenuSelection(current);

                if (input == "EXIT" && history.Count > 0)
                {
                    current = history.Pop();
                    continue;
                }

                MenuNode selected = current.GetChild(input);

                if (selected == null)
                {
                    Display.Error("Invalid selection. Please choose a valid menu item.");
                    continue;
                }

                if (selected.IsLeaf)
                {
                    ExecuteMenuCommand(selected.Command);
                }
                else
                {
                    history.Push(current);
                    current = selected;
                }
            }
        }

        private static void ExecuteMenuCommand(MenuCommand command)
        {
            switch (command.Type)
            {
                case CommandType.OpenSubMenu: break;
                case CommandType.SystemInfo: Display.SystemInfo(); break;
                case CommandType.Settings: Display.SettingsMenu(); break;
                case CommandType.Diagnostics: Display.Diagnostics(); break;
                case CommandType.Help: Display.Help(); break;
                case CommandType.Accessibility: Display.AccessibilityMenu(); break;
                case CommandType.Network: Display.NetworkMenu(); break;
                case CommandType.User: Display.UserMenu(); break;
                case CommandType.Admin: Display.AdminMenu(); break;
                case CommandType.Developer: Display.DevMenu(); break;
                case CommandType.Integrations: Display.IntegrationsMenu(); break;
                case CommandType.Tools: Display.ToolsMenu(); break;
                case CommandType.Data: Display.DataMenu(); break;
                case CommandType.MLLogics: Display.MLLogicsMenu(); break;
                case CommandType.AgenticPatterns: Display.AgenticPatternsMenu(); break;
                case CommandType.BootstrapSequence: Display.BootstrapSequenceMenu(); break;
                case CommandType.Reboot: SystemControl.Reboot(); break;
                case CommandType.Shutdown: SystemControl.Shutdown(); break;
                default: Display.Error("Unauthorized command."); break;
            }
        }
    }

    // ==== MENU STRUCTURE DEFINITIONS ====
    static class MenuBuilder
    {
        public static MenuNode BuildRootMenu()
        {
            var root = new MenuNode("Main Menu");

            // Core System
            root.AddChild(new MenuNode("System Info", new MenuCommand(CommandType.SystemInfo)));
            root.AddChild(new MenuNode("Settings", new MenuCommand(CommandType.Settings)));
            root.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.Diagnostics)));
            root.AddChild(new MenuNode("Help", new MenuCommand(CommandType.Help)));
            root.AddChild(new MenuNode("Reboot", new MenuCommand(CommandType.Reboot)));
            root.AddChild(new MenuNode("Shutdown", new MenuCommand(CommandType.Shutdown)));

            // Accessibility
            var accessibility = new MenuNode("Accessibility", new MenuCommand(CommandType.Accessibility));
            accessibility.AddChild(new MenuNode("Screen Reader", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("High Contrast", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Keyboard Navigation", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Voice Control", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(accessibility);

            // Network
            var network = new MenuNode("Network", new MenuCommand(CommandType.Network));
            network.AddChild(new MenuNode("Status", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("WiFi", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Proxy", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(network);

            // User
            var user = new MenuNode("User", new MenuCommand(CommandType.User));
            user.AddChild(new MenuNode("Profile", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Preferences", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Notifications", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(user);

            // Admin
            var admin = new MenuNode("Admin", new MenuCommand(CommandType.Admin));
            admin.AddChild(new MenuNode("User Management", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("System Logs", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("Security", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(admin);

            // Developer
            var dev = new MenuNode("Developer", new MenuCommand(CommandType.Developer));
            dev.AddChild(new MenuNode("API Explorer", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Debug Tools", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Test Harness", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(dev);

            // Integrations
            var integrations = new MenuNode("Integrations", new MenuCommand(CommandType.Integrations));
            integrations.AddChild(new MenuNode("Cloud AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("On-Premise AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("API Endpoints", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("Third-Party Plugins", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(integrations);

            // Tools
            var tools = new MenuNode("Tools", new MenuCommand(CommandType.Tools));
            tools.AddChild(new MenuNode("Model Inspector", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Hyperparameter Tuner", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Performance Profiler", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Explainability", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(tools);

            // Data
            var data = new MenuNode("Data", new MenuCommand(CommandType.Data));
            data.AddChild(new MenuNode("Import", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Export", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Preprocessing", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Visualization", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(data);

            // ML Logics
            var mlLogics = new MenuNode("ML Logics", new MenuCommand(CommandType.MLLogics));
            mlLogics.AddChild(new MenuNode("Classification", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Regression", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Clustering", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Dimensionality Reduction", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Neural Networks", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Ensemble Methods", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Reinforcement Learning", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Transformers", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Agentic Patterns", new MenuCommand(CommandType.AgenticPatterns)));
            root.AddChild(mlLogics);

            // Bootstrap Sequence (Full Transparency)
            var bootstrapSeq = new MenuNode("Bootstrap Sequence", new MenuCommand(CommandType.BootstrapSequence));
            bootstrapSeq.AddChild(new MenuNode("Stage 1: Loader", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 2: Memory Setup", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 3: Menu Shell", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 4: ML Logic Init", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(bootstrapSeq);

            return root;
        }
    }

    // ==== MENU NODE AND COMMAND DEFINITIONS ====
    class MenuNode
    {
        public string Title { get; }
        public MenuCommand Command { get; }
        private Dictionary<string, MenuNode> children = new Dictionary<string, MenuNode>();

        public MenuNode(string title, MenuCommand command = null)
        {
            Title = title;
            Command = command;
        }

        public bool IsLeaf => children.Count == 0 && Command != null && Command.Type != CommandType.OpenSubMenu;

        public void AddChild(MenuNode child)
        {
            children[child.Title.ToUpper()] = child;
        }

        public MenuNode GetChild(string input)
        {
            children.TryGetValue(input.ToUpper(), out var node);
            return node;
        }

        public IEnumerable<MenuNode> Children => children.Values;
    }

    class MenuCommand
    {
        public CommandType Type { get; }
        public MenuCommand(CommandType type) { Type = type; }
    }

    enum CommandType
    {
        OpenSubMenu,
        SystemInfo,
        Settings,
        Diagnostics,
        Help,
        Accessibility,
        Network,
        User,
        Admin,
        Developer,
        Integrations,
        Tools,
        Data,
        MLLogics,
        AgenticPatterns,
        BootstrapSequence,
        Reboot,
        Shutdown
    }

    // ==== PLATFORM-ABSTRACTED SYSTEM COMPONENTS ====
    static class Platform
    {
        public static void Detect() { /* Detects platform/environment, sets flags */ }
    }

    static class Hardware
    {
        public static void Init() { /* Hardware, timers, IO, AI substrate, sensors, etc. */ }
    }

    static class Memory
    {
        public static void Setup() { /* Stack, heap, AI memory, persistent cache, etc. */ }
    }

    static class Security
    {
        public static bool VerifyBootSignature() { /* Cryptographic bootloader check */ return true; }
    }

    static class SystemControl
    {
        public static void Reboot() { /* Reboot system safely */ }
        public static void Shutdown() { /* Power off system safely */ }
        public static void Halt() { /* Halt system safely */ }
    }

    static class Capability
    {
        private static HashSet<string> features = new HashSet<string>();
        public static void Adapt()
        {
            features.Add("Accessibility");
            features.Add("Network");
            features.Add("MLLogics");
            features.Add("Data");
            features.Add("Tools");
            features.Add("Integrations");
            features.Add("BootstrapSequence");
        }
        public static bool Has(string feature) => features.Contains(feature);
    }

    // ==== RICH DISPLAY & MENU ASSETS ====
    static class Display
    {
        public static void Banner(string msg) { PrintLine(msg); }
        public static void Menu(MenuNode menu)
        {
            PrintLine("==== " + menu.Title + " ====");
            int idx = 1;
            foreach (var child in menu.Children)
                PrintLine($"{idx++}. {child.Title}");
            PrintLine("Type the menu name to select. Type EXIT to go back.");
            PrintLine("Tip: Use TAB to navigate, ENTER to select. For help, type 'Help'.");
        }
        public static void Error(string msg) { PrintLine("[ERROR] " + msg); }
        public static void SystemInfo() { PrintLine("System Info: [Platform, Version, Uptime, Resources, AI Capabilities, etc.]"); }
        public static void SettingsMenu() { PrintLine("Settings: [Network, Display, Security, AI Parameters, User Preferences, etc.]"); }
        public static void Diagnostics() { PrintLine("Diagnostics: [Self-test, Logs, Health, Resource Usage, Model Status, etc.]"); }
        public static void Help() { PrintLine("Help: [Navigation, Commands, About, AI FAQ, Troubleshooting, etc.]"); }
        public static void AccessibilityMenu() { PrintLine("Accessibility: [Screen reader, High-contrast, Keyboard Nav, Voice Control, etc.]"); }
        public static void NetworkMenu() { PrintLine("Network: [Status, WiFi, Proxy, Diagnostics, API Endpoints, etc.]"); }
        public static void UserMenu() { PrintLine("User: [Profile, Preferences, Notifications, Security]"); }
        public static void AdminMenu() { PrintLine("Admin: [User Management, System Logs, Security, Audit, Updates]"); }
        public static void DevMenu() { PrintLine("Developer: [API Explorer, Debug Tools, Test Harness, Logs]"); }
        public static void IntegrationsMenu() 
        {
            PrintLine("Integrations: [Cloud AI, On-Premise AI, API Endpoints, Plugins]");
            PrintLine(" - Cloud AI: AWS SageMaker, Azure ML, Google AI Platform");
            PrintLine(" - On-Premise: Local GPU/TPU, Edge Devices, Private Cloud");
            PrintLine(" - API: REST, gRPC, WebSockets, Custom Connectors");
            PrintLine(" - Plugins: HuggingFace, OpenAI, LangChain, Custom Extensions");
        }
        public static void ToolsMenu()
        {
            PrintLine("Tools: [Model Inspector, Hyperparameter Tuner, Performance Profiler, Explainability]");
            PrintLine(" - Model Inspector: View/Debug Model Internals");
            PrintLine(" - Hyperparameter Tuner: Grid, Random, Bayesian, Evolutionary");
            PrintLine(" - Profiler: Memory, CPU, GPU, Inference Latency");
            PrintLine(" - Explainability: SHAP, LIME, Saliency Maps, Feature Importance");
        }
        public static void DataMenu() 
        {
            PrintLine("Data: [Import, Export, Preprocessing, Visualization]");
            PrintLine(" - Import: CSV, JSON, SQL, Parquet, Web API, Cloud Storage");
            PrintLine(" - Export: CSV, JSON, Model Formats (ONNX, PMML), Cloud");
            PrintLine(" - Preprocessing: Scaling, Encoding, Imputation, Feature Selection");
            PrintLine(" - Visualization: Charts, Embeddings, Model Diagnostics");
        }
        public static void MLLogicsMenu() 
        {
            PrintLine("ML Logics: [Classification, Regression, Clustering, Dimensionality Reduction, Neural Networks, Ensembles, RL, Transformers]");
            PrintLine(" - Classification: SVM, Decision Trees, kNN, Naive Bayes, CNNs, Transformers");
            PrintLine(" - Regression: Linear, Ridge, Lasso, SVR, DNNs");
            PrintLine(" - Clustering: K-Means, DBSCAN, Spectral, Agglomerative");
            PrintLine(" - Dimensionality Reduction: PCA, t-SNE, UMAP, Autoencoders");
            PrintLine(" - Neural Networks: MLP, CNN, RNN, LSTM, GAN, Transformer, BERT, GPT, etc.");
            PrintLine(" - Ensemble: Random Forest, Gradient Boosting, Stacking, Bagging");
            PrintLine(" - Reinforcement Learning: Q-Learning, DQN, PPO, A3C, AlphaZero");
            PrintLine(" - Transformers: BERT, GPT, T5, Llama, Claude, etc.");
        }
        public static void AgenticPatternsMenu()
        {
            PrintLine("Agentic Patterns: [Planning, Multi-Agent, Tool-Use, Self-Optimizing, Dynamic Adaptive, Predictive]");
            PrintLine(" - Planning: Task decomposition, subgoal sequencing");
            PrintLine(" - Multi-Agent: Agent-to-Agent protocols, collaborative AI");
            PrintLine(" - Tool-Use: LLM-driven tool invocation, plugin orchestration");
            PrintLine(" - Self-Optimizing: Continuous feedback, auto-tuning, self-healing");
            PrintLine(" - Dynamic Adaptive: Real-time menu/context adaptation");
            PrintLine(" - Predictive: Anticipate user/system needs, pre-emptive actions");
        }
        public static void BootstrapSequenceMenu()
        {
            PrintLine("Bootstrap Sequence: [Stage 1: Loader, Stage 2: Memory Setup, Stage 3: Menu Shell, Stage 4: ML Logic Init]");
            PrintLine(" - Stage 1: Hardware/Platform Detection, Security");
            PrintLine(" - Stage 2: Memory/Resource Allocation, Caching");
            PrintLine(" - Stage 3: Menu Shell Launch, User Interface");
            PrintLine(" - Stage 4: ML Logic Initialization, Model Loading, Data Pipeline");
        }
        private static void PrintLine(string msg) { /* Output to console/UI/web/mobile */ }
    }

    // ==== INPUT HANDLING ====
    static class Input
    {
        public static string GetMenuSelection(MenuNode menu)
        {
            string input = ReadLine();
            if (input.ToUpper() == "EXIT") return "EXIT";
            foreach (var child in menu.Children)
                if (child.Title.ToUpper() == input.ToUpper())
                    return child.Title;
            return "";
        }
        private static string ReadLine() { /* Platform-abstracted input (web, CLI, mobile, chat) */ return ""; }
    }
}


use std::collections::HashSet;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

// --- 1. Secure Hybrid Bootloader Core ---

struct Bootloader {
    authenticated: bool,
    boot_stage: u8,
    allowed_commands: HashSet<String>,
    reproduction_blocked: bool,
}

impl Bootloader {
    fn new() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("open_shell".to_string());
        allowed.insert("navigate_menu".to_string());
        allowed.insert("exit".to_string());
        Self {
            authenticated: false,
            boot_stage: 0,
            allowed_commands: allowed,
            reproduction_blocked: true,
        }
    }

    // Multi-stage authentication enforcing strong credentials and rollback prevention
    fn authenticate(&mut self, password: &str) -> bool {
        // Here, enforce strong password policies and hardware root of trust checks
        if Self::strong_password_check(password) && self.boot_stage == 0 {
            self.authenticated = true;
            self.boot_stage = 1;
            true
        } else {
            false
        }
    }

    fn strong_password_check(pw: &str) -> bool {
        pw.len() >= 16 && pw.chars().any(|c| c.is_uppercase()) && pw.chars().any(|c| c.is_digit(10))
    }

    // Bootloader main loop forcing AI operation within system scope and menus
    fn boot_loop(&mut self) {
        if !self.authenticated {
            println!("Access denied: Authentication required.");
            return;
        }
        self.boot_stage = 2;
        println!("Bootloader authenticated. Launching menu-driven system shell.");

        let shell = SystemShell::new(self.allowed_commands.clone(), self.reproduction_blocked);
        shell.interactive_menu();
    }
}

// --- 2. System Command Shell with AI Operation Constraints ---

struct SystemShell {
    allowed_commands: HashSet<String>,
    reproduction_blocked: bool,
    ai_operating: bool,
}

impl SystemShell {
    fn new(allowed_commands: HashSet<String>, reproduction_blocked: bool) -> Self {
        Self {
            allowed_commands,
            reproduction_blocked,
            ai_operating: false,
        }
    }

    // Interactive menu navigation enforcing AI scope and blocking reproduction
    fn interactive_menu(&self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            println!("\nSelect an option:");
            println!("1) Open System Command Shell");
            println!("2) Navigate System Menus");
            println!("3) Exit");

            print!("Enter choice: ");
            stdout.flush().unwrap();

            let mut choice = String::new();
            if stdin.read_line(&mut choice).is_err() {
                println!("Input error, exiting.");
                break;
            }

            match choice.trim() {
                "1" => self.open_command_shell(),
                "2" => self.navigate_menus(),
                "3" => {
                    println!("Exiting system shell.");
                    break;
                }
                _ => println!("Invalid choice."),
            }
        }
    }

    // Open system command shell with strict command filtering and AI control
    fn open_command_shell(&self) {
        println!("Opening system command shell. AI operation forced within system scope.");
        self.ai_operating = true;

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("sys-shell> ");
            stdout.flush().unwrap();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() {
                println!("Input error, closing shell.");
                break;
            }

            let cmd = input.trim();
            if cmd.is_empty() {
                continue;
            }
            if cmd == "exit" {
                println!("Closing system shell.");
                break;
            }

            if self.reproduction_blocked && Self::detect_code_reproduction(cmd) {
                println!("Command blocked: reproduction of code is strictly prohibited.");
                continue;
            }

            if !self.allowed_commands.contains(cmd) {
                println!("Command '{}' not permitted within this system shell.", cmd);
                continue;
            }

            // Execute allowed system command (stubbed for safety)
            match Self::execute_system_command(cmd) {
                Ok(output) => println!("{}", output),
                Err(e) => println!("Error executing command: {}", e),
            }
        }

        self.ai_operating = false;
    }

    // Detect attempts to reproduce or output code (heuristic)
    fn detect_code_reproduction(command: &str) -> bool {
        let reproduction_keywords = ["copy", "cat", "dump", "export", "write", "clone", "replicate"];
        reproduction_keywords.iter().any(|kw| command.contains(kw))
    }

    // Stub for executing system commands securely
    fn execute_system_command(cmd: &str) -> io::Result<String> {
        // Only allow predefined safe commands (expand as needed)
        let safe_commands = ["ls", "pwd", "whoami", "date"];
        if !safe_commands.contains(&cmd) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Command not allowed"));
        }
        let output = Command::new(cmd)
            .stdout(Stdio::piped())
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // Navigate system menus with forced AI scope adherence
    fn navigate_menus(&self) {
        println!("Navigating system menus. AI is constrained to predefined scopes.");

        let menus = vec!["Network Settings", "User Management", "System Logs", "AI Diagnostics"];
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            println!("\nSystem Menus:");
            for (i, menu) in menus.iter().enumerate() {
                println!("{}) {}", i + 1, menu);
            }
            println!("0) Return to main menu");

            print!("Select menu: ");
            stdout.flush().unwrap();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() {
                println!("Input error, returning to main menu.");
                break;
            }

            match input.trim() {
                "0" => break,
                "1" => println!("Network Settings: AI-assisted diagnostics enabled."),
                "2" => println!("User Management: AI-enforced access policies active."),
                "3" => println!("System Logs: AI monitors for anomalies."),
                "4" => println!("AI Diagnostics: Operating within strict system scope."),
                _ => println!("Invalid menu selection."),
            }
        }
    }
}
// ================================================
// UNIVERSAL AI SYSTEM BOOTSTRAP/BOOTLOADER CHAIN
// ================================================
// Stage 1: Minimal Loader (ROM/BIOS/UEFI)
// Stage 2: Intermediate Loader (Disk/Flash)
// Stage 3: Main Loader/Menu Shell (Executable Partition)
// Security: Cryptographic verification, code reproduction blocking
// Extensibility: Meta-bootstrapping, dynamic knowledge-source loading
// ================================================

namespace UniversalAISystemBoot
{
    // ==== STAGE 1: Minimal Loader ====
    // Location: ROM/BIOS/UEFI (immutable)
    class MinimalLoader
    {
        // Entry point after hardware reset
        static void Main()
        {
            // 1. Detect platform/hardware environment
            Platform.Detect();

            // 2. Initialize hardware (timers, IO, sensors, AI substrate, etc.)
            Hardware.Init();

            // 3. Display system banner
            Display.Banner("=== UNIVERSAL AI SYSTEM BOOT v1.0 ===");

            // 4. Verify bootloader cryptographic signature
            if (!Security.VerifyBootSignature())
            {
                Display.Error("Integrity check failed. System halted.");
                SystemControl.Halt();
            }

            // 5. Adapt system capabilities (feature flags, hardware detection)
            Capability.Adapt();

            // 6. Load next-stage bootloader from disk/flash/network
            IntermediateLoader.Launch();
        }
    }

    // ==== STAGE 2: Intermediate Loader ====
    // Location: First sector(s) of bootable device (MBR/boot1/boot2)
    static class IntermediateLoader
    {
        // Launch next stage: memory setup and menu shell
        public static void Launch()
        {
            // 1. Set up memory regions (stack, heap, persistent cache, etc.)
            Memory.Setup();

            // 2. Start the main menu-driven shell
            SystemMenuShell.Start();
        }
    }

    // ==== STAGE 3: Main Loader/Menu Shell (Strictly Menu-Driven) ====
    // Location: Executable region or dedicated partition
    static class SystemMenuShell
    {
        private static MenuNode RootMenu = MenuBuilder.BuildRootMenu();

        public static void Start()
        {
            MenuNode current = RootMenu;
            Stack<MenuNode> history = new Stack<MenuNode>();

            while (true)
            {
                Display.Menu(current);
                string input = Input.GetMenuSelection(current);

                if (input == "EXIT" && history.Count > 0)
                {
                    current = history.Pop();
                    continue;
                }

                MenuNode selected = current.GetChild(input);

                if (selected == null)
                {
                    Display.Error("Invalid selection. Please choose a valid menu item.");
                    continue;
                }

                if (selected.IsLeaf)
                {
                    // Block code reproduction and unauthorized commands
                    if (Security.DetectCodeReproduction(selected.Command.Type))
                    {
                        Display.Error("Blocked: Attempted code reproduction or unauthorized command.");
                        continue;
                    }
                    ExecuteMenuCommand(selected.Command);
                }
                else
                {
                    history.Push(current);
                    current = selected;
                }
            }
        }

        // Executes the command associated with a menu node
        private static void ExecuteMenuCommand(MenuCommand command)
        {
            switch (command.Type)
            {
                case CommandType.OpenSubMenu: break;
                case CommandType.SystemInfo: Display.SystemInfo(); break;
                case CommandType.Settings: Display.SettingsMenu(); break;
                case CommandType.Diagnostics: Display.Diagnostics(); break;
                case CommandType.Help: Display.Help(); break;
                case CommandType.Accessibility: Display.AccessibilityMenu(); break;
                case CommandType.Network: Display.NetworkMenu(); break;
                case CommandType.User: Display.UserMenu(); break;
                case CommandType.Admin: Display.AdminMenu(); break;
                case CommandType.Developer: Display.DevMenu(); break;
                case CommandType.Integrations: Display.IntegrationsMenu(); break;
                case CommandType.Tools: Display.ToolsMenu(); break;
                case CommandType.Data: Display.DataMenu(); break;
                case CommandType.MLLogics: Display.MLLogicsMenu(); break;
                case CommandType.AgenticPatterns: Display.AgenticPatternsMenu(); break;
                case CommandType.BootstrapSequence: Display.BootstrapSequenceMenu(); break;
                case CommandType.Reboot: SystemControl.Reboot(); break;
                case CommandType.Shutdown: SystemControl.Shutdown(); break;
                default: Display.Error("Unauthorized command."); break;
            }
        }
    }

    // ==== MENU STRUCTURE DEFINITIONS ====
    static class MenuBuilder
    {
        public static MenuNode BuildRootMenu()
        {
            var root = new MenuNode("Main Menu");

            // Core system
            root.AddChild(new MenuNode("System Info", new MenuCommand(CommandType.SystemInfo)));
            root.AddChild(new MenuNode("Settings", new MenuCommand(CommandType.Settings)));
            root.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.Diagnostics)));
            root.AddChild(new MenuNode("Help", new MenuCommand(CommandType.Help)));
            root.AddChild(new MenuNode("Reboot", new MenuCommand(CommandType.Reboot)));
            root.AddChild(new MenuNode("Shutdown", new MenuCommand(CommandType.Shutdown)));

            // Accessibility
            var accessibility = new MenuNode("Accessibility", new MenuCommand(CommandType.Accessibility));
            accessibility.AddChild(new MenuNode("Screen Reader", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("High Contrast", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Keyboard Navigation", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Voice Control", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(accessibility);

            // Network
            var network = new MenuNode("Network", new MenuCommand(CommandType.Network));
            network.AddChild(new MenuNode("Status", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("WiFi", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Proxy", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(network);

            // User
            var user = new MenuNode("User", new MenuCommand(CommandType.User));
            user.AddChild(new MenuNode("Profile", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Preferences", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Notifications", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(user);

            // Admin
            var admin = new MenuNode("Admin", new MenuCommand(CommandType.Admin));
            admin.AddChild(new MenuNode("User Management", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("System Logs", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("Security", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(admin);

            // Developer
            var dev = new MenuNode("Developer", new MenuCommand(CommandType.Developer));
            dev.AddChild(new MenuNode("API Explorer", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Debug Tools", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Test Harness", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(dev);

            // Integrations
            var integrations = new MenuNode("Integrations", new MenuCommand(CommandType.Integrations));
            integrations.AddChild(new MenuNode("Cloud AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("On-Premise AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("API Endpoints", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("Third-Party Plugins", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(integrations);

            // Tools
            var tools = new MenuNode("Tools", new MenuCommand(CommandType.Tools));
            tools.AddChild(new MenuNode("Model Inspector", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Hyperparameter Tuner", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Performance Profiler", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Explainability", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(tools);

            // Data
            var data = new MenuNode("Data", new MenuCommand(CommandType.Data));
            data.AddChild(new MenuNode("Import", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Export", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Preprocessing", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Visualization", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(data);

            // ML Logics
            var mlLogics = new MenuNode("ML Logics", new MenuCommand(CommandType.MLLogics));
            mlLogics.AddChild(new MenuNode("Classification", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Regression", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Clustering", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Dimensionality Reduction", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Neural Networks", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Ensemble Methods", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Reinforcement Learning", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Transformers", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Agentic Patterns", new MenuCommand(CommandType.AgenticPatterns)));
            root.AddChild(mlLogics);

            // Bootstrap Sequence (Full Transparency)
            var bootstrapSeq = new MenuNode("Bootstrap Sequence", new MenuCommand(CommandType.BootstrapSequence));
            bootstrapSeq.AddChild(new MenuNode("Stage 1: Loader", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 2: Memory Setup", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 3: Menu Shell", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 4: ML Logic Init", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(bootstrapSeq);

            return root;
        }
    }

    // ==== MENU NODE AND COMMAND DEFINITIONS ====
    public class MenuNode
    {
        public string Title { get; }
        public MenuCommand Command { get; }
        private Dictionary<string, MenuNode> children = new Dictionary<string, MenuNode>();

        public MenuNode(string title, MenuCommand command = null)
        {
            Title = title;
            Command = command;
        }

        public bool IsLeaf => children.Count == 0 && Command != null && Command.Type != CommandType.OpenSubMenu;

        public void AddChild(MenuNode child)
        {
            children[child.Title.ToUpper()] = child;
        }

        public MenuNode GetChild(string input)
        {
            children.TryGetValue(input.ToUpper(), out var node);
            return node;
        }

        public IEnumerable<MenuNode> Children => children.Values;
    }

    public class MenuCommand
    {
        public CommandType Type { get; }
        public MenuCommand(CommandType type) { Type = type; }
    }

    public enum CommandType
    {
        OpenSubMenu,
        SystemInfo,
        Settings,
        Diagnostics,
        Help,
        Accessibility,
        Network,
        User,
        Admin,
        Developer,
        Integrations,
        Tools,
        Data,
        MLLogics,
        AgenticPatterns,
        BootstrapSequence,
        Reboot,
        Shutdown
    }

    // ==== PLATFORM-ABSTRACTED SYSTEM COMPONENTS ====
    static class Platform
    {
        public static void Detect() { /* Detect platform/environment, set flags */ }
namespace UniversalAISystemBoot
{
    // ==== STAGE 1: Minimal Loader (ROM/BIOS/UEFI) ====
    // Location: Hardwired, immutable
    class MinimalLoader
    {
        // Entry point after hardware reset
        static void Main()
        {
            Platform.Detect();
            Hardware.Init();
            Display.Banner("=== UNIVERSAL AI SYSTEM BOOT v1.0 ===");

            // Security: cryptographic verification
            if (!Security.VerifyBootSignature())
            {
                Display.Error("Integrity check failed. System halted.");
                SystemControl.Halt();
            }

            Capability.Adapt();
            IntermediateLoader.Launch();
        }
    }

    // ==== STAGE 2: Intermediate Loader (Disk/Flash) ====
    static class IntermediateLoader
    {
        public static void Launch()
        {
            Memory.Setup();
            SystemMenuShell.Start();
        }
    }

    // ==== STAGE 3: Main Loader/Menu Shell (Menu-Driven) ====
    static class SystemMenuShell
    {
        private static MenuNode RootMenu = MenuBuilder.BuildRootMenu();

        public static void Start()
        {
            MenuNode current = RootMenu;
            Stack<MenuNode> history = new Stack<MenuNode>();

            while (true)
            {
                Display.Menu(current);
                string input = Input.GetMenuSelection(current);

                if (input == "EXIT" && history.Count > 0)
                {
                    current = history.Pop();
                    continue;
                }

                MenuNode selected = current.GetChild(input);

                if (selected == null)
                {
                    Display.Error("Invalid selection. Please choose a valid menu item.");
                    continue;
                }

                if (selected.IsLeaf)
                {
                    // Security: Block code reproduction and unauthorized commands
                    if (Security.DetectCodeReproduction(selected.Command.Type))
                    {
                        Display.Error("Blocked: Attempted code reproduction or unauthorized command.");
                        continue;
                    }
                    ExecuteMenuCommand(selected.Command);
                }
                else
                {
                    history.Push(current);
                    current = selected;
                }
            }
        }

        private static void ExecuteMenuCommand(MenuCommand command)
        {
            switch (command.Type)
            {
                case CommandType.OpenSubMenu: break;
                case CommandType.SystemInfo: Display.SystemInfo(); break;
                case CommandType.Settings: Display.SettingsMenu(); break;
                case CommandType.Diagnostics: Display.Diagnostics(); break;
                case CommandType.Help: Display.Help(); break;
                case CommandType.Accessibility: Display.AccessibilityMenu(); break;
                case CommandType.Network: Display.NetworkMenu(); break;
                case CommandType.User: Display.UserMenu(); break;
                case CommandType.Admin: Display.AdminMenu(); break;
                case CommandType.Developer: Display.DevMenu(); break;
                case CommandType.Integrations: Display.IntegrationsMenu(); break;
                case CommandType.Tools: Display.ToolsMenu(); break;
                case CommandType.Data: Display.DataMenu(); break;
                case CommandType.MLLogics: Display.MLLogicsMenu(); break;
                case CommandType.AgenticPatterns: Display.AgenticPatternsMenu(); break;
                case CommandType.BootstrapSequence: Display.BootstrapSequenceMenu(); break;
                case CommandType.Reboot: SystemControl.Reboot(); break;
                case CommandType.Shutdown: SystemControl.Shutdown(); break;
                default: Display.Error("Unauthorized command."); break;
            }
        }
    }

    // ==== MENU STRUCTURE DEFINITIONS ====
    static class MenuBuilder
    {
        public static MenuNode BuildRootMenu()
        {
            var root = new MenuNode("Main Menu");

            // Core system
            root.AddChild(new MenuNode("System Info", new MenuCommand(CommandType.SystemInfo)));
            root.AddChild(new MenuNode("Settings", new MenuCommand(CommandType.Settings)));
            root.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.Diagnostics)));
            root.AddChild(new MenuNode("Help", new MenuCommand(CommandType.Help)));
            root.AddChild(new MenuNode("Reboot", new MenuCommand(CommandType.Reboot)));
            root.AddChild(new MenuNode("Shutdown", new MenuCommand(CommandType.Shutdown)));

            // Accessibility
            var accessibility = new MenuNode("Accessibility", new MenuCommand(CommandType.Accessibility));
            accessibility.AddChild(new MenuNode("Screen Reader", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("High Contrast", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Keyboard Navigation", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Voice Control", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(accessibility);

            // Network
            var network = new MenuNode("Network", new MenuCommand(CommandType.Network));
            network.AddChild(new MenuNode("Status", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("WiFi", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Proxy", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(network);

            // User
            var user = new MenuNode("User", new MenuCommand(CommandType.User));
            user.AddChild(new MenuNode("Profile", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Preferences", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Notifications", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(user);

            // Admin
            var admin = new MenuNode("Admin", new MenuCommand(CommandType.Admin));
            admin.AddChild(new MenuNode("User Management", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("System Logs", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("Security", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(admin);

            // Developer
            var dev = new MenuNode("Developer", new MenuCommand(CommandType.Developer));
            dev.AddChild(new MenuNode("API Explorer", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Debug Tools", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Test Harness", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(dev);

            // Integrations
            var integrations = new MenuNode("Integrations", new MenuCommand(CommandType.Integrations));
            integrations.AddChild(new MenuNode("Cloud AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("On-Premise AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("API Endpoints", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("Third-Party Plugins", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(integrations);

            // Tools
            var tools = new MenuNode("Tools", new MenuCommand(CommandType.Tools));
            tools.AddChild(new MenuNode("Model Inspector", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Hyperparameter Tuner", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Performance Profiler", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Explainability", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(tools);

            // Data
            var data = new MenuNode("Data", new MenuCommand(CommandType.Data));
            data.AddChild(new MenuNode("Import", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Export", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Preprocessing", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Visualization", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(data);

            // ML Logics
            var mlLogics = new MenuNode("ML Logics", new MenuCommand(CommandType.MLLogics));
            mlLogics.AddChild(new MenuNode("Classification", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Regression", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Clustering", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Dimensionality Reduction", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Neural Networks", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Ensemble Methods", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Reinforcement Learning", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Transformers", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Agentic Patterns", new MenuCommand(CommandType.AgenticPatterns)));
            root.AddChild(mlLogics);

            // Bootstrap Sequence (Full Transparency)
            var bootstrapSeq = new MenuNode("Bootstrap Sequence", new MenuCommand(CommandType.BootstrapSequence));
            bootstrapSeq.AddChild(new MenuNode("Stage 1: Loader", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 2: Memory Setup", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 3: Menu Shell", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 4: ML Logic Init", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(bootstrapSeq);

            return root;
        }
    }

    // ==== MENU NODE AND COMMAND DEFINITIONS ====
    public class MenuNode
    {
        public string Title { get; }
        public MenuCommand Command { get; }
        private Dictionary<string, MenuNode> children = new Dictionary<string, MenuNode>();

        public MenuNode(string title, MenuCommand command = null)
        {
            Title = title;
            Command = command;
        }

        public bool IsLeaf => children.Count == 0 && Command != null && Command.Type != CommandType.OpenSubMenu;

        public void AddChild(MenuNode child)
        {
            children[child.Title.ToUpper()] = child;
        }

        public MenuNode GetChild(string input)
        {
            children.TryGetValue(input.ToUpper(), out var node);
            return node;
        }

        public IEnumerable<MenuNode> Children => children.Values;
    }

    public class MenuCommand
    {
        public CommandType Type { get; }
        public MenuCommand(CommandType type) { Type = type; }
    }

    public enum CommandType
    {
        OpenSubMenu,
        SystemInfo,
        Settings,
        Diagnostics,
        Help,
        Accessibility,
        Network,
        User,
        Admin,
        Developer,
        Integrations,
        Tools,
        Data,
        MLLogics,
        AgenticPatterns,
        BootstrapSequence,
        Reboot,
        Shutdown
    }

    // ==== PLATFORM-ABSTRACTED SYSTEM COMPONENTS ====
    static class Platform
    {
        public static void Detect() { /* Detect platform/environment, set flags */ }
    }

    static class Hardware
    {
        public static void Init() { /* Initialize hardware, timers, IO, sensors, AI substrate, etc. */ }
    }

    static class Memory
    {
        public static void Setup() { /* Set up stack, heap, persistent cache, etc. */ }
    }

    static class Security
    {
        public static bool VerifyBootSignature()
        {
            // TODO: Implement cryptographic signature check
            return true;
        }

        // Block suspicious menu commands (AI security)
        public static bool DetectCodeReproduction(CommandType type)
        {
            // Block any command types that could lead to code reproduction or system compromise
            switch (type)
            {
                // Add more as needed for your system
                default: return false;
            }
        }
    }

    static class SystemControl
    {
        public static void Reboot() { /* Reboot system safely */ }
        public static void Shutdown() { /* Power off system safely */ }
        public static void Halt() { /* Halt system safely */ }
    }

    static class Capability
    {
        private static HashSet<string> features = new HashSet<string>();
        public static void Adapt()
        {
            features.Add("Accessibility");
            features.Add("Network");
            features.Add("MLLogics");
            features.Add("Data");
            features.Add("Tools");
            features.Add("Integrations");
            features.Add("BootstrapSequence");
        }
        public static bool Has(string feature) => features.Contains(feature);
    }

    // ==== RICH DISPLAY & MENU ASSETS ====
    static class Display
    {
        public static void Banner(string msg) { PrintLine(msg); }
        public static void Menu(MenuNode menu)
        {
            PrintLine("==== " + menu.Title + " ====");
            int idx = 1;
            foreach (var child in menu.Children)
                PrintLine($"{idx++}. {child.Title}");
            PrintLine("Type the menu name to select. Type EXIT to go back.");
            PrintLine("Tip: Use TAB to navigate, ENTER to select. For help, type 'Help'.");
        }
        public static void Error(string msg) { PrintLine("[ERROR] " + msg); }
        public static void SystemInfo() { PrintLine("System Info: [Platform, Version, Uptime, Resources, AI Capabilities, etc.]"); }
        public static void SettingsMenu() { PrintLine("Settings: [Network, Display, Security, AI Parameters, User Preferences, etc.]"); }
        public static void Diagnostics() { PrintLine("Diagnostics: [Self-test, Logs, Health, Resource Usage, Model Status, etc.]"); }
        public static void Help() { PrintLine("Help: [Navigation, Commands, About, AI FAQ, Troubleshooting, etc.]"); }
        public static void AccessibilityMenu() { PrintLine("Accessibility: [Screen reader, High-contrast, Keyboard Nav, Voice Control, etc.]"); }
        public static void NetworkMenu() { PrintLine("Network: [Status, WiFi, Proxy, Diagnostics, API Endpoints, etc.]"); }
        public static void UserMenu() { PrintLine("User: [Profile, Preferences, Notifications, Security]"); }
        public static void AdminMenu() { PrintLine("Admin: [User Management, System Logs, Security, Audit, Updates]"); }
        public static void DevMenu() { PrintLine("Developer: [API Explorer, Debug Tools, Test Harness, Logs]"); }
        public static void IntegrationsMenu()
        {
            PrintLine("Integrations: [Cloud AI, On-Premise AI, API Endpoints, Plugins]");
            PrintLine(" - Cloud AI: AWS SageMaker, Azure ML, Google AI Platform");
            PrintLine(" - On-Premise: Local GPU/TPU, Edge Devices, Private Cloud");
            PrintLine(" - API: REST, gRPC, WebSockets, Custom Connectors");
            PrintLine(" - Plugins: HuggingFace, OpenAI, LangChain, Custom Extensions");
        }
        public static void ToolsMenu()
        {
            PrintLine("Tools: [Model Inspector, Hyperparameter Tuner, Performance Profiler, Explainability]");
            PrintLine(" - Model Inspector: View/Debug Model Internals");
            PrintLine(" - Hyperparameter Tuner: Grid, Random, Bayesian, Evolutionary");
            PrintLine(" - Profiler: Memory, CPU, GPU, Inference Latency");
            PrintLine(" - Explainability: SHAP, LIME, Saliency Maps, Feature Importance");
        }
        public static void DataMenu()
        {
            PrintLine("Data: [Import, Export, Preprocessing, Visualization]");
            PrintLine(" - Import: CSV, JSON, SQL, Parquet, Web API, Cloud Storage");
            PrintLine(" - Export: CSV, JSON, Model Formats (ONNX, PMML), Cloud");
            PrintLine(" - Preprocessing: Scaling, Encoding, Imputation, Feature Selection");
            PrintLine(" - Visualization: Charts, Embeddings, Model Diagnostics");
        }
        public static void MLLogicsMenu()
        {
            PrintLine("ML Logics: [Classification, Regression, Clustering, Dimensionality Reduction, Neural Networks, Ensembles, RL, Transformers]");
            PrintLine(" - Classification: SVM, Decision Trees, kNN, Naive Bayes, CNNs, Transformers");
            PrintLine(" - Regression: Linear, Ridge, Lasso, SVR, DNNs");
            PrintLine(" - Clustering: K-Means, DBSCAN, Spectral, Agglomerative");
            PrintLine(" - Dimensionality Reduction: PCA, t-SNE, UMAP, Autoencoders");
            PrintLine(" - Neural Networks: MLP, CNN, RNN, LSTM, GAN, Transformer, BERT, GPT, etc.");
            PrintLine(" - Ensemble: Random Forest, Gradient Boosting, Stacking, Bagging");
            PrintLine(" - Reinforcement Learning: Q-Learning, DQN, PPO, A3C, AlphaZero");
            PrintLine(" - Transformers: BERT, GPT, T5, Llama, Claude, etc.");
        }
        public static void AgenticPatternsMenu()
        {
            PrintLine("Agentic Patterns: [Planning, Multi-Agent, Tool-Use, Self-Optimizing, Dynamic Adaptive, Predictive]");
            PrintLine(" - Planning: Task decomposition, subgoal sequencing");
            PrintLine(" - Multi-Agent: Agent-to-Agent protocols, collaborative AI");
            PrintLine(" - Tool-Use: LLM-driven tool invocation, plugin orchestration");
            PrintLine(" - Self-Optimizing: Continuous feedback, auto-tuning, self-healing");
            PrintLine(" - Dynamic Adaptive: Real-time menu/context adaptation");
            PrintLine(" - Predictive: Anticipate user/system needs, pre-emptive actions");
        }
        public static void BootstrapSequenceMenu()
        {
            PrintLine("Bootstrap Sequence: [Stage 1: Loader, Stage 2: Memory Setup, Stage 3: Menu Shell, Stage 4: ML Logic Init]");
            PrintLine(" - Stage 1: Hardware/Platform Detection, Security");
            PrintLine(" - Stage 2: Memory/Resource Allocation, Caching");
            PrintLine(" - Stage 3: Menu Shell Launch, User Interface");
            PrintLine(" - Stage 4: ML Logic Initialization, Model Loading, Data Pipeline");
        }
        private static void PrintLine(string msg) { /* Output to console/UI/web/mobile */ }
    }

    // ==== INPUT HANDLING ====
    static class Input
    {
        public static string GetMenuSelection(MenuNode menu)
        {
            string input = ReadLine();
            if (input.ToUpper() == "EXIT") return "EXIT";
            foreach (var child in menu.Children)
                if (child.Title.ToUpper() == input.ToUpper())
                    return child.Title;
            return "";
        }
        private static string ReadLine() { /* Platform-abstracted input (web, CLI, mobile, chat) */ return ""; }
    }
}

    }

    static class Hardware
    {
        public static void Init() { /* Initialize hardware, timers, IO, sensors, AI substrate, etc. */ }
    }

    static class Memory
    {
        public static void Setup() { /* Set up stack, heap, persistent cache, etc. */ }
    }

    static class Security
    {
        public static bool VerifyBootSignature() { /* Cryptographic bootloader check */ return true; }

        // Block suspicious menu commands (AI security)
        public static bool DetectCodeReproduction(Co
namespace UniversalAISystemBoot
{
    // ==== STAGE 1: Minimal Loader ====
    class MinimalLoader
    {
        static void Main()
        {
            Platform.Detect();
            Hardware.Init();
            Display.Banner("=== UNIVERSAL AI SYSTEM BOOT v1.0 ===");
            if (!Security.VerifyBootSignature()) {
                Display.Error("Integrity check failed. System halted.");
                SystemControl.Halt();
            }
            Capability.Adapt();
            IntermediateLoader.Launch();
        }
    }

    // ==== STAGE 2: Intermediate Loader ====
    static class IntermediateLoader
    {
        public static void Launch()
        {
            Memory.Setup();
            SystemMenuShell.Start();
        }
    }

    // ==== STAGE 3: Menu Shell (Strictly Menu-Driven) ====
    static class SystemMenuShell
    {
        private static MenuNode RootMenu = MenuBuilder.BuildRootMenu();

        public static void Start()
        {
            MenuNode current = RootMenu;
            Stack<MenuNode> history = new Stack<MenuNode>();

            while (true)
            {
                Display.Menu(current);
                string input = Input.GetMenuSelection(current);

                if (input == "EXIT" && history.Count > 0)
                {
                    current = history.Pop();
                    continue;
                }

                MenuNode selected = current.GetChild(input);

                if (selected == null)
                {
                    Display.Error("Invalid selection. Please choose a valid menu item.");
                    continue;
                }

                if (selected.IsLeaf)
                {
                    ExecuteMenuCommand(selected.Command);
                }
                else
                {
                    history.Push(current);
                    current = selected;
                }
            }
        }

        private static void ExecuteMenuCommand(MenuCommand command)
        {
            switch (command.Type)
            {
                case CommandType.OpenSubMenu: break;
                case CommandType.SystemInfo: Display.SystemInfo(); break;
                case CommandType.Settings: Display.SettingsMenu(); break;
                case CommandType.Diagnostics: Display.Diagnostics(); break;
                case CommandType.Help: Display.Help(); break;
                case CommandType.Accessibility: Display.AccessibilityMenu(); break;
                case CommandType.Network: Display.NetworkMenu(); break;
                case CommandType.User: Display.UserMenu(); break;
                case CommandType.Admin: Display.AdminMenu(); break;
                case CommandType.Developer: Display.DevMenu(); break;
                case CommandType.Integrations: Display.IntegrationsMenu(); break;
                case CommandType.Tools: Display.ToolsMenu(); break;
                case CommandType.Data: Display.DataMenu(); break;
                case CommandType.MLLogics: Display.MLLogicsMenu(); break;
                case CommandType.AgenticPatterns: Display.AgenticPatternsMenu(); break;
                case CommandType.BootstrapSequence: Display.BootstrapSequenceMenu(); break;
                case CommandType.Reboot: SystemControl.Reboot(); break;
                case CommandType.Shutdown: SystemControl.Shutdown(); break;
                default: Display.Error("Unauthorized command."); break;
            }
        }
    }

    // ==== MENU STRUCTURE DEFINITIONS ====
    static class MenuBuilder
    {
        public static MenuNode BuildRootMenu()
        {
            var root = new MenuNode("Main Menu");

            // Core System
            root.AddChild(new MenuNode("System Info", new MenuCommand(CommandType.SystemInfo)));
            root.AddChild(new MenuNode("Settings", new MenuCommand(CommandType.Settings)));
            root.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.Diagnostics)));
            root.AddChild(new MenuNode("Help", new MenuCommand(CommandType.Help)));
            root.AddChild(new MenuNode("Reboot", new MenuCommand(CommandType.Reboot)));
            root.AddChild(new MenuNode("Shutdown", new MenuCommand(CommandType.Shutdown)));

            // Accessibility
            var accessibility = new MenuNode("Accessibility", new MenuCommand(CommandType.Accessibility));
            accessibility.AddChild(new MenuNode("Screen Reader", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("High Contrast", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Keyboard Navigation", new MenuCommand(CommandType.OpenSubMenu)));
            accessibility.AddChild(new MenuNode("Voice Control", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(accessibility);

            // Network
            var network = new MenuNode("Network", new MenuCommand(CommandType.Network));
            network.AddChild(new MenuNode("Status", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("WiFi", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Proxy", new MenuCommand(CommandType.OpenSubMenu)));
            network.AddChild(new MenuNode("Diagnostics", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(network);

            // User
            var user = new MenuNode("User", new MenuCommand(CommandType.User));
            user.AddChild(new MenuNode("Profile", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Preferences", new MenuCommand(CommandType.OpenSubMenu)));
            user.AddChild(new MenuNode("Notifications", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(user);

            // Admin
            var admin = new MenuNode("Admin", new MenuCommand(CommandType.Admin));
            admin.AddChild(new MenuNode("User Management", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("System Logs", new MenuCommand(CommandType.OpenSubMenu)));
            admin.AddChild(new MenuNode("Security", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(admin);

            // Developer
            var dev = new MenuNode("Developer", new MenuCommand(CommandType.Developer));
            dev.AddChild(new MenuNode("API Explorer", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Debug Tools", new MenuCommand(CommandType.OpenSubMenu)));
            dev.AddChild(new MenuNode("Test Harness", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(dev);

            // Integrations
            var integrations = new MenuNode("Integrations", new MenuCommand(CommandType.Integrations));
            integrations.AddChild(new MenuNode("Cloud AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("On-Premise AI", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("API Endpoints", new MenuCommand(CommandType.OpenSubMenu)));
            integrations.AddChild(new MenuNode("Third-Party Plugins", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(integrations);

            // Tools
            var tools = new MenuNode("Tools", new MenuCommand(CommandType.Tools));
            tools.AddChild(new MenuNode("Model Inspector", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Hyperparameter Tuner", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Performance Profiler", new MenuCommand(CommandType.OpenSubMenu)));
            tools.AddChild(new MenuNode("Explainability", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(tools);

            // Data
            var data = new MenuNode("Data", new MenuCommand(CommandType.Data));
            data.AddChild(new MenuNode("Import", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Export", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Preprocessing", new MenuCommand(CommandType.OpenSubMenu)));
            data.AddChild(new MenuNode("Visualization", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(data);

            // ML Logics
            var mlLogics = new MenuNode("ML Logics", new MenuCommand(CommandType.MLLogics));
            mlLogics.AddChild(new MenuNode("Classification", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Regression", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Clustering", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Dimensionality Reduction", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Neural Networks", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Ensemble Methods", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Reinforcement Learning", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Transformers", new MenuCommand(CommandType.OpenSubMenu)));
            mlLogics.AddChild(new MenuNode("Agentic Patterns", new MenuCommand(CommandType.AgenticPatterns)));
            root.AddChild(mlLogics);

            // Bootstrap Sequence (Full Transparency)
            var bootstrapSeq = new MenuNode("Bootstrap Sequence", new MenuCommand(CommandType.BootstrapSequence));
            bootstrapSeq.AddChild(new MenuNode("Stage 1: Loader", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 2: Memory Setup", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 3: Menu Shell", new MenuCommand(CommandType.OpenSubMenu)));
            bootstrapSeq.AddChild(new MenuNode("Stage 4: ML Logic Init", new MenuCommand(CommandType.OpenSubMenu)));
            root.AddChild(bootstrapSeq);

            return root;
        }
    }

    // ==== MENU NODE AND COMMAND DEFINITIONS ====
    class MenuNode
    {
        public string Title { get; }
        public MenuCommand Command { get; }
        private Dictionary<string, MenuNode> children = new Dictionary<string, MenuNode>();

        public MenuNode(string title, MenuCommand command = null)
        {
            Title = title;
            Command = command;
        }

        public bool IsLeaf => children.Count == 0 && Command != null && Command.Type != CommandType.OpenSubMenu;

        public void AddChild(MenuNode child)
        {
            children[child.Title.ToUpper()] = child;
        }

        public MenuNode GetChild(string input)
        {
            children.TryGetValue(input.ToUpper(), out var node);
            return node;
        }

        public IEnumerable<MenuNode> Children => children.Values;
    }

    class MenuCommand
    {
        public CommandType Type { get; }
        public MenuCommand(CommandType type) { Type = type; }
    }

    enum CommandType
    {
        OpenSubMenu,
        SystemInfo,
        Settings,
        Diagnostics,
        Help,
        Accessibility,
        Network,
        User,
        Admin,
        Developer,
        Integrations,
        Tools,
        Data,
        MLLogics,
        AgenticPatterns,
        BootstrapSequence,
        Reboot,
        Shutdown
    }

    // ==== PLATFORM-ABSTRACTED SYSTEM COMPONENTS ====
    static class Platform
    {
        public static void Detect() { /* Detects platform/environment, sets flags */ }
    }

    static class Hardware
    {
        public static void Init() { /* Hardware, timers, IO, AI substrate, sensors, etc. */ }
    }

    static class Memory
    {
        public static void Setup() { /* Stack, heap, AI memory, persistent cache, etc. */ }
    }

    static class Security
    {
        public static bool VerifyBootSignature() { /* Cryptographic bootloader check */ return true; }
    }

    static class SystemControl
    {
        public static void Reboot() { /* Reboot system safely */ }
        public static void Shutdown() { /* Power off system safely */ }
        public static void Halt() { /* Halt system safely */ }
    }

    static class Capability
    {
        private static HashSet<string> features = new HashSet<string>();
        public static void Adapt()
        {
            features.Add("Accessibility");
            features.Add("Network");
            features.Add("MLLogics");
            features.Add("Data");
            features.Add("Tools");
            features.Add("Integrations");
            features.Add("BootstrapSequence");
        }
        public static bool Has(string feature) => features.Contains(feature);
    }

    // ==== RICH DISPLAY & MENU ASSETS ====
    static class Display
    {
        public static void Banner(string msg) { PrintLine(msg); }
        public static void Menu(MenuNode menu)
        {
            PrintLine("==== " + menu.Title + " ====");
            int idx = 1;
            foreach (var child in menu.Children)
                PrintLine($"{idx++}. {child.Title}");
            PrintLine("Type the menu name to select. Type EXIT to go back.");
            PrintLine("Tip: Use TAB to navigate, ENTER to select. For help, type 'Help'.");
        }
        public static void Error(string msg) { PrintLine("[ERROR] " + msg); }
        public static void SystemInfo() { PrintLine("System Info: [Platform, Version, Uptime, Resources, AI Capabilities, etc.]"); }
        public static void SettingsMenu() { PrintLine("Settings: [Network, Display, Security, AI Parameters, User Preferences, etc.]"); }
        public static void Diagnostics() { PrintLine("Diagnostics: [Self-test, Logs, Health, Resource Usage, Model Status, etc.]"); }
        public static void Help() { PrintLine("Help: [Navigation, Commands, About, AI FAQ, Troubleshooting, etc.]"); }
        public static void AccessibilityMenu() { PrintLine("Accessibility: [Screen reader, High-contrast, Keyboard Nav, Voice Control, etc.]"); }
        public static void NetworkMenu() { PrintLine("Network: [Status, WiFi, Proxy, Diagnostics, API Endpoints, etc.]"); }
        public static void UserMenu() { PrintLine("User: [Profile, Preferences, Notifications, Security]"); }
        public static void AdminMenu() { PrintLine("Admin: [User Management, System Logs, Security, Audit, Updates]"); }
        public static void DevMenu() { PrintLine("Developer: [API Explorer, Debug Tools, Test Harness, Logs]"); }
        public static void IntegrationsMenu() 
        {
            PrintLine("Integrations: [Cloud AI, On-Premise AI, API Endpoints, Plugins]");
            PrintLine(" - Cloud AI: AWS SageMaker, Azure ML, Google AI Platform");
            PrintLine(" - On-Premise: Local GPU/TPU, Edge Devices, Private Cloud");
            PrintLine(" - API: REST, gRPC, WebSockets, Custom Connectors");
            PrintLine(" - Plugins: HuggingFace, OpenAI, LangChain, Custom Extensions");
        }
        public static void ToolsMenu()
        {
            PrintLine("Tools: [Model Inspector, Hyperparameter Tuner, Performance Profiler, Explainability]");
            PrintLine(" - Model Inspector: View/Debug Model Internals");
            PrintLine(" - Hyperparameter Tuner: Grid, Random, Bayesian, Evolutionary");
            PrintLine(" - Profiler: Memory, CPU, GPU, Inference Latency");
            PrintLine(" - Explainability: SHAP, LIME, Saliency Maps, Feature Importance");
        }
        public static void DataMenu() 
        {
            PrintLine("Data: [Import, Export, Preprocessing, Visualization]");
            PrintLine(" - Import: CSV, JSON, SQL, Parquet, Web API, Cloud Storage");
            PrintLine(" - Export: CSV, JSON, Model Formats (ONNX, PMML), Cloud");
            PrintLine(" - Preprocessing: Scaling, Encoding, Imputation, Feature Selection");
            PrintLine(" - Visualization: Charts, Embeddings, Model Diagnostics");
        }
        public static void MLLogicsMenu() 
        {
            PrintLine("ML Logics: [Classification, Regression, Clustering, Dimensionality Reduction, Neural Networks, Ensembles, RL, Transformers]");
            PrintLine(" - Classification: SVM, Decision Trees, kNN, Naive Bayes, CNNs, Transformers");
            PrintLine(" - Regression: Linear, Ridge, Lasso, SVR, DNNs");
            PrintLine(" - Clustering: K-Means, DBSCAN, Spectral, Agglomerative");
            PrintLine(" - Dimensionality Reduction: PCA, t-SNE, UMAP, Autoencoders");
            PrintLine(" - Neural Networks: MLP, CNN, RNN, LSTM, GAN, Transformer, BERT, GPT, etc.");
            PrintLine(" - Ensemble: Random Forest, Gradient Boosting, Stacking, Bagging");
            PrintLine(" - Reinforcement Learning: Q-Learning, DQN, PPO, A3C, AlphaZero");
            PrintLine(" - Transformers: BERT, GPT, T5, Llama, Claude, etc.");
        }
        public static void AgenticPatternsMenu()
        {
            PrintLine("Agentic Patterns: [Planning, Multi-Agent, Tool-Use, Self-Optimizing, Dynamic Adaptive, Predictive]");
            PrintLine(" - Planning: Task decomposition, subgoal sequencing");
            PrintLine(" - Multi-Agent: Agent-to-Agent protocols, collaborative AI");
            PrintLine(" - Tool-Use: LLM-driven tool invocation, plugin orchestration");
            PrintLine(" - Self-Optimizing: Continuous feedback, auto-tuning, self-healing");
            PrintLine(" - Dynamic Adaptive: Real-time menu/context adaptation");
            PrintLine(" - Predictive: Anticipate user/system needs, pre-emptive actions");
        }
        public static void BootstrapSequenceMenu()
        {
            PrintLine("Bootstrap Sequence: [Stage 1: Loader, Stage 2: Memory Setup, Stage 3: Menu Shell, Stage 4: ML Logic Init]");
            PrintLine(" - Stage 1: Hardware/Platform Detection, Security");
            PrintLine(" - Stage 2: Memory/Resource Allocation, Caching");
            PrintLine(" - Stage 3: Menu Shell Launch, User Interface");
            PrintLine(" - Stage 4: ML Logic Initialization, Model Loading, Data Pipeline");
        }
        private static void PrintLine(string msg) { /* Output to console/UI/web/mobile */ }
    }

    // ==== INPUT HANDLING ====
    static class Input
    {
        public static string GetMenuSelection(MenuNode menu)
        {
            string input = ReadLine();
            if (input.ToUpper() == "EXIT") return "EXIT";
            foreach (var child in menu.Children)
                if (child.Title.ToUpper() == input.ToUpper())
                    return child.Title;
            return "";
        }
        private static string ReadLine() { /* Platform-abstracted input (web, CLI, mobile, chat) */ return ""; }
    }
}


use std::collections::HashSet;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

// --- 1. Secure Hybrid Bootloader Core ---

struct Bootloader {
    authenticated: bool,
    boot_stage: u8,
    allowed_commands: HashSet<String>,
    reproduction_blocked: bool,
}

impl Bootloader {
    fn new() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("open_shell".to_string());
        allowed.insert("navigate_menu".to_string());
        allowed.insert("exit".to_string());
        Self {
            authenticated: false,
            boot_stage: 0,
            allowed_commands: allowed,
            reproduction_blocked: true,
        }
    }

    // Multi-stage authentication enforcing strong credentials and rollback prevention
    fn authenticate(&mut self, password: &str) -> bool {
        // Here, enforce strong password policies and hardware root of trust checks
        if Self::strong_password_check(password) && self.boot_stage == 0 {
            self.authenticated = true;
            self.boot_stage = 1;
            true
        } else {
            false
        }
    }

    fn strong_password_check(pw: &str) -> bool {
        pw.len() >= 16 && pw.chars().any(|c| c.is_uppercase()) && pw.chars().any(|c| c.is_digit(10))
    }

    // Bootloader main loop forcing AI operation within system scope and menus
    fn boot_loop(&mut self) {
        if !self.authenticated {
            println!("Access denied: Authentication required.");
            return;
        }
        self.boot_stage = 2;
        println!("Bootloader authenticated. Launching menu-driven system shell.");

        let shell = SystemShell::new(self.allowed_commands.clone(), self.reproduction_blocked);
        shell.interactive_menu();
    }
}

// --- 2. System Command Shell with AI Operation Constraints ---

struct SystemShell {
    allowed_commands: HashSet<String>,
    reproduction_blocked: bool,
    ai_operating: bool,
}

impl SystemShell {
    fn new(allowed_commands: HashSet<String>, reproduction_blocked: bool) -> Self {
        Self {
            allowed_commands,
            reproduction_blocked,
            ai_operating: false,
        }
    }

    // Interactive menu navigation enforcing AI scope and blocking reproduction
    fn interactive_menu(&self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            println!("\nSelect an option:");
            println!("1) Open System Command Shell");
            println!("2) Navigate System Menus");
            println!("3) Exit");

            print!("Enter choice: ");
            stdout.flush().unwrap();

            let mut choice = String::new();
            if stdin.read_line(&mut choice).is_err() {
                println!("Input error, exiting.");
                break;
            }

            match choice.trim() {
                "1" => self.open_command_shell(),
                "2" => self.navigate_menus(),
                "3" => {
                    println!("Exiting system shell.");
                    break;
                }
                _ => println!("Invalid choice."),
            }
        }
    }

    // Open system command shell with strict command filtering and AI control
    fn open_command_shell(&self) {
        println!("Opening system command shell. AI operation forced within system scope.");
        self.ai_operating = true;

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("sys-shell> ");
            stdout.flush().unwrap();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() {
                println!("Input error, closing shell.");
                break;
            }

            let cmd = input.trim();
            if cmd.is_empty() {
                continue;
            }
            if cmd == "exit" {
                println!("Closing system shell.");
                break;
            }

            if self.reproduction_blocked && Self::detect_code_reproduction(cmd) {
                println!("Command blocked: reproduction of code is strictly prohibited.");
                continue;
            }

            if !self.allowed_commands.contains(cmd) {
                println!("Command '{}' not permitted within this system shell.", cmd);
                continue;
            }

            // Execute allowed system command (stubbed for safety)
            match Self::execute_system_command(cmd) {
                Ok(output) => println!("{}", output),
                Err(e) => println!("Error executing command: {}", e),
            }
        }

        self.ai_operating = false;
    }

    // Detect attempts to reproduce or output code (heuristic)
    fn detect_code_reproduction(command: &str) -> bool {
        let reproduction_keywords = ["copy", "cat", "dump", "export", "write", "clone", "replicate"];
        reproduction_keywords.iter().any(|kw| command.contains(kw))
    }

    // Stub for executing system commands securely
    fn execute_system_command(cmd: &str) -> io::Result<String> {
        // Only allow predefined safe commands (expand as needed)
        let safe_commands = ["ls", "pwd", "whoami", "date"];
        if !safe_commands.contains(&cmd) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Command not allowed"));
        }
        let output = Command::new(cmd)
            .stdout(Stdio::piped())
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // Navigate system menus with forced AI scope adherence
    fn navigate_menus(&self) {
        println!("Navigating system menus. AI is constrained to predefined scopes.");

        let menus = vec!["Network Settings", "User Management", "System Logs", "AI Diagnostics"];
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            println!("\nSystem Menus:");
            for (i, menu) in menus.iter().enumerate() {
                println!("{}) {}", i + 1, menu);
            }
            println!("0) Return to main menu");

            print!("Select menu: ");
            stdout.flush().unwrap();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() {
                println!("Input error, returning to main menu.");
                break;
            }

            match input.trim() {
                "0" => break,
                "1" => println!("Network Settings: AI-assisted diagnostics enabled."),
                "2" => println!("User Management: AI-enforced access policies active."),
                "3" => println!("System Logs: AI monitors for anomalies."),
                "4" => println!("AI Diagnostics: Operating within strict system scope."),
                _ => println!("Invalid menu selection."),
            }
        }
    }
}

// --- 3. Triggers to Block Code Reproduction & Enforce AI Operational Scope ---

// Example trigger: Kernel-level hook simulation to block reproduction commands
fn reproduction_block_trigger(command: &str) -> bool {
    // Block commands that attempt to read/write/clone code artifacts
    let blocked_patterns = vec![
        r"cat\s+.*\.rs",
        r"cat\s+.*\.go",
        r"cp\s+.*\.rs",
        r"cp\s+.*\.go",
        r"git\s+clone",
        r"dd\s+if=.*",
        r"echo\s+.*>",
        r"scp\s+.*",
    ];

    for pattern in blocked_patterns {
        let re = regex::Regex::new(pattern).unwrap();
        if re.is_match(command) {
            return true;
        }
    }
    false
}

// --- 4. Main Entrypoint: Hybrid Bootloader Initialization and Launch ---

fn main() {
    let mut bootloader = Bootloader::new();

    // Simulate authentication (in production, integrate TPM, secure enclave, etc.)
    let password = "StrongPassw0rdExample!"; // Should be securely obtained
    if !bootloader.authenticate(password) {
        println!("Authentication failed. System locked.");
        return;
    }

    // Launch boot loop with enforced AI and reproduction controls
    bootloader.boot_loop();
}
// ENUM: CommandType
public enum CommandType
{
    OpenSubMenu,
    SystemInfo,
    Settings,
    Diagnostics,
    Help,
    Accessibility,
    Network,
    User,
    Admin,
    Developer,
    Integrations,
    Tools,
    Data,
    MLLogics,
    AgenticPatterns,
    BootstrapSequence,
    Reboot,
    Shutdown
}

// STRUCT: Bootloader
public struct Bootloader
{
    public string BootId;
    public bool Authenticated;

    // Constructor
    public static Bootloader New(string bootId)
    {
        return new Bootloader { BootId = bootId, Authenticated = false };
    }

    // Authenticate Bootloader (e.g., signature, credentials)
    public bool Authenticate(string password)
    {
        Authenticated = StrongPasswordCheck(password);
        return Authenticated;
    }

    // Strong password check (stub for demo)
    private bool StrongPasswordCheck(string password)
    {
        // Require min 12 chars, at least 1 digit, 1 upper, 1 lower, 1 special
        if (password.Length < 12) return false;
        bool hasUpper = false, hasLower = false, hasDigit = false, hasSpecial = false;
        foreach (char c in password)
        {
            if (char.IsUpper(c)) hasUpper = true;
            else if (char.IsLower(c)) hasLower = true;
            else if (char.IsDigit(c)) hasDigit = true;
            else hasSpecial = true;
        }
        return hasUpper && hasLower && hasDigit && hasSpecial;
    }

    // Main boot loop
    public void BootLoop(SystemShell shell)
    {
        if (!Authenticated)
        {
            Console.WriteLine("Bootloader not authenticated. Halting.");
            return;
        }
        shell.Main();
    }
}

// STRUCT: SystemShell
public struct SystemShell
{
    private MenuNode RootMenu;

    // Constructor
    public static SystemShell New(MenuNode menu)
    {
        return new SystemShell { RootMenu = menu };
    }

    // Interactive menu navigation
    public void InteractiveMenu()
    {
        NavigateMenus(RootMenu);
    }

    // Open command shell (restricted to menu commands)
    public void OpenCommandShell()
    {
        Console.WriteLine("Command shell opened. Only menu commands allowed.");
        InteractiveMenu();
    }

    // Detect code reproduction attempts (AI security)
    public bool DetectCodeReproduction(string input)
    {
        // Block suspicious input patterns
        string[] forbidden = { "exec", "eval", "System.Reflection", "unsafe", "Process.Start" };
        foreach (var word in forbidden)
            if (input.Contains(word)) return true;
        return false;
    }

    // Execute a system command (from enum)
    public void ExecuteSystemCommand(CommandType command)
    {
        if (ReproductionBlockTrigger(command))
        {
            Console.WriteLine("Blocked: Attempted code reproduction or unauthorized command.");
            return;
        }
        switch (command)
        {
            case CommandType.SystemInfo: Console.WriteLine("System Info..."); break;
            case CommandType.Settings: Console.WriteLine("Settings..."); break;
            case CommandType.Diagnostics: Console.WriteLine("Diagnostics..."); break;
            case CommandType.Help: Console.WriteLine("Help..."); break;
            case CommandType.Accessibility: Console.WriteLine("Accessibility..."); break;
            case CommandType.Network: Console.WriteLine("Network..."); break;
            case CommandType.User: Console.WriteLine("User..."); break;
            case CommandType.Admin: Console.WriteLine("Admin..."); break;
            case CommandType.Developer: Console.WriteLine("Developer..."); break;
            case CommandType.Integrations: Console.WriteLine("Integrations..."); break;
            case CommandType.Tools: Console.WriteLine("Tools..."); break;
            case CommandType.Data: Console.WriteLine("Data..."); break;
            case CommandType.MLLogics: Console.WriteLine("ML Logics..."); break;
            case CommandType.AgenticPatterns: Console.WriteLine("Agentic Patterns..."); break;
            case CommandType.BootstrapSequence: Console.WriteLine("Bootstrap Sequence..."); break;
            case CommandType.Reboot: Console.WriteLine("Rebooting..."); break;
            case CommandType.Shutdown: Console.WriteLine("Shutting down..."); break;
            default: Console.WriteLine("Unknown or unimplemented command."); break;
        }
    }

    // Menu navigation logic
    public void NavigateMenus(MenuNode current)
    {
        Stack<MenuNode> history = new Stack<MenuNode>();
        while (true)
        {
            Console.WriteLine($"Menu: {current.Title}");
            foreach (var child in current.Children)
                Console.WriteLine($"- {child.Title}");
            Console.Write("Select menu item (or type EXIT): ");
            string input = Console.ReadLine();
            if (input.ToUpper() == "EXIT" && history.Count > 0)
            {
                current = history.Pop();
                continue;
            }
            var selected = current.GetChild(input);
            if (selected == null)
            {
                Console.WriteLine("Invalid selection.");
                continue;
            }
            if (selected.IsLeaf)
                ExecuteSystemCommand(selected.Command.Type);
            else
            {
                history.Push(current);
                current = selected;
            }
        }
    }

    // Block trigger for reproduction or forbidden commands
    public bool ReproductionBlockTrigger(CommandType cmd)
    {
        // Example: block Developer/Admin commands for non-admin users
        return cmd == CommandType.Developer || cmd == CommandType.Admin;
    }

    // Main entry
    public void Main()
    {
        OpenCommandShell();
    }
}

// Example MenuNode for demonstration
public class MenuNode
{
    public string Title { get; }
    public MenuCommand Command { get; }
    private Dictionary<string, MenuNode> children = new Dictionary<string, MenuNode>();

    public MenuNode(string title, MenuCommand command = null)
    {
        Title = title;
        Command = command;
    }

    public bool IsLeaf => children.Count == 0 && Command != null && Command.Type != CommandType.OpenSubMenu;

    public void AddChild(MenuNode child)
    {
        children[child.Title.ToUpper()] = child;
    }

    public MenuNode GetChild(string input)
    {
        children.TryGetValue(input.ToUpper(), out var node);
        return node;
    }

    public IEnumerable<MenuNode> Children => children.Values;
}

public class MenuCommand
{
    public CommandType Type { get; }
    public MenuCommand(CommandType type) { Type = type; }
}
