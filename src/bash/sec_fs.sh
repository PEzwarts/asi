# https://kspp.github.io/Recommended_Settings

sysctl -w fs.protected_regular=2
sysctl -w fs.protected_fifos=2

sysctl -w fs.protected_hardlinks=1
sysctl -w fs.protected_symlinks=1
