$scriptpath = $MyInvocation.MyCommand.Path
$dir = Split-Path $scriptpath
Push-Location $dir\..

cargo build
.\target\debug\simplebuffers-compiler.exe --dstdir test cpp c:\Users\zachc\Projects\simple-buffers\test.sb

Pop-Location