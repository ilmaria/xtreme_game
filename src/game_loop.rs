// use gfx::{Device, Factory, RenderTarget, DepthStencil};
// use gfx::format::{RenderFormat, DepthFormat};
use gfx;
use gfx::texture;
use gfx_device_gl as gfx_gl;

use math::Vec3;
use cgmath;
use cgmath::{Deg, Matrix4, Point3, Vector3};

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
        Vertex {
            pos: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
            tex_coord: [t[0] as f32, t[1] as f32],
        }
    }
}

#[no_mangle]
pub fn render_and_update_gl(
    encoder: &mut gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>,
    factory: &mut gfx_gl::Factory,
) {
    let pso = factory
        .create_pipeline_simple(
            include_bytes!("../data/shaders/cube.vert"),
            include_bytes!("../data/shaders/cube.frag"),
            pipe::new(),
        )
        .unwrap();

    let mut encoder = factory.create_command_buffer().into();

    let vertex_data = [
        // top (0, 0, 1)
        Vertex::new([-1, -1, 1], [0, 0]),
        Vertex::new([1, -1, 1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        Vertex::new([-1, 1, -1], [1, 0]),
        Vertex::new([1, 1, -1], [0, 0]),
        Vertex::new([1, -1, -1], [0, 1]),
        Vertex::new([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        Vertex::new([1, -1, -1], [0, 0]),
        Vertex::new([1, 1, -1], [1, 0]),
        Vertex::new([1, 1, 1], [1, 1]),
        Vertex::new([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        Vertex::new([-1, -1, 1], [1, 0]),
        Vertex::new([-1, 1, 1], [0, 0]),
        Vertex::new([-1, 1, -1], [0, 1]),
        Vertex::new([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        Vertex::new([1, 1, -1], [1, 0]),
        Vertex::new([-1, 1, -1], [0, 0]),
        Vertex::new([-1, 1, 1], [0, 1]),
        Vertex::new([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        Vertex::new([1, -1, 1], [0, 0]),
        Vertex::new([-1, -1, 1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([1, -1, -1], [0, 1]),
    ];

    let index_data: &[u16] = &[
        0,  1,  2,  2,  3,  0, // top
        4,  5,  6,  6,  7,  4, // bottom
        8,  9, 10, 10, 11,  8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data);

    let texels = [[0x20, 0xA0, 0xC0, 0x00]];
    let (_, texture_view) = factory
        .create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single),
            &[&texels],
        )
        .unwrap();

    let sinfo =
        texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

    let proj = cgmath::perspective(Deg(45.0f32), 1.77, 1.0, 10.0);

    let data = pipe::Data {
        vbuf: vbuf,
        transform: (proj * default_view()).into(),
        locals: factory.create_constant_buffer(1),
        color: (texture_view, factory.create_sampler(sinfo)),
        out_color: gfx::handle::RenderTargetView<gfx_gl::Resources, ColorFormat>,
        out_depth: gfx::handle::DepthStencilView<gfx_gl::Resources, DepthFormat>,
    };


    let locals = Locals { transform: data.transform };
    encoder.update_constant_buffer(&data.locals, &locals);
    encoder.clear(&data.out_color, [0.1, 0.2, 0.3, 1.0]);
    encoder.clear_depth(&data.out_depth, 1.0);
    encoder.draw(&slice, &pso, &data);
}


fn default_view() -> Matrix4<f32> {
    Matrix4::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0f32, 0.0, 0.0),
        Vector3::unit_z(),
    )
}