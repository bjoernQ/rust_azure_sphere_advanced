@echo off
setlocal
set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set PATH=%PATH%;%AZSPHERETOOLS%

azsphere device sideload delete
azsphere device sideload deploy -p target\manual.imagepackage