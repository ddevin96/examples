mod model;
use crate::model::state::Environment;
use cfg_if::cfg_if;
use {
    krabmaga::engine::schedule::Schedule, krabmaga::engine::state::State, krabmaga::*,
    std::time::Duration,
};

cfg_if! {
    if #[cfg(any(feature = "parallel", feature = "visualization", feature = "visualization_wasm"))]{
        mod visualization;
        use crate::visualization::environment_vis::EnvironmentVis;
        use crate::visualization::eater_vis::EaterVis;
        use {
            krabmaga::bevy::prelude::Color,
            krabmaga::bevy::prelude::IntoSystem,
            krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D,
            krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
            krabmaga::visualization::fields::number_grid_2d::BatchRender,
            krabmaga::visualization::visualization::Visualization,
        };
    }
}

pub const MAX_SUGAR: u32 = 3;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 50;

    let dim = (64, 64);
    let num_agents = 64;

    let state = Environment::new(dim, num_agents);
    let _ = simulate!(state, step, 1, false);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let step = 10;

    let dim = (75, 75);
    let num_agents = 150;

    let state = Environment::new(dim, num_agents);
    let mut app = Visualization::default()
        .with_window_dimensions(1000.0, 600.0)
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_background_color(Color::WHITE)
        .with_name("Sugarscape")
        .setup::<EnvironmentVis, Environment>(EnvironmentVis, state);
    app.add_system(DenseNumberGrid2D::batch_render.system());
    app.run()
}
