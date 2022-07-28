COMPILER=gcc
INPUT=main.c
OUTPUT=pong.exe
FLAGS=-o${OUTPUT}\ -Wall\ -Wextra\ -pedantic\ -fno-common\ -fno-builtin\ -mwindows
LIBS=-lraylib\ -lopengl32\ -lgdi32\ -lwinmm

if [ $# = 0 ] ; then
    ${COMPILER} ${INPUT} ${FLAGS} ${LIBS}

    if [ $? ] ; then
        ./${OUTPUT}
    fi
else
    if [ $1 = "help" -o $1 = "--help" -o $1 = "-h" ] ; then
        echo "help  --help  -h : Print help"
        echo "build --build -b : Build the application"
    fi

    if [ $1 = "build" -o $1 = "--build" -o $1 = "-b" ] ; then
        ${COMPILER} ${INPUT} ${FLAGS} ${LIBS}
    fi
fi
