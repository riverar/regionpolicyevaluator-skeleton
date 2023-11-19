SetACL.exe -on "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator" -ot reg -actn setowner -ownr "n:TrustedInstaller"
SetACL.exe -on "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator" -ot reg -actn ace -ace "n:Administrators;p:read"

reg add HKLM\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator /v DllPath /t REG_SZ /d "C:\Windows\System32\SystemSettings.DataModel.dll" /f
reg add HKLM\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator /v TrustLevel /t REG_DWORD /d 2 /f
