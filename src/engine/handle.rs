use std::rc::Rc;

use rapier2d::prelude::{ColliderHandle, CollisionEvent, ContactForceEvent};
use winit::event::WindowEvent;

use super::{Body, BodyBuilder, Engine, Joint};

/// Scene
pub struct SceneHandle<'a> {
    pub(crate) engine: &'a mut Engine,
    pub(crate) scene_id: u64,
}

impl<'a> SceneHandle<'a> {
    /// Scene id
    pub fn scene_id(&self) -> u64 {
        return self.scene_id;
    }

    //// Add a body into this scene
    pub fn add_body(&mut self, mut body: BodyBuilder) -> u64 {
        let body_id = self.engine.unique_id;
        self.engine.unique_id += 1;
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        body.rigid.user_data = body_id as u128;
        let body_handle = scene.physics_engine.rigid_body_set.insert(body.rigid);
        self.engine.body_mp.insert(
            body_id,
            Body {
                class: body.class,
                name: body.name,
                look: body.look,
                rigid: body_handle,
                life_step_op: body.life_step_op,
            },
        );
        for collider in body.collider.collider_v {
            scene.physics_engine.collider_set.insert_with_parent(
                collider,
                body_handle,
                &mut scene.physics_engine.rigid_body_set,
            );
        }
        body_id
    }

    pub fn add_joint(&mut self, mut joint: Joint) -> u64 {
        let joint_id = self.engine.unique_id;
        self.engine.unique_id += 1;
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        let body1 = &self.engine.body_mp[&joint.body1];
        let body2 = &self.engine.body_mp[&joint.body2];
        joint.joint.user_data = joint_id as u128;
        scene
            .physics_engine
            .impulse_joint_set
            .insert(body1.rigid, body2.rigid, joint.joint, true);
        joint_id
    }

    pub fn set_event_listener(&mut self, listener: Rc<dyn Fn(SceneHandle, WindowEvent)>) {
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        scene.on_event = Some(listener);
    }

    pub fn set_step_listener(&mut self, listener: Rc<dyn Fn(SceneHandle, u128)>) {
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        scene.on_step = Some(listener);
    }

    pub fn bind_watcher(&mut self, body_id: u64) {
        self.engine.watcher_binding_body_id = body_id
    }

    pub fn set_collision_event_handler(
        &mut self,
        event_handler: Rc<dyn Fn(SceneHandle, CollisionEvent)>,
    ) {
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        scene.on_collision_event = Some(event_handler);
    }

    pub fn set_force_event_handler(
        &mut self,
        event_handler: Rc<dyn Fn(SceneHandle, ContactForceEvent)>,
    ) {
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        scene.on_force_event = Some(event_handler);
    }

    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }

    pub fn get_engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }

    pub fn get_body_id_of_collider(&mut self, ch: ColliderHandle) -> u64 {
        let scene = self.engine.scene_mp.get_mut(&self.scene_id).unwrap();
        let rigid_body = &scene.physics_engine.rigid_body_set
            [scene.physics_engine.collider_set[ch].parent().unwrap()];
        rigid_body.user_data as u64
    }

    pub fn get_body_mut(&mut self, id: &u64) -> Option<&mut Body> {
        self.engine.body_mp.get_mut(id)
    }

    pub fn get_body(&mut self, id: &u64) -> Option<&Body> {
        self.engine.body_mp.get(id)
    }
}
