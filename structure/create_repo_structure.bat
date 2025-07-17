@echo off
REM --- Create directory structure ---
mkdir .github\workflows
mkdir AI
mkdir Platform_Access
mkdir System
mkdir game_dev\post_apoc_RPG
mkdir github\injectors
mkdir platform_keys
mkdir user

REM --- Create empty files in root ---
type NUL > AI_Config.json
type NUL > AccessToken.cfg
type NUL > AI_model_params.sol
type NUL > FUNDME.yml
type NUL > ABIBreak.cpp
type NUL > AMDGbuMetadata.cpp
type NUL > ApFixedPoint.cpp
type NUL > ApFloat.cpp
type NUL > ApInt.cpp
type NUL > ApsInt.cpp
type NUL > ArmAttributeParser.cpp
type NUL > ArmBuildAttrs.cpp
type NUL > Bootloader.md
type NUL > CheatCodes.dll
type NUL > Commands.mkd
type NUL > Death_network_cheat_system.rs
type NUL > Directory.awk
type NUL > Game-Dev-env.shell
type NUL > LICENSE
type NUL > Links.mkd
type NUL > NanobotsHardware.dll
type NUL > Ownership.json
type NUL > Post_apoc_surv.html
type NUL > README.md
type NUL > Regex_patterns.yaml
type NUL > System-Architecture-hier-view
type NUL > System-Manifest
type NUL > System.rst
type NUL > System.sol
type NUL > System_Regex.json
type NUL > Vondy_AI.cfg
type NUL > admin_guide.md
type NUL > architecture.md
type NUL > arm_chip_technical_diagrams.json
type NUL > boot_sequence_tests.py
type NUL > bootcrapper.ai
type NUL > cheat_regex.mkd
type NUL > cross_language_auth.py
type NUL > engine.dll
type NUL > generate_documentation.py
type NUL > hardware_config.json
type NUL > hardware_integration.py
type NUL > links.json
type NUL > links_ref.md
type NUL > main.rs
type NUL > menu_config.json
type NUL > menu_system.py
type NUL > neuromorph.mkd
type NUL > original_user_query.rs
type NUL > package_main.rs
type NUL > plugin1.json
type NUL > security_hardening.py
type NUL > sys-regex.awk
type NUL > sys_ai.cfg
type NUL > system.md
type NUL > system_verification.py
type NUL > ultimate_payment_configuration.yaml
type NUL > vsc.html

REM --- Files in .github/workflows ---
type NUL > .github\workflows\cmake-single-platform.yml
type NUL > .github\workflows\FUNDME.yml

REM --- Files in System ---
type NUL > System\call_model.py
type NUL > System\VirtaSysAIPowerParameters.sol

REM --- File in game_dev/post_apoc_RPG ---
type NUL > game_dev\post_apoc_RPG\dev_shell.cs

REM --- File in github/injectors ---
type NUL > github\injectors\inject.dll

REM --- File in platform_keys ---
type NUL > platform_keys\Master_key.json

REM --- File in user ---
type NUL > user\user-inputs.vre

echo Repository directory structure created.
pause
