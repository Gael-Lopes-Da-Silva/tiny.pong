#pragma once

#define SCREEN_WIDTH 1360
#define SCREEN_HEIGHT 760

typedef struct Player
{
    int position;
    int speed;
    int point;
    Rectangle texture;
    Color color;
} Player;

typedef struct Enemy
{
    int position;
    int speed;
    int point;
    Rectangle texture;
    Color color;
} Enemy;

typedef struct Ball
{
    int speed;
    Vector2 position;
    Vector2 velocity;
    Color color;
} Ball;
