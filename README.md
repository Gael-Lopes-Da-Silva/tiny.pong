<h3 align="center">Pong</h3>

---

<p align="center">⚙️ This is a pong create in C with Raylib.</p>

---

### 🖼️ Screenshot

![](./Screenshots/Screenshot1.png)

### ❓ How to use the build script

Before building, make sure you have a C compiler installed on your computer.

~~~shell
# windows
$ cc main.c -o Pong.exe -Wall -Wextra -pedantic -fno-common -fno-builtin -mwindows -lraylib -lopengl32 -lgdi32 -lwinmm

# linux
$ cc main.c -o Pong.exe -Wall -Wextra -pedantic -fno-common -fno-builtin -mwindows -lraylib -lGL -lm -lpthread -ldl -lrt

# macos
$ cc main.c -o Pong.exe -Wall -Wextra -pedantic -fno-common -fno-builtin -mwindows -lraylib -lGL -lm -lpthread -ldl -lrt -lX11
~~~

If you want a more customizable build method see <a href="https://github.com/raysan5/raylib/blob/master/examples/Makefile">here</a>.
If you want to build on android see <a href="https://github.com/raysan5/raylib/blob/master/examples/Makefile.Android">here</a> and <a href="https://github.com/raysan5/raylib/blob/master/examples/Makefile.Web">here</a> if you want to build for the web.
