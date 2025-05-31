use crate::*;
use crate::menu::GameState;
use crate::car::Car;

use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    render::{
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
        RenderApp,
    },
};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<RacingPostProcessSettings>::default(),
            UniformComponentPlugin::<RacingPostProcessSettings>::default(),
        ))
        .add_systems(OnEnter(GameState::InGame), setup_post_processing)
        .add_systems(Update, update_post_process_settings.run_if(in_state(GameState::InGame)));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
                Core3d,
                PostProcessLabel,
            )
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    PostProcessLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PostProcessPipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static RacingPostProcessSettings,
        &'static DynamicUniformIndex<RacingPostProcessSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<RacingPostProcessSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "racing_post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("racing_post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "racing_post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<RacingPostProcessSettings>(true),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.load_asset("shaders/racing_post_process.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("racing_post_process_pipeline".into()),
                layout: vec![layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
                zero_initialize_workgroup_memory: false,
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct RacingPostProcessSettings {
    pub speed_intensity: f32,       // Speed-based effects intensity
    pub chromatic_aberration: f32,  // Color distortion at edges
    pub vignette_strength: f32,     // Dark edge vignette
    pub speed_lines: f32,           // Radial blur from center
    pub color_saturation: f32,      // Enhanced colors
    pub contrast: f32,              // Enhanced contrast
}

fn setup_post_processing(
    mut commands: Commands, 
    camera_query: Query<Entity, (With<Camera3d>, Without<RacingPostProcessSettings>)>
) {
    // Only add post-processing to 3D cameras (game cameras, not menu 2D cameras)
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).insert(RacingPostProcessSettings {
            speed_intensity: 0.0,
            chromatic_aberration: 0.004,
            vignette_strength: 0.4,
            speed_lines: 0.0,
            color_saturation: 1.3,
            contrast: 1.2,
        });
    }
}

fn update_post_process_settings(
    car_query: Query<&Car>,
    mut camera_query: Query<&mut RacingPostProcessSettings>,
    settings: Res<crate::menu::GameSettings>,
) {
    if let Ok(car) = car_query.single() {
        for mut post_settings in camera_query.iter_mut() {
            if settings.post_processing_enabled {
                // Calculate speed factor (0.0 to 1.0)
                let speed_factor = (car.speed.abs() / car.max_speed).clamp(0.0, 1.0);
                
                // Speed-based effects - increased intensity
                post_settings.speed_intensity = speed_factor;
                post_settings.chromatic_aberration = 0.004 + speed_factor * 0.012;
                post_settings.speed_lines = speed_factor * 0.7;
                post_settings.vignette_strength = 0.4 + speed_factor * 0.5;
                
                // Enhanced visuals for racing - more dramatic
                post_settings.color_saturation = 1.3 + speed_factor * 0.4;
                post_settings.contrast = 1.2 + speed_factor * 0.3;
            } else {
                // Disable all effects when post-processing is off
                post_settings.speed_intensity = 0.0;
                post_settings.chromatic_aberration = 0.0;
                post_settings.speed_lines = 0.0;
                post_settings.vignette_strength = 0.0;
                post_settings.color_saturation = 1.0; // Normal saturation
                post_settings.contrast = 1.0; // Normal contrast
            }
        }
    }
} 