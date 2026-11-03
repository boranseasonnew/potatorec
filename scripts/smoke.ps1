param([string]$Executable, [switch]$Audio, [switch]$Microphone)
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
if (!$Executable) { $Executable=Join-Path $root 'target\release\potato-recorder-cli.exe' }
$output=Join-Path $root 'test-output\manual-smoke'
$arguments=@('--self-test',$output,'--rate','60000/1001','--resolution','1280x720','--pause','--stems')
if($Audio){$arguments+='--audio'}
if($Microphone){$arguments+=@('--mic','--rnnoise')}
& $Executable @arguments
if($LASTEXITCODE -ne 0){throw 'Recording smoke test failed'}
$latest=Get-ChildItem -LiteralPath $output -File -Filter '*.mkv' | Sort-Object LastWriteTime -Descending | Select-Object -First 1
& ffmpeg -v error -i $latest.FullName -map 0 -f null -
if($LASTEXITCODE -ne 0){throw 'Output decode failed'}
Write-Output "Verified: $($latest.FullName)"
