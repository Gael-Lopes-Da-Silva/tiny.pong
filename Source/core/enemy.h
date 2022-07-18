#pragma once

#ifndef RAYLIB_LOADED
    #include "raylib/raylib.h"
    #include "raylib/raymath.h"
    #include "raylib/easings.h"
#endif

#ifndef CORE_LOADED
    #include "core.h"
#endif

void enemy_Draw(Enemy enemy)
{
    DrawRectangleRec(enemy.texture, enemy.color);
}

void enemy_UpdatePosition(Enemy *enemy, Ball ball)
{
    if (enemy -> point == 0) return;
    
    if (ball.position.x > SCREEN_WIDTH/2)
    {
        if (ball.position.y-25 < enemy -> position)
        {
            enemy -> position -= enemy -> speed;
        }
        else if (ball.position.y+25 > (enemy -> position)+(enemy -> texture.height))
        {
            enemy -> position += enemy -> speed;
        }
        
        enemy -> position = Clamp(enemy -> position, 10, SCREEN_HEIGHT-10-(enemy -> texture.height));
        enemy -> texture.y = enemy -> position;
    }
}
