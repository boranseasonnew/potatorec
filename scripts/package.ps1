$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$exe=Join-Path $root 'target\release\potato-recorder.exe'
if (!(Test-Path -LiteralPath $exe)) { throw 'Build first: cargo build --release --locked' }
$destination=Join-Path $root 'dist\Potato Recorder'
New-Item -ItemType Directory -Force $destination | Out-Null
Copy-Item -LiteralPath $exe -Destination $destination
Copy-Item -LiteralPath (Join-Path $root 'target\release\potato-recorder-cli.exe') -Destination $destination
foreach($name in @('README.md','LICENSE','THIRD_PARTY_NOTICES.md')) {
    Copy-Item -LiteralPath (Join-Path $root $name) -Destination $destination
}
Copy-Item -LiteralPath (Join-Path $root 'docs') -Destination $destination -Recurse -Force
$metadata=& (Join-Path $root 'scripts\cargo.ps1') metadata --locked --format-version 1 | ConvertFrom-Json
$licenseRoot=Join-Path $destination 'third-party-licenses'
foreach($package in $metadata.packages) {
    if($package.name -in @('potato-core','potato-recorder')) {continue}
    $sourceRoot=Split-Path $package.manifest_path -Parent
    $licenseDestination=Join-Path $licenseRoot ($package.name+'-'+$package.version)
    $files=Get-ChildItem -LiteralPath $sourceRoot -File | Where-Object Name -Match '^(LICENSE|LICENCE|COPYING|NOTICE|AUTHORS|COPYRIGHT)'
    if($files) {New-Item -ItemType Directory -Force $licenseDestination | Out-Null; foreach($file in $files){Copy-Item -LiteralPath $file.FullName -Destination $licenseDestination}}
    foreach($directory in @('licenses','LICENSES')) {
        $licenseDirectory=Join-Path $sourceRoot $directory
        if(Test-Path -LiteralPath $licenseDirectory -PathType Container){Copy-Item -LiteralPath $licenseDirectory -Destination (Join-Path $licenseDestination 'licenses') -Recurse -Force;break}
    }
}
$version=($metadata.packages | Where-Object name -EQ 'potato-recorder' | Select-Object -First 1).version
$archive=Join-Path $root ('dist\Potato-Recorder-'+$version+'-windows-x64.zip')
Get-ChildItem -LiteralPath $licenseRoot -Recurse -File | Where-Object LastWriteTime -LT ([datetime]'1980-01-01') | ForEach-Object {$_.LastWriteTime=[datetime]'1980-01-01'}
Compress-Archive -LiteralPath $destination -DestinationPath $archive -Force
Get-FileHash -LiteralPath $exe -Algorithm SHA256 | Format-List
Write-Output "Portable package: $archive"
Write-Output 'FFmpeg/ffprobe are external requirements and were not bundled.'
