use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::components::{Tower, LaserTower, RangeIndicator};
use crate::resources::{GameStatus, SelectedTower};

pub fn update_range_indicator(
    mut commands: Commands,
    selected_tower: Res<SelectedTower>,
    towers_query: Query<(&Transform, Option<&Tower>, Option<&LaserTower>)>,
    indicator_query: Query<Entity, With<RangeIndicator>>,
    game_status: Res<GameStatus>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if *game_status != GameStatus::Playing {
        for entity in indicator_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    let tower_entity = selected_tower.0;
    let mut range_radius = 0.0;
    let mut tower_position = Vec3::ZERO;

    if let Some(entity) = tower_entity {
        if let Ok((transform, maybe_tower, maybe_laser)) = towers_query.get(entity) {
            tower_position = transform.translation;
            if let Some(tower) = maybe_tower {
                range_radius = tower.range;
            } else if let Some(laser) = maybe_laser {
                range_radius = laser.range;
            }
        } else {
            commands.insert_resource(SelectedTower(None));
        }
    }

    let indicator_entity = indicator_query.iter().next();

    if range_radius > 0.0 {
        // 创建一个圆形网格
        let circle_mesh = Mesh::from(shape::Circle::new(1.0));
        let mesh_handle = meshes.add(circle_mesh);
        let material_handle = materials.add(ColorMaterial::from(Color::rgba(0.0, 0.8, 0.0, 0.3)));

        if let Some(entity) = indicator_entity {
            commands.entity(entity).insert(Transform {
                translation: tower_position + Vec3::new(0.0, 0.0, 0.5),
                scale: Vec3::splat(range_radius),
                ..Default::default()
            });
        } else {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(mesh_handle),
                    material: material_handle,
                    transform: Transform {
                        translation: tower_position + Vec3::new(0.0, 0.0, 0.5),
                        scale: Vec3::splat(range_radius),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                RangeIndicator,
            ));
        }
    } else {
        if let Some(entity) = indicator_entity {
            commands.entity(entity).despawn();
        }
    }
}