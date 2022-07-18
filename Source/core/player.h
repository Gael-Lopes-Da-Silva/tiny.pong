#pragma once

#ifndef RAYLIB_LOADED
    #include "raylib/raylib.h"
    #include "raylib/raymath.h"
    #include "raylib/easings.h"
#endif

#ifndef CORE_LOADED
    #include "core.h"
#endif

void player_Draw(Player player)
{
    DrawRectangleRec(player.texture, player.color);
}

void player_UpdatePosition(Player *player)
{
    if (player -> point == 0) return;
    
    if (IsKeyDown(KEY_W)) player -> position -= player -> speed;
    if (IsKeyDown(KEY_S)) player -> position += player -> speed;

    player -> position = Clamp(player -> position, 10, SCREEN_HEIGHT-10-(player -> texture.height));
    player -> texture.y = player -> position;
}
