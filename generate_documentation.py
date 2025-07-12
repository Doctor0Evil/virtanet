
import matplotlib
matplotlib.use('Agg')  
import matplotlib.pyplot as plt
plt.ioff()
import os
os.environ['TERM'] = 'dumb'

import os
from datetime import datetime

class DocumentationGenerator:
    def __init__(self):
        self.docs_dir = "docs"
        os.makedirs(self.docs_dir, exist_ok=True)
        
    def generate_architecture_diagram(self):
        """Generate architecture diagram in markdown"""
        content = f"""# Architecture Diagram (Generated {datetime.now().isoformat()})

## Boot Sequence
1. **MinimalLoader** (ROM/BIOS)
   - Hardware initialization
   - Secure boot verification

2. **IntermediateLoader**
   - Memory setup
   - Driver initialization

3. **SystemMenuShell**
   - Menu-driven interface
   - Plugin integration

## Security Model
- Cryptographic boot verification
- Hardware-based authentication
- Code reproduction protection

## Hardware Abstraction
- Automatic hardware detection
- Vendor-specific drivers
- Fallback mechanisms

## Plugin System
- JSON-based configuration
- Dynamic menu integration
- Version verification
"""
        with open(os.path.join(self.docs_dir, "architecture.md"), "w") as f:
            f.write(content)
        return "architecture.md"

    def generate_admin_guide(self):
        """Generate administrator guide"""
        content = f"""# Administrator Guide (Generated {datetime.now().isoformat()})

## Menu System Operations
1. Navigate using menu numbers or names
2. Type 'EXIT' to go back
3. Use TAB/ENTER for accessibility

## Security Configuration
- Set password policies in auth_config.json
- Manage cryptographic keys in boot_private_key.pem
- Configure allowed commands in menu_config.json

## Plugin Management
1. Place plugin JSON files in /plugins directory
2. Required format:
{{
  "name": "PluginName",
  "version": "1.0.0",
  "commands": [
    {{
      "name": "CommandName",
      "type": "command_type"
    }}
  ]
}}

## Hardware Configuration
- View detected hardware in hardware_config.json
- Override automatic detection if needed
"""
        with open(os.path.join(self.docs_dir, "admin_guide.md"), "w") as f:
            f.write(content)
        return "admin_guide.md"

    def generate_all_docs(self):
        """Generate all documentation files"""
        files = [
            self.generate_architecture_diagram(),
            self.generate_admin_guide()
        ]
        print(f"Documentation generated: {', '.join(files)}")

if __name__ == "__main__":
    generator = DocumentationGenerator()
    generator.generate_all_docs()
