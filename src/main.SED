[Version]
Class=IEXPRESS
SEDVersion=3
[Options]
PackagePurpose=InstallApp
ShowInstallProgramWindow=0
HideExtractAnimation=0
UseLongFileName=0
InsideCompressed=0
CAB_FixedSize=0
CAB_ResvCodeSigning=0
RebootMode=N
InstallPrompt=%InstallPrompt%
DisplayLicense=%DisplayLicense%
FinishMessage=%FinishMessage%
TargetName=%TargetName%
FriendlyName=%FriendlyName%
AppLaunched=%AppLaunched%
PostInstallCmd=%PostInstallCmd%
AdminQuietInstCmd=%AdminQuietInstCmd%
UserQuietInstCmd=%UserQuietInstCmd%
SourceFiles=SourceFiles
[Strings]
InstallPrompt=Ready to dive into sheller?
DisplayLicense=D:\Programming\Rust\shell\LICENSE
FinishMessage=Delve into Sheller, cuz we are done installing!
TargetName=D:\Programming\Rust\shell\src\main.exe
FriendlyName=Sheller installation Wizard v1.0.0
AppLaunched=main.exe
PostInstallCmd=main.exe
AdminQuietInstCmd=
UserQuietInstCmd=
FILE0="main.exe"
[SourceFiles]
SourceFiles0=D:\Programming\Rust\shell\src\
[SourceFiles0]
%FILE0%=
