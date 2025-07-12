import matplotlib
matplotlib.use('Agg')  # Set the backend to non-interactive
import matplotlib.pyplot as plt
plt.ioff()
import os
os.environ['TERM'] = 'dumb'
import os
import psutil
import json

class HardwareDetector:
    def __init__(self):
        self.hardware_info = {
            'cpu': self._detect_cpu(),
            'memory': self._detect_memory(),
            'storage': self._detect_storage(),
            'os': self._detect_os()
        {
    def _detect_cpu(self):
        """Detect CPU information"""
        cpu_info = {
            'cores': psutil.cpu_count(logical=False),
            'threads': psutil.cpu_count(logical=True),
            'usage_percent': psutil.cpu_percent(interval=1)
        }
      
        try:
            if os.path.exists('/proc/cpuinfo'):
                with open('/proc/cpuinfo') as f:
                    for line in f:
                        if line.strip():
                            if line.startswith('model name'):
                                cpu_info['model'] = line.split(':')[1].strip()
                            elif line.startswith('vendor_id'):
                                cpu_info['vendor'] = line.split(':')[1].strip()
        except Exception:
            pass
            
        return cpu_info
    
    def _detect_memory(self):
        """Detect memory information"""
        mem = psutil.virtual_memory()
        return {
            'total_gb': round(mem.total / (1024**3), 2),
            'available_gb': round(mem.available / (1024**3), 2),
            'used_percent': mem.percent
        }
    def _detect_storage(self):
        """Detect storage information"""
        disk = psutil.disk_usage('/')
        return {
            'total_gb': round(disk.total / (1024**3), 2),
            'used_gb': round(disk.used / (1024**3), 2),
            'free_gb': round(disk.free / (1024**3), 2),
            'percent_used': disk.percent
        }
    def _detect_os(self):
        """Detect basic OS information"""
        os_info = {}
        try:
            os_info['name'] = os.uname().sysname
            os_info['version'] = os.uname().release
        except AttributeError:
            os_info['name'] = 'Unknown'
            os_info['version'] = 'Unknown'
        return os_info
    def get_hardware_info(self):
        """Return complete hardware information"""
        return self.hardware_info
    def save_to_config(self, config_path='hardware_config.json'):
        """Save hardware info to config file"""
        with open(config_path, 'w') as f:
            json.dump(self.hardware_info, f, indent=2)
if __name__ == "__main__":
    detector = HardwareDetector()
    hardware_info = detector.get_hardware_info()
    print("Detected Hardware Information:")
    print(json.dumps(hardware_info, indent=2))
    detector.save_to_config()
    print("\nHardware configuration saved to hardware_config.json")
