
import matplotlib
matplotlib.use('Agg') 
import matplotlib.pyplot as plt
plt.ioff()
import os
os.environ['TERM'] = 'dumb'
import unittest
import os
import json
from security_hardening import BootSecurity
from hardware_integration import HardwareDetector
from menu_system import MenuSystem
from cross_language_auth import AuthStandardizer

class TestBootSequence(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        """Set up test environment"""
        cls.security = BootSecurity()
        cls.hardware = HardwareDetector()
        cls.menu = MenuSystem()
        cls.auth = AuthStandardizer()
        
        
        cls.bootloader_path = "test_loader.bin"
        with open(cls.bootloader_path, "wb") as f:
            f.write(b"TEST_BOOTLOADER_DATA")
        
  
        cls.security.sign_bootloader(cls.bootloader_path)

    def test_security_verification(self):
        """Test bootloader signature verification"""
     
        self.assertTrue(self.security.verify_boot_signature(self.bootloader_path))
        
       permanently changing file)
        original_data = None
        with open(self.bootloader_path, "rb") as f:
            original_data = f.read()
        
        try:
            with open(self.bootloader_path, "wb") as f:
                f.write(b"MODIFIED_DATA")
            self.assertFalse(self.security.verify_boot_signature(self.bootloader_path))
        finally:
            
            with open(self.bootloader_path, "wb") as f:
                f.write(original_data)
           
            self.security.sign_bootloader(self.bootloader_path)

    def test_hardware_detection(self):
        """Test hardware detection"""
        hw_info = self.hardware.get_hardware_info()
        
        
        self.assertIn('cpu', hw_info)
        self.assertIn('memory', hw_info)
        self.assertIn('storage', hw_info)
        self.assertIn('os', hw_info)
        
       
        self.assertGreater(hw_info['cpu']['cores'], 0)
        self.assertGreater(hw_info['cpu']['threads'], 0)
        
     
        self.assertGreater(hw_info['memory']['total_gb'], 0)
        
       
        self.assertGreater(hw_info['storage']['total_gb'], 0)

    def test_menu_structure(self):
        """Test menu system structure"""
        self.menu.build_menu()
        menu_config = self.menu.root_menu.to_dict()
        
        self.assertEqual(menu_config['title'], 'Main Menu')
        
       
        system_menu = next(
            (item for item in menu_config['children'] if item['title'] == 'System'),
            None
        )
        self.assertIsNotNone(system_menu)
        
      
        plugin_menus = [
            item for item in menu_config['children'] 
            if item['command_type'] == 'plugin'
        ]
        self.assertGreater(len(plugin_menus), 0)

    def test_auth_standardization(self):
        """Test authentication standardization"""
      
        self.assertFalse(self.auth.validate_password("weak"))
        self.assertTrue(self.auth.validate_password("StrongPassword123!"))
        
        
        password = "TestPassword123!"
        pwd_hash = self.auth.hash_password(password)
        self.assertTrue(self.auth.verify_password(password, pwd_hash))
        self.assertFalse(self.auth.verify_password("wrong", pwd_hash))

if __name__ == "__main__":
   
    unittest.main(verbosity=2)
