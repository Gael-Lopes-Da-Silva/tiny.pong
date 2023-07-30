/* 
@author: Gael Lopes Da Silva
@project: Pong
@github: https://github.com/Gael-Lopes-Da-Silva/Pong
@gitlab: https://gitlab.com/Gael-Lopes-Da-Silva/Pong
*/

#ifndef RAYLIB_LOADED
    #define RAYLIB_LOADED
    #include "../include/raylib.h"
    #include "../include/raymath.h"
    #include "../include/reasings.h"
#endif

#include "pong.h"

void player_Draw(Player player)
{
    DrawRectangleRec(player.texture, player.color);
}

void player_UpdatePosition(Player *player)
{
    if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) player->position -= player->speed;
    if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) player->position += player->speed;

    player->position = Clamp(player->position, 10, SCREEN_HEIGHT - 10 - (player->texture.height));
    player->texture.y = player->position;
}

void enemy_Draw(Enemy enemy)
{
    DrawRectangleRec(enemy.texture, enemy.color);
}

void enemy_UpdatePosition(Enemy *enemy, Ball ball)
{
    if (ball.position.x > SCREEN_WIDTH / 2)
    {
        if (ball.position.y - ball.size < enemy->position)
        {
            enemy->position -= enemy->speed;
        }
        else if (ball.position.y + ball.size > (enemy->position) + (enemy->texture.height))
        {
            enemy->position += enemy->speed;
        }

        enemy->position = Clamp(enemy->position, 10, SCREEN_HEIGHT - 10 - (enemy->texture.height));
        enemy->texture.y = enemy->position;
    }
}

void ball_Draw(Ball ball)
{
    DrawCircle(ball.position.x, ball.position.y, ball.size, ball.color);
}

void ball_UpdatePosition(Ball *ball)
{
    ball->position.x += ball->velocity.x * ball->speed;
    ball->position.y += ball->velocity.y * ball->speed;
}

void ball_CheckCollision(Ball *ball, Player *player, Enemy *enemy)
{

    if (ball->position.x <= 0 + ball->size)
    {
        ball->position = (Vector2){SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2};
        ball->velocity.x = -(ball->velocity.x);
        if (player->point != 0) player->point -= 1;
        enemy->collision = true;
    }
    if (ball->position.x >= SCREEN_WIDTH - ball->size)
    {
        ball->position = (Vector2){SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2};
        ball->velocity.x = -(ball->velocity.x);
        if (enemy->point != 0) enemy->point -= 1;
        player->collision = true;
    }

    if (ball->position.y <= 0 + ball->size) ball->velocity.y = -(ball->velocity.y);
    if (ball->position.y >= SCREEN_HEIGHT - ball->size) ball->velocity.y = -(ball->velocity.y);

    if (CheckCollisionCircleRec(ball->position, ball->size, player->texture) && player->collision)
    {
        ball->speed += 0.2f;
        ball->speed += 0.2f;
        ball->velocity.x = -(ball->velocity.x);
        player->collision = false;
        enemy->collision = true;
    }
    if (CheckCollisionCircleRec(ball->position, ball->size, enemy->texture) && enemy->collision)
    {
        ball->speed += 0.2f;
        ball->speed += 0.2f;
        ball->velocity.x = -(ball->velocity.x);
        enemy->collision = false;
        player->collision = true;
    }
}

int main(void)
{
    Player player = {0};
    Enemy enemy = {0};
    Ball ball = {0};

    player.color = BLACK;
    player.texture = (Rectangle){10, 10, 40, 150};
    player.speed = 5;
    player.position = 0;
    player.point = 3;
    player.collision = true;

    enemy.color = BLACK;
    enemy.texture = (Rectangle){SCREEN_WIDTH - 40 - 10, 10, 40, 150};
    enemy.speed = 5;
    enemy.position = 0;
    enemy.point = 3;
    enemy.collision = true;

    ball.color = BLACK;
    ball.speed = 5;
    ball.size = 25;
    ball.position = (Vector2){SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2};
    ball.velocity = (Vector2){1, 1};

    InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "pong");
    SetTargetFPS(60);
    SetExitKey(0);

    while (!WindowShouldClose())
    {
        player_UpdatePosition(&player);
        enemy_UpdatePosition(&enemy, ball);
        ball_UpdatePosition(&ball);
        ball_CheckCollision(&ball, &player, &enemy);

        if (player.point == 0)
        {
            ball.color = RED;
            ball.velocity = (Vector2){0, 0};
        }
        if (enemy.point == 0)
        {
            ball.color = RED;
            ball.velocity = (Vector2){0, 0};
        }

        BeginDrawing();
            ClearBackground(WHITE);

            DrawText(TextFormat("%d", player.point), 350, 5, 60, RED);
            DrawText(TextFormat("%d", enemy.point), SCREEN_WIDTH - 350 - 60, 5, 60, RED);

            player_Draw(player);
            enemy_Draw(enemy);
            ball_Draw(ball);

            if (player.point == 0)
            {
                DrawText("You lost", SCREEN_WIDTH / 2 - 120, SCREEN_HEIGHT / 2 - 30, 60, BLACK);
            }
            if (enemy.point == 0)
            {
                DrawText("You win", SCREEN_WIDTH / 2 - 120, SCREEN_HEIGHT / 2 - 30, 60, BLACK);
            }
        EndDrawing();
    }

    CloseWindow();
    return 0;
}
