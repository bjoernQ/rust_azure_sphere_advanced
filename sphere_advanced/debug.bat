@echo off
setlocal
set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set SYSROOT=C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1+Beta1902
set PATH=%PATH%;%AZSPHERETOOLS%

azsphere device sideload stop -i dfc29b2a-1ac1-4328-a734-00770b9789b9
azsphere device sideload start -d -i dfc29b2a-1ac1-4328-a734-00770b9789b9

start telnet 192.168.35.2 2342

echo try this:
echo first connect:
echo target remote 192.168.35.2:2345
echo.
echo run the application:
echo c
echo.

"%SYSROOT%\tools\gcc\arm-poky-linux-musleabi-gdb.exe" target\arm-v7-none-eabi\debug\advanced_sphere