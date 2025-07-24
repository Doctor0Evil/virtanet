-- build_virtasys_structure.lua
local lfs = require "lfs" -- Ensure luafilesystem is installed

-- Function to create directories recursively
local function make_dir_recursive(path)
    -- Split path by separator (assumes '/')
    local parts = {}
    for part in string.gmatch(path, "[^/]+") do
        table.insert(parts, part)
    end

    local current_path = ""
    for i, part in ipairs(parts) do
        current_path = current_path .. part
        if lfs.attributes(current_path, "mode") ~= "directory" then
            local success, err = lfs.mkdir(current_path)
            if not success then
                print("Error creating directory " .. current_path .. ": " .. err)
                return false
            else
                print("Created directory: " .. current_path)
            end
        end
        current_path = current_path .. "/"
    end
    return true
end

-- Function to create an empty file
local function touch_file(filepath)
    local file = io.open(filepath, "w")
    if file then
        file:close()
        print("Created file: " .. filepath)
        return true
    else
        print("Error creating file: " .. filepath)
        return false
    end
end

-- Function to create the structure
local function create_structure(base_path, items)
    for _, item_path in ipairs(items) do
        local full_path = base_path .. "/" .. item_path

        -- Heuristic to determine if it's a file (check for extension or known name)
        local is_file = string.match(item_path, "%.[a-zA-Z0-9]+$") or
                        item_path == "LICENSE" or item_path == "README.md" or
                        item_path == "SECURITY.md" or item_path == "System-Manifest" or
                        item_path == "System-Architecture-hier-view" or item_path == "System.rst" or
                        item_path == "System.sol" or item_path == "Bootloader.md" or
                        item_path == "Commands.mkd" or item_path == "Links.mkd" or
                        item_path == "neuromorph.mkd" or item_path == "cheat_regex.mkd" or
                        item_path == "sys-regex.awk" or item_path == "vsc.html" or
                        item_path == "Post_apoc_surv.html" or item_path == "Game-Dev-env.shell" or
                        item_path == "console_out_in.cmd" or item_path == "user-inputs.vre" or
                        item_path == "Grok.chat.01" or item_path == "cybernetic-system.txt" or
                        item_path == "perplexity.cmd.integration.ipfs.txt" or item_path == "qwen.txt" or
                        item_path == "qwen2.txt"

        if is_file then
            -- Ensure parent directory exists
            local last_sep = string.find(item_path, "/[^/]*$")
            if last_sep then
                local parent_dir = string.sub(item_path, 1, last_sep - 1)
                make_dir_recursive(base_path .. "/" .. parent_dir)
            end
            -- Create the file
            touch_file(full_path)
        else
            -- Create the directory
            make_dir_recursive(full_path)
        end
    end
end

-- Main execution block
local project_root = "Virta-Sys"

-- Define the complete directory and file structure as a table
local structure = {
    ".github/workflows",
    ".github/workflows/cmake-single-platform.yml",
    ".github/workflows/FUNDME.yml",
    "AI/Platform_Access",
    "AI/Platform_Access/AI_Config.json",
    "AI/Platform_Access/AccessToken.cfg",
    "AI/call_model.py",
    "AI/VirtaSysAIPowerParameters.sol",
    "AI/sys_ai.cfg",
    "Chat_wars",
    "Chat_wars/Chat_wars.gd",
    "Chat_wars/original_user_query.rs",
    "game_dev/post_apoc_RPG/maps",
    "game_dev/post_apoc_RPG/AI_model_params.sol",
    "game_dev/post_apoc_RPG/maps/lwalls.lua", -- Inferred from Pasted_Text_1753373547229.txt
    "github/injectors",
    "github/injectors/FUNDME.yml",
    "github/platform_keys",
    "github/platform_keys/Master_key.json",
    "structure",
    "structure/create_repo_structure.bat",
    "system/engine",
    "system/engine/engine.rs",
    "system/engine/mojo_script.mojo",
    "system/engine/system.dll",
    "system/engine/matlab.rs",
    "system/virta_sys_localization.m",
    "system/main.rs",
    "system/package_main.rs",
    "system/menu_system.py",
    "system/menu_config.json",
    "system/plugin1.json",
    "system/generate_documentation.py",
    "system/system_verification.py",
    "system/bootcrapper.ai",
    "system/hardware_integration.py",
    "system/cross_language_auth.py",
    "system/security_hardening.py",
    "system/boot_sequence_tests.py",
    "system/firmware/virta-firmware-ai-core/bin",
    "system/firmware/virta-firmware-ai-core/bin/virta-keygen-cli",
    "system/firmware/virta-firmware-ai-core/bin/virta-capsule-builder",
    "system/firmware/virta-firmware-ai-core/bin/verify-capsule-trust-ai",
    "system/firmware/virta-firmware-ai-core/bin/ai-lock-injector",
    "system/firmware/virta-firmware-ai-core/bin/virta-reporter-traceroute",
    "system/firmware/virta-firmware-ai-core/config",
    "system/firmware/virta-firmware-ai-core/config/LOCKPATCH.virtjson",
    "system/firmware/virta-firmware-ai-core/ai-core",
    "system/firmware/virta-firmware-ai-core/ai-core/validate_merkle.py",
    "system/firmware/virta-firmware-ai-core/ai-core/otacontexter.infer.aiq",
    "system/firmware/virta-firmware-ai-core/otacapsules",
    "system/firmware/virta-firmware-ai-core/otacapsules/VRTAXIOM-B28-20250721.json",
    "system/firmware/virta-firmware-ai-core/keys",
    "system/firmware/virta-firmware-ai-core/keys/ota_chain.json",
    "system/firmware/virta-firmware-ai-core/sys_link",
    "system/firmware/virta-firmware-ai-core/sys_link/ingest_virtual_tracer",
    "system/firmware/virta-firmware-ai-core/logs",
    "system/firmware/virta-firmware-ai-core/logs/ota_audit.log",
    "system/firmware/virta-firmware-ai-core/init_virta_core.sh",
    "system/firmware/virta-firmware-ai-core/README.md",
    "system/firmware/virta-firmware-ai-core/.prebuild.yml",
    "system/firmware/Capsule_OTA_Signer.bat",
    "system/firmware/VR_Launcher.js",
    "system/firmware/SimRunner.bash",
    "user/chats/art",
    "user/chats/links.txt",
    "Documentation",
    "Documentation/MT6883_BIOS_README.md",
    "Documentation/BCI_Telemetry_API.pdf",
    "Documentation/Security_Model.md",
    "Documentation/Compliance_Map.xlsx",
    "Documentation/ChangeLog.txt",
    "Grok.chat.01",
    "cybernetic-system.txt",
    "perplexity.cmd.integration.ipfs.txt",
    "qwen.txt",
    "qwen2.txt",
    "console_out_in.cmd",
    "user-inputs.vre",
    "ABIBreak.cpp",
    "AMDGbuMetadata.cpp",
    "ApFixedPoint.cpp",
    "ApFloat.cpp",
    "ApInt.cpp",
    "ApsInt.cpp",
    "ArmAttributeParser.cpp",
    "ArmBuildAttrs.cpp",
    "Bootloader.md",
    "CheatCodes.dll",
    "Commands.mkd",
    "Death_network_cheat_system.rs",
    "Directory.awk",
    "Game-Dev-env.shell",
    "LICENSE",
    "Links.mkd",
    "NanobotsHardware.dll",
    "Ownership.json",
    "Post_apoc_surv.html",
    "README.md",
    "Regex_patterns.yaml",
    "SECURITY.md",
    "System-Architecture-hier-view",
    "System-Manifest",
    "System.rst",
    "System.sol",
    "System_Regex.json",
    "Vondy_AI.cfg",
    "admin_guide.md",
    "architecture.md",
    "arm_chip_technical_diagrams.json",
    "hardware_config.json",
    "links.json",
    "links_ref.md",
    "neuromorph.mkd",
    "cheat_regex.mkd",
    "sys-regex.awk",
    "ultimate_payment_configuration.yaml",
    "vsc.html"
}

print("Starting Virta-Sys directory structure creation...")
create_structure(project_root, structure)
print("Virta-Sys directory structure creation complete.")
