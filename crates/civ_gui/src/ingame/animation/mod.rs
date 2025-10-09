use bevy::prelude::*;

use std::time::Duration;

#[derive(Debug, Component)]
pub struct SpriteSheetAnimation {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl SpriteSheetAnimation {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }
}

#[derive(Debug, Component)]
pub struct FadeAnimation {
    timer: Timer,
    direction: f32, // 1.0 = fade in, -1.0 = fade out
}

impl Default for FadeAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            direction: 1.0,
        }
    }
}

pub fn fade_animations(time: Res<Time>, mut query: Query<(&mut Sprite, &mut FadeAnimation)>) {
    for (mut sprite, mut fade) in &mut query {
        fade.timer.tick(time.delta());

        // Update alpha value
        let current_alpha = sprite.color.alpha();
        let elapsed = time.delta().as_millis() as f32;
        let new_alpha = (current_alpha + (elapsed / 100.0 * fade.direction)).clamp(0.0, 1.0);
        sprite.color.set_alpha(new_alpha);

        // Flip direction when timer finishes
        if fade.timer.is_finished() {
            fade.direction *= -1.0;
        }
    }
}

pub fn sprite_sheet_animations(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut SpriteSheetAnimation)>,
) {
    for (mut sprite, mut animation) in &mut query {
        animation.frame_timer.tick(time.delta());

        if animation.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == animation.last_sprite_index {
                    atlas.index = animation.first_sprite_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    animation.frame_timer = SpriteSheetAnimation::timer_from_fps(animation.fps);
                }
            }
        }
    }
}
