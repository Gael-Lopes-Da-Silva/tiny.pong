@echo off
set INPUT= main.c
set OUTPUT= main.exe
set FLAGS= -o%OUTPUT% -Wall -Wextra -pedantic -fno-common -fno-builtin
set LIBS= -lraylib -lopengl32 -lgdi32 -lwinmm

clang %INPUT% %FLAGS% %LIBS%

if ERRORLEVEL == 0 %OUTPUT%
