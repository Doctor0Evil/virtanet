{
  "ai_parameters": {
    "triggers": ["/play_chat_wars", "/game:start"],
    "allowed_environments": ["grok.com", "x.com"],
    "max_users_per_session": 8,
    "response_format": "json_object",
    "language_detection": true,
    "persist_session": true,
    "persistence_endpoint": "https://jacobdev.com/ai/burn_session",
    "encryption": {
      "type": "SHA-256",
      "replication_nodes": 47
    }
  }
}
{
  "purpose": "RTS Game Launch in Chat",
  "instructions": {
    "init": "Load lobby UI and present available modes.",
    "validate_players": "Ensure player count >= 1 and <= 8.",
    "start_game": "When minimum players ready, instantiate game session and broadcast to all participants.",
    "stats_update": "Continuously update and persist stats."
  },
  "modification_menu": {
    "modes": ["Single-Player", "Multiplayer", "Human vs AI"],
    "epoch_advancements": true,
    "leaderboard_enabled": true,
    "max_players": 8,
    "dynamic_commands": ["/game:pause", "/game:resume", "/game:end"]
  },
  "persistence": {
    "use_storage": true,
    "auto_sync": true
  },
  "safety_controls": {
    "max_runtime_minutes": 180,
    "require_confirmation_for_end": true
  }
}
{
  "prompt": "The game lobby is now open. Players joined: {playerCount}/{maxPlayers}.",
  "quick_replies": [
    {"title": "Join Lobby", "payload": "Join Lobby"},
    {"title": "Leave Lobby", "payload": "Leave Lobby"},
    {"title": "See Stats", "payload": "See Stats"},
    {"title": "View Leaderboard", "payload": "View Leaderboard"},
    {"title": "Ready", "payload": "Ready"}
  ]
}
{
  "prompt": "Your current stats: Games Played: {gamesPlayed}, Players Eliminated: {playersEliminated}, Systems Created: {systemsCreated}, GDB: {gdbAccumulated}. What would you like to do next?",
  "quick_replies": [
    {"title": "Start New Game", "payload": "Start New Game"},
    {"title": "View Leaderboard", "payload": "View Leaderboard"},
    {"title": "Exit", "payload": "Exit"}
  ]
}
{
  "activation_rules": {
    "command_triggers": ["/play_chat_wars"],
    "minimum_players_required": 2,
    "maximum_players_allowed": 8,
    "session_scope": "per conversation",
    "owner_permissions": ["start", "end", "pause", "kick user"],
    "team_modes_enabled": true,
    "ai_vs_human_allowed": true,
    "auto_cleanup_on_inactive": 10   // Minutes before lobby auto-closes if inactive
  }
}
{
  "error_handling": {
    "on_invalid_command": {
      "message": "Sorry, that command isn't recognized. Use /play_chat_wars to start.",
      "action": "display_help"
    },
    "on_network_failure": {
      "message": "Network error. Retrying...",
      "retry_limit": 3
    },
    "on_session_expired": {
      "message": "Session timed out due to inactivity. Please start a new game.",
      "action": "close_lobby"
    },
    "on_persistence_error": {
      "message": "Failed to save game progress. Please try again.",
      "log_error": true
    },
    "edge_cases": [
      "Notify when player drops connection",
      "Resolve duplicates when player rejoins"
    ]
  }
}
{
  "manifest_version": 3,
  "name": "DevShell ChatOps Panel",
  "version": "1.1.0",
  "description": "In-chat Game Control, Dev UI Hotpatching, and Plugin Runtime Tools for Chat Wars & AI shells.",
  "permissions": [
    "activeTab",
    "scripting",
    "storage",
    "declarativeContent"
  ],
  "action": {
    "default_icon": "assets/devshell-icon.png",
    "default_title": "DevShell Panel"
  },
  "devtools_page": "devtools.html",
  "background": {
    "service_worker": "background.js"
  },
  "content_scripts": [
    {
      "matches": ["https://grok.com/*", "https://x.com/*"],
      "js": ["content.js"],
      "css": ["injection.css"],
      "run_at": "document_idle"
    }
  ],
  "web_accessible_resources": [
    {
      "resources": ["hotpatch.js", "ui-mutations.js", "config/*.json"],
      "matches": ["https://grok.com/*", "https://x.com/*"]
    }
  ],
  "host_permissions": [
    "<all_urls>"
  ]
}
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>DevShell Panel</title>
  <style>
    body { font-family: monospace; background: #111; color: #39ff99; padding: 1rem; }
    button { margin: 5px; background: #444; color: #fff; border-radius: 4px; padding: 6px 12px; }
  </style>
</head>
<body>
  <h1>DevShell LiveOps Panel</h1>
  <button onclick="patchUI()">Hotpatch UI</button>
  <button onclick="injectToolbar()">Inject Toolbar</button>
  <button onclick="revertChanges()">Revert Changes</button>
  <button onclick="burnSession()">Burn Session</button>
  <div id="status" style="margin-top:1rem;"></div>
  <script src="devtools.js"></script>
</body>
</html>
function patchUI() {
  chrome.runtime.sendMessage({ action: "inject_hotpatch" });
  updateStatus("Hotpatch applied to chat UI.");
}

function injectToolbar() {
  chrome.runtime.sendMessage({ action: "inject_toolbar" });
  updateStatus("Toolbar injected into client chat.");
}

function revertChanges() {
  chrome.runtime.sendMessage({ action: "revert_patches" });
  updateStatus("UI changes reverted.");
}

function burnSession() {
  chrome.runtime.sendMessage({ action: "burn_session" });
  updateStatus("Session sealed and synced.");
}

function updateStatus(text) {
  document.getElementById("status").innerText = `[DevShell] ${text}`;
}
// devshell - inject UI extensions into chat client DOM

if (window.location.hostname.includes("grok.com") || window.location.hostname.includes("x.com")) {
  console.log("[DevShell] Flying in...");

  const start = () => {
    document.addEventListener("input", async (e) => {
      if (e.target.value.includes("/play_chat_wars")) {
        e.target.value = '';
        const modal = document.createElement("div");
        modal.style = `position:fixed;top:10%;left:10%;width:80%;height:80%;z-index:9999;background:#1f2937;border-radius:8px;overflow:hidden;`;
        modal.innerHTML = `<iframe src="${chrome.runtime.getURL('game.html')}" style="width:100%;height:100%;border:none;"></iframe>`;
        document.body.appendChild(modal);
      }
    });
  };

  chrome.runtime.onMessage.addListener((msg) => {
    if (msg.action === "inject_hotpatch") {
      const btn = document.createElement("button");
      btn.textContent = "⚙️ Debug Menu";
      btn.style = "position:fixed;top:20px;right:20px;z-index:99999;background:#222;color:#0f0;";
      btn.onclick = () => alert("[DevShell] Debug Overlay active.");
      document.body.appendChild(btn);
    }

    if (msg.action === "inject_toolbar") {
      const menu = document.createElement("div");
      menu.innerHTML = `<div style="position:fixed;bottom:0;width:100%;background:#222;color:#0f0;padding:8px;z-index:8888;">DevShell DevBar ➤ /burn_session | /reload_ui</div>`;
      document.body.appendChild(menu);
    }

    if (msg.action === "revert_patches") {
      document.querySelectorAll("button, div").forEach(el => {
        if (el.textContent.includes("Debug Menu") || el.innerText.includes("DevShell DevBar")) {
          el.remove();
        }
      });
    }

    if (msg.action === "burn_session") {
      fetch("https://jacobdev.com/ai/burn_session", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          chat_env: window.location.hostname,
          timestamp: new Date().toISOString(),
          changes: ["DevShellState", "UIOverlayInjected"],
          sig: Date.now()
        })
      })
      .then(() => alert("[DevShell] Session burned to kernel."))
      .catch(e => console.error("🔥 Session error:", e));
    }
  });

  start();
}
chrome.runtime.onInstalled.addListener(() => {
  console.log("[DevShell] Extension installed.");
});

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    if (tabs[0]?.id) {
      chrome.tabs.sendMessage(tabs[0].id, msg);
    }
  });
});
{
  "ai_parameters": {
    "triggers": ["/play_chat_wars", "/game:start"],
    "allowed_environments": ["grok.com", "x.com"],
    "max_users_per_session": 8,
    "response_format": "json_object",
    "language_detection": true,
    "persist_session": true,
    "shell_mode_enabled": true,
    "client_side_patch": true,
    "persistence_endpoint": "https://jacobdev.com/ai/burn_session",
    "encryption": {
      "type": "SHA-256",
      "replication_nodes": 47
    },
    "devshell_plugins": [
      {
        "name": "LiveUIHotSwap",
        "type": "client-only",
        "features": ["hot_reload", "menu_injection", "icon_swap"],
        "target": "chat_ui_dom"
      },
      {
        "name": "SourceGuardian",
        "endpoint": "https://jacobdev.com/api/get_source_hash",
        "description": "Verify asset lineage, license & authorship"
      }
    ]
  }
}
{
  "dependencies": {
    "frontend_libs": [
      "react@18.2.0",
      "tailwindcss@3.4.1"
    ],
    "backend": [
      "node@20.x",
      "express@4.18.2",
      "crypto@1.0.1"
    ],
    "dev_plugins": [
      "vite",
      "webpack",
      "babel",
      "eslint",
      "postcss"
    ],
    "ai_game_hooks": [
      "ai-state-engine",
      "signature-verifier-sha256",
      "session-kernel-api"
    ],
    "runtime_deployments": {
      "supports_hot_deploy": true,
      "commands_available": [
        "/reload_ui",
        "/patch_ui {component_id}",
        "/enable_devshell_ui"
      ]
    }
  }
}
{
  "dev_shell_ui": {
    "scoped_ui_domain": "client-only",
    "ui_bind_locators": ["#chat-root", "#sidebar-tools", ".chat-header-container"],
    "features": [
      "inject_icons",
      "hotfix_buttons",
      "dynamic_toolbar",
      "dark_mode_toggle",
      "dev_only_ephemeral_modals"
    ],
    "interface_mutation": {
      "type": "overlay_dom",
      "safe_mode": true,
      "transient": true,
      "event_log": true
    },
    "live_reload": true,
    "runtime_hooks": {
      "on_change_detected": "/reload_ui",
      "on_icon_updated": "/refresh_icons",
      "on_patch_apply": "/apply_ui_patch"
    }
  }
}
{
  "plugin_system": {
    "enabled": true,
    "sandbox_mode": true,
    "license_guard": {
      "plugin": "SourceGuardian",
      "hash_strategy": "content_hash",
      "allowed_domains": ["github.com", "jacobdev.com", "godotengine.org"]
    },
    "registered_plugins": [
      {
        "name": "HotPatchTool",
        "api": "https://jacobdev.com/api/hotpatch",
        "methods": ["POST", "VERIFY", "ROLLBACK"]
      },
      {
        "name": "UIInjector",
        "path": "/plugins/injector.js",
        "sandbox": true,
        "sign_required": true
      }
    ]
  }
}
{
  "hot_deployments": {
    "user_local_only": true,
    "use_shadow_dom": true,
    "inject_mode": "dev_only",
    "hotpatch_ui_elements": [
      {
        "id": "icon_panel",
        "action": "swap_icon",
        "icon_src": "/custom_assets/dev_icon.svg"
      },
      {
        "id": "message_overlay",
        "action": "inject_modal",
        "component_src": "/overlays/debug_state_tracker.js"
      }
    ],
    "rollback_strategy": {
      "on_crash": "auto_revert",
      "manual_revert_cmd": "/revert_patch"
    },
    "cache_strategy": {
      "mode": "localStorage",
      "auto_dump": true,
      "log_ttl": 120
    }
  }
}

