$ID = $args[0];
Invoke-WebRequest -Uri "https://github.com/Drew-Chase/Curseforge-Pack-Downloader/releases/download/1.0.0/curseforge_pack_downloader-windows-1.0.0.exe" -OutFile "curseforge_pack_downloader-windows-1.0.0.exe"
Start-Process "curseforge_pack_downloader-windows-1.0.0.exe" -ArgumentList @("--id", "$ID") -NoNewWindow -Wait
Remove-Item "curseforge_pack_downloader-windows-1.0.0.exe"