# https://tails.net/contribute/design/kernel_hardening/
# https://kspp.github.io/Recommended_Settings

# Prevent JIT-spraying.
net.core.bpf_jit_harden = 2
