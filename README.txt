                             Pong
          https://github.com/Gael-Lopes-Da-Silva/Pong

Description
------------------------------------------------------------------
This is a pong game created in C with Raylib.


Screenshots
------------------------------------------------------------------
./screenshots/screenshot1.png


Installation
------------------------------------------------------------------
Before building, make sure you have a C compiler installed on
your computer.

Then run this commands:
git clone https://github.com/Gael-Lopes-Da-Silva/Pong
cd Pong

# windows
cc pong.c -o pong.exe -Wall -Wextra -pedantic -fno-common
  -fno-builtin -mwindows -Iinclude/ -Llib/ -lraylib -lopengl32
  -lgdi32 -lwinmm

# linux
cc pong.c -o pong.exe -Wall -Wextra -pedantic -fno-common
  -fno-builtin -mwindows -Iinclude/ -Llib/ -lraylib -lGL -lm
  -lpthread -ldl -lrt

# macos
cc pong.c -o pong.exe -Wall -Wextra -pedantic -fno-common
  -fno-builtin -mwindows -Iinclude/ -Llib/ -lraylib -lGL -lm
  -lpthread -ldl -lrt -lX11


Note
------------------------------------------------------------------
If you want a more customizable build method see here:
https://github.com/raysan5/raylib/blob/master/examples/Makefile
If you want to build on android see here:
https://github.com/raysan5/raylib/blob/master/examples/Makefile.Android
and https://github.com/raysan5/raylib/blob/master/examples/Makefile.Web
if you want to build for the web.
