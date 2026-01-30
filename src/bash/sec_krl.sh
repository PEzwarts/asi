# https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html
# https://tails.net/contribute/design/kernel_hardening/
# https://github.com/torvalds/linux/blob/master/Documentation/security/self-protection.rst

# Ptrace restriction.
sysctl -w kernel.yama.ptrace_scope=2

# Improve ASLR randomization.
vm.mmap_rnd_bits=32
vm.mmap_rnd_compat_bits=16
kernel.randomize_va_space=2

# Prevents kexec from replacing the running kernel.
kernel.kexec_load_disabled=1

# Prevents malicious software from reading exposed kernel addresses from /proc/kallsyms .
kernel.kptr_restrict=2

# Randomizes kernel stack offset on syscall entry.
sysctl -w randomize_kstack_offset=on

# Prevents kernel addresses exposure via dmesg.
sysctl -w kernel.dmesg_restrict=1

# Disable modules loading.
sysctl -w kernel.modules_disabled=1

# Disable user namespaces.
sysctl -w user.max_user_namespaces=0
