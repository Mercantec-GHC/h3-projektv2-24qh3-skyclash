use engine::{query, rigid_body::RigidBody, spawn, Collider, Component, System};

use crate::{
    hurtbox::{HurtDirection, Hurtbox},
    keyset::Keyset,
    sprite_renderer::Sprite,
};

#[derive(Component, Clone)]
pub struct PlayerAttack {
    pub keyset: Keyset,
    pub cooldown: f64,
}

impl PlayerAttack {
    pub fn new(keyset: Keyset, cooldown: f64) -> Self {
        Self { keyset, cooldown }
    }
}

struct SpawnAttackArgs {
    id: Option<engine::Id>,
    direction: HurtDirection,
    pos: (f64, f64),
    player_size: (f64, f64),
    attack_size: (f64, f64),
    vel: (f64, f64),
    textures: Vec<String>,
}

fn spawn_attack(
    ctx: &mut engine::Context,
    SpawnAttackArgs {
        id,
        direction,
        pos,
        player_size,
        attack_size,
        vel,
        textures,
    }: SpawnAttackArgs,
) {
    let textures = textures
        .into_iter()
        .map(|path| ctx.load_texture(path).unwrap())
        .collect::<Vec<_>>();
    spawn!(
        ctx,
        Sprite::new(textures[0]),
        RigidBody::new()
            .with_pos(match direction {
                HurtDirection::Up => (pos.0, pos.1 - attack_size.1),
                HurtDirection::Down => (pos.0, pos.1 + player_size.1),
                HurtDirection::Left => (pos.0 - attack_size.0, pos.1),
                HurtDirection::Right => (pos.0 + player_size.0, pos.1),
            })
            .with_vel(vel)
            .with_rect(attack_size),
        Hurtbox {
            direction,
            power: 20.0,
            owner: id,
            duration: 0.3,
            stun_time: Some(0.5),
            textures,
            ..Default::default()
        }
    );
}

pub struct PlayerAttackSystem(pub u64);
impl System for PlayerAttackSystem {
    fn on_update(&self, ctx: &mut engine::Context, delta: f64) -> Result<(), engine::Error> {
        for id in query!(ctx, RigidBody, Collider, PlayerAttack) {
            let player_attack = ctx.select::<PlayerAttack>(id).clone();
            let keyset = player_attack.keyset;
            let right_pressed = ctx.key_pressed(keyset.right());
            let left_pressed = ctx.key_pressed(keyset.left());
            let down_pressed = ctx.key_pressed(keyset.down());
            let light_attack_pressed = ctx.key_just_pressed(keyset.light_attack());
            let body = ctx.select::<RigidBody>(id).clone();
            if player_attack.cooldown >= 0.0 {
                let player_attack = ctx.select::<PlayerAttack>(id);
                player_attack.cooldown -= delta;
                continue;
            }
            if !light_attack_pressed {
                continue;
            }
            if down_pressed {
                spawn_attack(
                    ctx,
                    SpawnAttackArgs {
                        id: Some(id),
                        direction: HurtDirection::Down,
                        pos: body.pos,
                        player_size: body.rect,
                        attack_size: (128.0, 128.0),
                        vel: (0.0, 0.0),
                        textures: vec!["textures/nuh-uh.png".to_string()],
                    },
                );
            } else if left_pressed && !right_pressed {
                spawn_attack(
                    ctx,
                    SpawnAttackArgs {
                        id: Some(id),
                        direction: HurtDirection::Left,
                        pos: body.pos,
                        player_size: body.rect,
                        attack_size: (64.0, 128.0),
                        vel: (body.vel.0 / 2.0, body.vel.1 / 2.0),
                        textures: vec![
                            "textures/attacks/left_0.png".to_string(),
                            "textures/attacks/left_1.png".to_string(),
                            "textures/attacks/left_2.png".to_string(),
                            "textures/attacks/left_3.png".to_string(),
                            "textures/attacks/left_4.png".to_string(),
                        ],
                    },
                );
            } else if right_pressed && !left_pressed {
                spawn_attack(
                    ctx,
                    SpawnAttackArgs {
                        id: Some(id),
                        direction: HurtDirection::Right,
                        pos: body.pos,
                        player_size: body.rect,
                        attack_size: (64.0, 128.0),
                        vel: (body.vel.0 / 2.0, body.vel.1 / 2.0),
                        textures: vec![
                            "textures/attacks/right_0.png".to_string(),
                            "textures/attacks/right_1.png".to_string(),
                            "textures/attacks/right_2.png".to_string(),
                            "textures/attacks/right_3.png".to_string(),
                            "textures/attacks/right_4.png".to_string(),
                        ],
                    },
                );
            } else {
                spawn_attack(
                    ctx,
                    SpawnAttackArgs {
                        id: Some(id),
                        direction: HurtDirection::Up,
                        pos: body.pos,
                        player_size: body.rect,
                        attack_size: (128.0, 64.0),
                        vel: (0.0, 0.0),
                        textures: vec![
                            "textures/attacks/up_0.png".to_string(),
                            "textures/attacks/up_1.png".to_string(),
                            "textures/attacks/up_2.png".to_string(),
                            "textures/attacks/up_3.png".to_string(),
                            "textures/attacks/up_4.png".to_string(),
                        ],
                    },
                );
            }
            let player_attack = ctx.select::<PlayerAttack>(id);
            player_attack.cooldown = 0.5;
        }

        Ok(())
    }
}
