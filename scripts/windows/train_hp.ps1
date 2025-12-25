# High-performance training helper for Windows/NVIDIA.
# Sets wgpu env vars to prefer the discrete GPU, then calls the cargo alias.

param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $Args
)

Push-Location (Split-Path -Parent $MyInvocation.MyCommand.Path)

$env:WGPU_POWER_PREF = "high-performance"
$env:WGPU_BACKEND = "dx12"
# Uncomment if you have multiple adapters and want to force NVIDIA:
# $env:WGPU_ADAPTER_NAME = "NVIDIA"

cargo train_hp -- @Args

Pop-Location
