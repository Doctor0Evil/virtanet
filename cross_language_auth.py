import matplotlib
matplotlib.use('Agg') 
import matplotlib.pyplot as plt
plt.ioff()
import os
os.environ['TERM'] = 'dumb'
import os
import json
import hashlib
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.backends import default_backend
import base64

class AuthStandardizer:
    def __init__(self):
        self.auth_config = {
            'min_password_length': 12,
            'required_special_chars': 1,
            'required_digits': 1,
            'required_uppercase': 1,
            'salt_length': 16,
            'iterations': 100000,
            'hash_algorithm': 'sha256'
        }
        
    def validate_password(self, password: str) -> bool:
        """Validate password against security requirements"""
        if len(password) < self.auth_config['min_password_length']:
            return False
            
        counts = {
            'special': 0,
            'digit': 0,
            'upper': 0
        }
        
        for char in password:
            if char in '!@#$%^&*()_+-=[]{}|;:,.<>?':
                counts['special'] += 1
            elif char.isdigit():
                counts['digit'] += 1
            elif char.isupper():
                counts['upper'] += 1
                
        return (counts['special'] >= self.auth_config['required_special_chars'] and
                counts['digit'] >= self.auth_config['required_digits'] and
                counts['upper'] >= self.auth_config['required_uppercase'])
    
    def hash_password(self, password: str) -> str:
        """Hash password using PBKDF2 for consistent cross-language hashing"""
        salt = os.urandom(self.auth_config['salt_length'])
        kdf = PBKDF2HMAC(
            algorithm=hashes.SHA256(),
            length=32,
            salt=salt,
            iterations=self.auth_config['iterations'],
            backend=default_backend()
        )
        key = kdf.derive(password.encode())
        return f"{base64.b64encode(salt).decode()}:{base64.b64encode(key).decode()}"
    
    def verify_password(self, password: str, stored_hash: str) -> bool:
        """Verify password against stored hash"""
        try:
            salt_b64, key_b64 = stored_hash.split(':')
            salt = base64.b64decode(salt_b64.encode())
            stored_key = base64.b64decode(key_b64.encode())
            
            kdf = PBKDF2HMAC(
                algorithm=hashes.SHA256(),
                length=32,
                salt=salt,
                iterations=self.auth_config['iterations'],
                backend=default_backend()
            )
            new_key = kdf.derive(password.encode())
            return new_key == stored_key
        except Exception:
            return False
    
    def save_auth_config(self, path: str = 'auth_config.json'):
        """Save authentication configuration to file"""
        with open(path, 'w') as f:
            json.dump(self.auth_config, f, indent=2)


if __name__ == "__main__":
    auth = AuthStandardizer()
    
    test_passwords = [
        ("Weak1!", False),  
        ("StrongPassword1", False),  
        ("StrongPassword!", False),  
        ("strongpassword1!", False),  
        ("StrongPassword1!", True)  
    ]
    
    print("Password Validation Tests:")
    for pwd, expected in test_passwords:
        result = auth.validate_password(pwd)
        print(f"{pwd}: {'PASS' if result == expected else 'FAIL'} (Expected: {expected}, Got: {result})")
    
  
    password = "SecurePassword123!"
    if auth.validate_password(password):
        pwd_hash = auth.hash_password(password)
        print(f"\nPassword hash: {pwd_hash}")
        print(f"Verification test: {auth.verify_password(password, pwd_hash)}")
        print(f"Wrong password test: {auth.verify_password('wrong', pwd_hash)}")
    else:
        print("\nPassword doesn't meet requirements")
    auth.save_auth_config()
    print("\nAuthentication configuration saved to auth_config.json")
