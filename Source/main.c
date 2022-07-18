#ifndef RAYLIB_LOADED
    #define RAYLIB_LOADED
    #include "raylib/raylib.h"
    #include "raylib/raymath.h"
    #include "raylib/easings.h"
#endif

#ifndef CORE_LOADED
    #define CORE_LOADED
    #include "core/core.h"
    #include "core/player.h"
    #include "core/enemy.h"
    #include "core/ball.h"
#endif

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

    enemy.color = BLACK;
    enemy.texture = (Rectangle){SCREEN_WIDTH-40-10, 10, 40, 150};
    enemy.speed = 5;
    enemy.position = 0;
    enemy.point = 3;

    ball.color = BLACK;
    ball.speed = 5;
    ball.position = (Vector2){500, 300};
    ball.velocity = (Vector2){5, 5};
    
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
            DrawText(TextFormat("%d", enemy.point), SCREEN_WIDTH-350-60, 5, 60, RED);

            if (player.point == 0)
            {
                DrawText("You lost", GetScreenWidth()/2-60, GetScreenHeight()/2, 60, BLACK);
            }
            if (enemy.point == 0)
            {
                DrawText("You win", GetScreenWidth()/2-60, GetScreenHeight()/2, 60, BLACK);
            }

            player_Draw(player);
            enemy_Draw(enemy);
            ball_Draw(ball);
        EndDrawing();
    }
    
    CloseWindow();
    return 0;
}
