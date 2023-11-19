![Hello!](.github/readme.jpg)

# Region Policy Evaluator Skeleton

A skeleton WinRT component that can serve as a substitute for the [Region Policy Evaluator][rpe] in Windows. Please note that this is an experiment 🧪 and is not intended for production use.

[rpe]: https://withinrafael.com/2023/11/17/device-region-and-integrated-services-region-policy-in-windows/

## Building
From a Visual Studio Developer Command Prompt, simply run `cargo build`. This specialized environment is needed for access to MIDLRT tooling.

## Installing / Uninstalling

From an **elevated** Visual Studio Developer Command Prompt, run `pwsh scripts\Install.ps1`. Reverse the changes with `pwsh scripts\Uninstall.ps1`. Note: These scripts currently assume a standard Windows installation assigned to letter `C:\`.

Opening Microsoft Edge or Windows Settings will result in the Region Policy Evaluator component load, as of Windows vNext Build 25997. The skeleton component will currently write debug output to `%Temp%\component.log`.