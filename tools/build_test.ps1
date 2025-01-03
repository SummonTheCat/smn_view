# BuildRustProject.ps1

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
    $buildResult = cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Throw "Error: Cargo build failed."
    }
    Write-Host "Build completed successfully." -ForegroundColor Green

    # Define paths
    $targetReleasePath = Join-Path $projectRoot "target\release"
    $buildDir = Join-Path $projectRoot "build_test"

    # Step 3: Create the build directory if it doesn't exist
    if (!(Test-Path $buildDir)) {
        New-Item -ItemType Directory -Path $buildDir | Out-Null
        Write-Host "Created build directory at: $buildDir" -ForegroundColor Yellow
    }

    # Step 4: Copy specific built artifacts to the build directory
    Write-Host "Copying 'deps' folder and 'smn_view_test.exe' to the build directory..." -ForegroundColor Cyan

    # Copy the 'deps' folder with only .dll filessmn_view_test
    $depsSource = Join-Path $targetReleasePath "deps"
    $depsDestination = Join-Path $buildDir "deps"
    if (Test-Path $depsSource) {
        # Create the 'deps' directory in build if it doesn't exist
        if (!(Test-Path $depsDestination)) {
            New-Item -ItemType Directory -Path $depsDestination | Out-Null
            Write-Host "Created 'deps' directory at: $depsDestination" -ForegroundColor Yellow
        }

        # Get all .dll files in depsSource and copy them to depsDestination
        Get-ChildItem -Path $depsSource -Filter *.dll | ForEach-Object {
            Copy-Item -Path $_.FullName -Destination $depsDestination -Force
        }
        Write-Host "Only '.dll' files from 'deps' folder copied successfully." -ForegroundColor Green
    } else {
        Write-Warning "Warning: 'deps' folder not found at $depsSource. Skipping copy."
    }

    # Copy the 'smn_view_test.exe' executable
    $exeSource = Join-Path $targetReleasePath "smn_view_test.exe"
    if (Test-Path $exeSource) {
        Copy-Item -Path $exeSource -Destination $buildDir -Force
        Write-Host "'smn_view_test.exe' copied successfully." -ForegroundColor Green
    } else {
        Write-Warning "Warning: 'smn_view_test.exe' not found at $exeSource. Skipping copy."
    }

    # Step 5: Copy the entire 'statics' folder to the build directory
    $staticsSource = Join-Path $projectRoot "statics"
    $staticsDestination = Join-Path $buildDir "statics"
    if (Test-Path $staticsSource) {
        Copy-Item -Path $staticsSource -Destination $buildDir -Recurse -Force
        Write-Host "'statics' folder copied successfully." -ForegroundColor Green
    } else {
        Write-Warning "Warning: 'statics' folder not found at $staticsSource. Skipping copy."
    }

    Write-Host "All tasks completed successfully!" -ForegroundColor Green

} catch {
    Write-Error $_.Exception.Message
    exit 1
}
