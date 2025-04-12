use bevy::prelude::*;

use crate::entities::spawn_wall::RemovableWall;



pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit::Success); 
    }
}

pub fn remove_gap(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut RemovableWall)>,
){  

    if keys.just_pressed(KeyCode::Space){
        for (mut transform, mut open) in query.iter_mut(){
            match open.0{
                true => {
                    transform.translation = Vec3::new(0.,0.,0.);
                    open.0 = false;
                }
                false => {
                    transform.translation = Vec3::new(0.,10000.,0.);
                    open.0 = true;
                }
            }
            
        }
    }

}