@echo off
setlocal
set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set SYSROOT=C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1+Beta1902
set CCPATH=%SYSROOT%\tools\gcc
set PATH=%PATH%;%AZSPHERETOOLS%;%CCPATH%

strip target\arm-v7-none-eabi\debug\advanced_sphere

mkdir target\approot\bin
copy target\arm-v7-none-eabi\debug\advanced_sphere target\approot\bin\app
copy app_manifest.json target\approot

azsphere image package-application --input target\approot --output target\manual.imagepackage --sysroot 1+Beta1811 -v

echo Now do this:
echo azsphere device sideload delete
echo azsphere device sideload deploy -p target\manual.imagepackage
echo.

