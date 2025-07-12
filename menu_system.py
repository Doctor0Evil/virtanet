import matplotlib
matplotlib.use('Agg')  # Set the backend to non-interactive
import matplotlib.pyplot as plt
plt.ioff()
import os
os.environ['TERM'] = 'dumb'
import os
import json
from typing import List, Dict

class MenuItem:
    def __init__(self, title: str, command_type: str, children: List['MenuItem'] = None):
        self.title = title
        self.command_type = command_type
        self.children = children or []

    def add_child(self, child: 'MenuItem'):
        self.children.append(child)

    def to_dict(self) -> Dict:
        return {
            'title': self.title,
            'command_type': self.command_type,
            'children': [child.to_dict() for child in self.children]
        }

class PluginManager:
    def __init__(self, plugin_dir: str = 'plugins'):
        self.plugin_dir = plugin_dir
        os.makedirs(plugin_dir, exist_ok=True)

    def load_plugins(self) -> List[Dict]:
        plugins = []
        for filename in os.listdir(self.plugin_dir):
            if filename.endswith('.json'):
                try:
                    with open(os.path.join(self.plugin_dir, filename)) as f:
                        plugin_data = json.load(f)
                        plugins.append(plugin_data)
                except json.JSONDecodeError:
                    continue
        return plugins

class MenuSystem:
    def __init__(self):
        self.root_menu = MenuItem("Main Menu", "root")
        self.plugin_manager = PluginManager()

    def build_menu(self):
        """Build the base menu structure"""
        # Core system menu
        system_menu = MenuItem("System", "system")
        system_menu.add_child(MenuItem("Info", "system_info"))
        system_menu.add_child(MenuItem("Settings", "system_settings"))
        self.root_menu.add_child(system_menu)

        # Load plugins and add their menu items
        plugins = self.plugin_manager.load_plugins()
        for plugin in plugins:
            plugin_menu = MenuItem(plugin.get('name', 'Plugin'), 'plugin')
            for command in plugin.get('commands', []):
                plugin_menu.add_child(MenuItem(
                    command.get('name', 'Command'),
                    command.get('type', 'plugin_command')
                ))
            self.root_menu.add_child(plugin_menu)

    def save_menu_config(self, path: str = 'menu_config.json'):
        """Save the menu structure to a file"""
        with open(path, 'w') as f:
            json.dump(self.root_menu.to_dict(), f, indent=2)
    def print_menu(self):
        """Print the menu structure"""
        print("Current Menu Structure:")
        print(json.dumps(self.root_menu.to_dict(), indent=2))
if __name__ == "__main__":
    os.makedirs('plugins', exist_ok=True)
    sample_plugins = [
        {
            'name': 'Sample Plugin',
            'commands': [
                {'name': 'Run Test', 'type': 'test_command'},
                {'name': 'Configure', 'type': 'config_command'}
            ]
        },
        {
            'name': 'Diagnostics',
            'commands': [
                {'name': 'System Check', 'type': 'diagnostic'},
                {'name': 'Network Test', 'type': 'diagnostic'}
            ]
        }
    ]

    for i, plugin in enumerate(sample_plugins):
        with open(f'plugins/plugin_{i}.json', 'w') as f:
            json.dump(plugin, f)

    menu_system = MenuSystem()
    menu_system.build_menu()
    menu_system.print_menu()
    menu_system.save_menu_config()
    print("\nMenu configuration saved to menu_config.json")
