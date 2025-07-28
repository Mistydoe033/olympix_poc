[Setup]
AppName=Olympix Pattern Tool
AppVersion=1.0.0
DefaultDirName={autopf}\Olympix Pattern Tool
DefaultGroupName=Olympix Pattern Tool
OutputBaseFilename=olympix_pattern_tool-installer
Compression=lzma
SolidCompression=yes
DisableProgramGroupPage=no
SetupIconFile=favicon.ico
UninstallDisplayIcon={app}\olympix_pattern_tool.exe
PrivilegesRequired=lowest

[Files]
Source: "target\release\olympix-pattern-tool.exe"; DestDir: "{app}"; DestName: "olympix_pattern_tool.exe"; Flags: ignoreversion
Source: "contracts\*"; DestDir: "{userdocs}\Olympix Pattern Tool\contracts"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "patterns\*"; DestDir: "{userdocs}\Olympix Pattern Tool\patterns"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "favicon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Olympix Pattern Tool"; Filename: "{app}\olympix_pattern_tool.exe"; IconFilename: "{app}\favicon.ico"; IconIndex: 0
Name: "{autodesktop}\Olympix Pattern Tool"; Filename: "{app}\olympix_pattern_tool.exe"; IconFilename: "{app}\favicon.ico"; IconIndex: 0; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Additional icons:"

[Run]
Filename: "{app}\olympix_pattern_tool.exe"; Description: "Launch Olympix Pattern Tool"; Flags: nowait postinstall skipifsilent