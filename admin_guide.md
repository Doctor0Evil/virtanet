Administrator Guide (Generated 2025-07-
12T21:10:34.170678)
Menu System Operations
1. Navigate using menu numbers or names
2. Type 'EXIT' to go back
3. Use TAB/ENTER for accessibility
Security Configuration
Set password policies in auth_config.json
Manage cryptographic keys in boot_private_key.pem
Configure allowed commands in menu_config.json
Plugin Management
1. Place plugin JSON files in /plugins directory
2. Required format: { "name": "PluginName", "version": "1.0.0", "commands": [ { "name":
"CommandName", "type": "command_type" } ] }
Hardware Configuration
View detected hardware in hardware_config.json
Override automatic detection if needed
