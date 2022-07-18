#pragma once

#ifndef RAYLIB_LOADED
    #include "raylib/raylib.h"
    #include "raylib/raymath.h"
    #include "raylib/easings.h"
#endif

#ifndef CORE_LOADED
    #include "core.h"
#endif

void ball_Draw(Ball ball)
{
    DrawCircle(ball.position.x, ball.position.y, 25, ball.color);
}

void ball_UpdatePosition(Ball *ball)
{
    ball -> position.x += ball -> velocity.x;
    ball -> position.y += ball -> velocity.y;
}

void ball_CheckCollision(Ball *ball, Player *player, Enemy *enemy)
{

    if (ball -> position.x <= 0+25)
    {
        ball -> velocity.x = -(ball -> velocity.x);
        if (player -> point != 0) player -> point -= 1;
    }
    if (ball -> position.x >= SCREEN_WIDTH-25) 
    {
        ball -> velocity.x = -(ball -> velocity.x);
        if (enemy -> point != 0) enemy -> point -= 1;
    }

    if (ball -> position.y <= 0+25) ball -> velocity.y = -(ball -> velocity.y);
    if (ball -> position.y >= SCREEN_HEIGHT-25) ball -> velocity.y = -(ball -> velocity.y);

    if (CheckCollisionCircleRec(ball -> position, 25, player -> texture))
    {
        ball -> velocity.x += 0.7f;
        ball -> velocity.y += 0.7f;
        ball -> velocity.x = -(ball -> velocity.x);
    }
    if (CheckCollisionCircleRec(ball -> position, 25, enemy -> texture))
    {
        ball -> velocity.x += 0.7f;
        ball -> velocity.y += 0.7f;
        ball -> velocity.x = -(ball -> velocity.x);
    }
}
