use avian3d::prelude::*;
use bevy::prelude::*;

use crate::character::components::ActiveCharacter;
use super::third_person::OrbitCamera;

/// Pull camera forward if it collides with terrain.
pub fn camera_collision(
    spatial_query: SpatialQuery,
    target_query: Query<&Transform, (With<ActiveCharacter>, Without<OrbitCamera>)>,
    mut camera_query: Query<(&mut Transform, &OrbitCamera)>,
) {
    let Ok(target_tf) = target_query.single() else {
        return;
    };
    let Ok((mut cam_tf, orbit)) = camera_query.single_mut() else {
        return;
    };

    let target = target_tf.translation + orbit.target_offset;
    let to_camera = cam_tf.translation - target;
    let direction = Dir3::new(to_camera).ok();

    let Some(direction) = direction else {
        return;
    };

    let max_dist = to_camera.length();

    // Raycast from target toward desired camera position
    if let Some(hit) = spatial_query.cast_ray(
        target,
        direction,
        max_dist,
        true,
        &SpatialQueryFilter::default(),
    ) {
        // Pull camera in front of the hit point
        let safe_distance = (hit.distance - 0.3).max(orbit.min_distance);
        cam_tf.translation = target + direction.as_vec3() * safe_distance;
        cam_tf.look_at(target, Vec3::Y);
    }
}
