# Exhaustive directory listing for all drives including V:// in CLF system
# Author: Jacob Scott Farmer (CIA-ID:0047)
# Date: 2025-07-12 21:03 MST

BEGIN {
    print "Listing all drive directories for C://, Z://, p://, VIR://, V://"
}

# C:// (Local System Drive)
function list_c_drive() {
    print "C://"
    print "├── Users"
    print "│   ├── User1"
    print "│   │   ├── Documents"
    print "│   │   │   ├── Work"
    print "│   │   │   └── Personal"
    print "│   │   ├── Downloads"
    print "│   │   └── Pictures"
    print "│   └── User2"
    print "│       ├── Documents"
    print "│       └── Downloads"
    print "├── Windows"
    print "│   ├── System32"
    print "│   ├── SysWOW64"
    print "│   └── Temp"
    print "├── Program Files"
    print "│   ├── App1"
    print "│   └── App2"
    print "├── Program Files (x86)"
    print "│   ├── App3"
    print "│   └── App4"
    print "└── Perflogs"
}

# Z:// (Specialized System Drive, mapped to \\server1\share)
function list_z_drive() {
    print "Z:// (mapped to \\\\server1\\share)"
    print "├── System"
    print "│   ├── Reality_OS"
    print "│   │   ├── Bin"
    print "│   │   └── Config"
    print "│   └── VirtaSys.md"
    print "├── Backups"
    print "│   ├── Data"
    print "│   │   ├── 2025"
    print "│   │   └── 2024"
    print "│   └── Projects"
    print "│       ├── ProjectA"
    print "│       └── ProjectB"
    print "└── Archives"
    print "    ├── Logs"
    print "    │   ├── SystemLogs"
    print "    │   └── AppLogs"
    print "    └── Reports"
    print "        ├── Monthly"
    print "        └── Yearly"
}

# p:// (Project Drive, mapped to \\server2\projects)
function list_p_drive() {
    print "p:// (mapped to \\\\server2\\projects)"
    print "├── Projects"
    print "│   ├── Project1"
    print "│   │   ├── Source"
    print "│   │   └── Docs"
    print "│   └── Project2"
    print "│       ├── Source"
    print "│       └── Docs"
    print "└── Data"
    print "    ├── Datasets"
    print "    │   ├── Dataset1"
    print "    │   └── Dataset2"
    print "    └── Logs"
    print "        ├── Log1"
    print "        └── Log2"
}

# VIR:// (Virtual Drive)
function list_vir_drive() {
    print "VIR://"
    print "├── Virtual"
    print "│   ├── Google"
    print "│   │   └── Drive"
    print "│   │       ├── Backups"
    print "│   │       │   ├── 2025"
    print "│   │       │   └── 2024"
    print "│   │       └── Shared"
    print "│   │           ├── TeamA"
    print "│   │           └── TeamB"
    print "│   └── Dropbox"
    print "│       ├── Files"
    print "│       └── Photos"
    print "└── Cloud"
    print "    ├── AWS"
    print "    │   ├── S3"
    print "    │   └── EC2"
    print "    └── Azure"
    print "        ├── Storage"
    print "        └── VMs"
}

# V:// (Simulation Drive)
function list_v_drive() {
    print "V://"
    print "├── Simulations"
    print "│   ├── Reality"
    print "│   │   ├── Scenarios"
    print "│   │   └── Results"
    print "│   └── OtherSim"
    print "│       ├── Scenarios"
    print "│       └── Results"
    print "├── Models"
    print "│   ├── 3DModels"
    print "│   │   ├── Model1"
    print "│   │   └── Model2"
    print "│   └── DataModels"
    print "│       ├── ModelA"
    print "│       └── ModelB"
    print "└── Results"
    print "    ├── Logs"
    print "    │   ├── SimLog1"
    print "    │   └── SimLog2"
    print "    └── Outputs"
    print "        ├── Output1"
    print "        └── Output2"
}

END {
    list_c_drive()
    print ""
    list_z_drive()
    print ""
    list_p_drive()
    print ""
    list_vir_drive()
    print ""
    list_v_drive()
}
