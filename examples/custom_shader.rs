use bevy::prelude::*;

fn main() {
    App::build().add_defaults().setup(setup).run();
}

#[derive(Uniforms, Default)]
struct MyMaterial {
    pub color: Color,
    #[uniform(ignore, shader_def)]
    pub always_red: bool,
}

fn setup(world: &mut World, resources: &mut Resources) {
    let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
    let mut pipelines = resources.get_mut::<AssetStorage<PipelineDescriptor>>().unwrap();
    let mut shaders = resources.get_mut::<AssetStorage<Shader>>().unwrap();
    render_graph
        .build(&mut pipelines, &mut shaders)
        .add_resource_provider(UniformResourceProvider::<MyMaterial>::new(true))
        .add_pipeline_to_pass(resource_name::pass::MAIN, "MyMaterial", |builder| {
            builder
                .with_vertex_shader(Shader::from_glsl(
                    ShaderStage::Vertex,
                    r#"
                    #version 450
                    layout(location = 0) in vec4 Vertex_Position;
                    layout(location = 0) out vec4 v_Position;
                    layout(set = 0, binding = 0) uniform Camera {
                        mat4 ViewProj;
                    };
                    layout(set = 1, binding = 0) uniform Object {
                        mat4 Model;
                    };
                    void main() {
                        v_Position = Model * Vertex_Position;
                        gl_Position = ViewProj * v_Position;
                    }
                "#,
                ))
                .with_fragment_shader(Shader::from_glsl(
                    ShaderStage::Fragment,
                    r#"
                    #version 450
                    layout(location = 0) in vec4 v_Position;
                    layout(location = 0) out vec4 o_Target;
                    layout(set = 1, binding = 1) uniform MyMaterial_color {
                        vec4 color;
                    };
                    void main() {
                        o_Target = color;

                    # ifdef MYMATERIAL_ALWAYS_RED
                        o_Target = vec4(0.8, 0.0, 0.0, 1.0);
                    # endif
                    }
                "#,
                ))
                .with_default_config();
        });

    let mut mesh_storage = resources.get_mut::<AssetStorage<Mesh>>().unwrap();
    let cube_handle = mesh_storage.add(Mesh::load(MeshType::Cube));

    let mut pipeline_storage = resources
        .get_mut::<AssetStorage<PipelineDescriptor>>()
        .unwrap();
    let material_handle = pipeline_storage.get_named("MyMaterial").unwrap();

    world
        .build()
        // cube
        .add_entity(MeshMaterialEntity::<MyMaterial> {
            mesh: cube_handle,
            renderable: Renderable {
                pipelines: vec![material_handle],
                ..Default::default()
            },
            material: MyMaterial {
                color: Color::rgb(0.0, 0.8, 0.0),
                always_red: false,
            },
            translation: Translation::new(-2.0, 0.0, 0.0),
            ..Default::default()
        })
        // cube
        .add_entity(MeshMaterialEntity::<MyMaterial> {
            mesh: cube_handle,
            renderable: Renderable {
                pipelines: vec![material_handle],
                ..Default::default()
            },
            material: MyMaterial {
                color: Color::rgb(0.0, 0.0, 0.0),
                always_red: true,
            },
            translation: Translation::new(2.0, 0.0, 0.0),
            ..Default::default()
        })
        // camera
        .add_entity(CameraEntity {
            local_to_world: LocalToWorld(Mat4::look_at_rh(
                Vec3::new(3.0, 8.0, 5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            ..Default::default()
        })
        .build();
}
