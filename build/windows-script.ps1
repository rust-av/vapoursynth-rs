# Get the arch and dir
param([String]$arch="x86_64")

$TARGET_DIR = "C:\Program Files\VapourSynth"

# Determine Python version argument based on architecture
If ($arch -eq "i686") {
        Write-Error "32-bit builds are not supported for VapourSynth R73"
        Exit 1
} Else {
        # R73 requires Python 3.12+, using 3.13 as it's well-supported
        $PYTHON_ARG = "-Python313"
}

# Download and run the official VapourSynth portable installation script
# This script automatically downloads Python embeddable and VapourSynth, and properly installs the wheel package
$VS_INSTALLER_URL = "https://github.com/vapoursynth/vapoursynth/releases/download/R73/Install-Portable-VapourSynth-R73.ps1"

Write-Host "Downloading VapourSynth R73 portable installer..."
$installerScript = Invoke-RestMethod $VS_INSTALLER_URL

Write-Host "Running VapourSynth portable installer..."
Invoke-Expression "& {$installerScript} -TargetFolder '$TARGET_DIR' $PYTHON_ARG -Unattended"
