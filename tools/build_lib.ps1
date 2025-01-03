# build_lib.ps1

# Function to find the project root by locating Cargo.toml
function Get-ProjectRoot {
    $currentDir = Get-Location
    while ($currentDir -ne $currentDir.Parent) {
        if (Test-Path (Join-Path $currentDir "Cargo.toml")) {
            return $currentDir
        }
        $currentDir = $currentDir.Parent
    }
    Throw "Error: Cargo.toml not found in any parent directories."
}

try {
    # Step 1: Find the project root
    $projectRoot = Get-ProjectRoot
    Write-Host "Project root found at: $projectRoot" -ForegroundColor Green

    # Navigate to the project root
    Set-Location $projectRoot

    # Step 2: Build the Rust project in release mode
    Write-Host "Building the Rust project in release mode..." -ForegroundColor Cyan
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Throw "Error: Cargo build failed."
    }
    Write-Host "Build completed successfully." -ForegroundColor Green

    # Define paths
    $targetReleasePath = Join-Path $projectRoot "target\release"
    $buildLibDir = Join-Path $projectRoot "build_lib"

    # Step 3: Create the build_lib directory if it doesn't exist
    if (!(Test-Path $buildLibDir)) {
        New-Item -ItemType Directory -Path $buildLibDir | Out-Null
        Write-Host "Created build_lib directory at: $buildLibDir" -ForegroundColor Yellow
    }

    # Step 4.1: Copy only smn_view.dll to the build_lib directory
    Write-Host "Copying 'smn_view.dll' to the build_lib directory..." -ForegroundColor Cyan
    $libSource = Join-Path $targetReleasePath "smn_view.dll"
    if (Test-Path $libSource) {
        Copy-Item -Path $libSource -Destination $buildLibDir -Force
        Write-Host "'smn_view.dll' copied successfully." -ForegroundColor Green
    } else {
        Write-Warning "Warning: 'smn_view.dll' not found at $libSource. Skipping copy."
    }

    # Step 4.2: Copy all .dll files from deps to build_lib\deps
    Write-Host "Copying all '.dll' files from 'deps' to the build_lib directory..." -ForegroundColor Cyan
    $depsSource = Join-Path $targetReleasePath "deps"
    $depsDestination = Join-Path $buildLibDir "deps"

    if (Test-Path $depsSource) {
        # Create the 'deps' directory in build_lib if it doesn't exist
        if (!(Test-Path $depsDestination)) {
            New-Item -ItemType Directory -Path $depsDestination | Out-Null
            Write-Host "Created 'deps' directory at: $depsDestination" -ForegroundColor Yellow
        }

        # Get all .dll files in depsSource and copy them to depsDestination
        Get-ChildItem -Path $depsSource -Filter *.dll | ForEach-Object {
            Copy-Item -Path $_.FullName -Destination $depsDestination -Force
        }
        Write-Host "All '.dll' files from 'deps' folder copied successfully." -ForegroundColor Green
    } else {
        Write-Warning "Warning: 'deps' folder not found at $depsSource. Skipping copy of dependencies."
    }

    # Step 5: Attempt to publish the crate
    Write-Host "Attempting to publish the crate..." -ForegroundColor Cyan
    cargo publish
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Warning: Cargo publish failed. Please check the error messages above."
    } else {
        Write-Host "Crate published successfully." -ForegroundColor Green
    }

    Write-Host "All tasks completed successfully!" -ForegroundColor Green

} catch {
    Write-Error $_.Exception.Message
    exit 1
}
