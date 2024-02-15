// @author: Gael Lopes Da Silva
// @project: Pong
// @github: https://github.com/Gael-Lopes-Da-Silva/Pong

#pragma once

#ifndef RAYLIB_LOADED
#include "../include/raylib.h"
#include "../include/raymath.h"
#include "../include/reasings.h"
#endif

#define SCREEN_WIDTH 1360
#define SCREEN_HEIGHT 760

typedef struct Player
{
	int position;
	int point;
	float speed;
	bool collision;
	Rectangle texture;
	Color color;
} Player;

typedef struct Enemy
{
	int position;
	int point;
	float speed;
	bool collision;
	Rectangle texture;
	Color color;
} Enemy;

typedef struct Ball
{
	float size;
	float speed;
	Vector2 position;
	Vector2 velocity;
	Color color;
} Ball;

void player_Draw(Player player);
void player_UpdatePosition(Player *player);

void enemy_Draw(Enemy enemy);
void enemy_UpdatePosition(Enemy *enemy, Ball ball);

void ball_Draw(Ball ball);
void ball_UpdatePosition(Ball *ball);
void ball_CheckCollision(Ball *ball, Player *player, Enemy *enemy);
