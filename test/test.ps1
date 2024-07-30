$scriptpath = $MyInvocation.MyCommand.Path
$dir = Split-Path $scriptpath
Push-Location $dir\..

cargo build
.\target\debug\simplebuffers-compiler.exe --dstdir test cpp .\test\test.sb

Pop-Location