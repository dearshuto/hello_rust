use kiss3d::{
    camera::ArcBall,
    light::Light,
    nalgebra::{Point3, Translation3},
    window::Window,
};
use rapier3d::prelude::*;

fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
    collider_set.insert(collider);

    /* Create the bounding ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 7.0, 0.0])
        .build();
    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    let rigid_body = RigidBodyBuilder::fixed().build();
    let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5)
        .restitution(1.0)
        .build();
    let cuboid_body_handlw = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, cuboid_body_handlw, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut window = Window::new("hello_rapier");
    let mut sphere = window.add_sphere(0.5);
    sphere.set_color(1.0, 0.0, 0.0);

    let _floor = window.add_cube(5.0, 1.0, 5.0);

    window.set_light(Light::StickToCamera);

    let eye = Point3::new(10.0f32, 5.0, 10.0);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);
    while window.render_with_camera(&mut camera) {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        let ball_body = &rigid_body_set[ball_body_handle];
        let t = ball_body.translation();

        let translation = Translation3::new(t.x, t.y, t.z);
        sphere.set_local_translation(translation);
    }
}
