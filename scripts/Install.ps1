SetACL.exe -on "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator" -ot reg -actn setowner -ownr "n:Administrators"
SetACL.exe -on "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator" -ot reg -actn ace -ace "n:Administrators;p:full"

reg add HKLM\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator /v DllPath /t REG_SZ /d $(Resolve-Path "%~dp0\..\target\debug\component.dll") /f
reg add HKLM\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Internal.System.Profile.RegionPolicyEvaluator /v TrustLevel /t REG_DWORD /d 0 /f
