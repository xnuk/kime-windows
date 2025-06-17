# sudo pwsh .\debug.ps1
#Requires -RunAsAdministrator
regsvr32.exe '.\target\debug\tsf_again.dll'
notepad.exe | out-null
regsvr32.exe /u '.\target\debug\tsf_again.dll'
