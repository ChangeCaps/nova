use glam::*;
use nova_render::{color::Color, mesh::Mesh};

use crate::Vertex3d;

pub struct Cube {
    pub size: Vec3,
}

impl Into<Mesh<Vertex3d>> for Cube {
    #[inline]
    fn into(mut self) -> Mesh<Vertex3d> {
        self.size /= 2.0;

        let mut mesh = Mesh::default();

        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, self.size.y, self.size.z),
            normal: Vec3::Y,
            uv: Vec2::new(0.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, self.size.y, -self.size.z),
            normal: Vec3::Y,
            uv: Vec2::new(1.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, self.size.y, self.size.z),
            normal: Vec3::Y,
            uv: Vec2::new(0.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, self.size.y, -self.size.z),
            normal: Vec3::Y,
            uv: Vec2::new(1.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });

        mesh.indices.push(0);
        mesh.indices.push(1);
        mesh.indices.push(2);
        mesh.indices.push(2);
        mesh.indices.push(1);
        mesh.indices.push(3);

        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, -self.size.y, self.size.z),
            normal: -Vec3::Y,
            uv: Vec2::new(0.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, -self.size.y, self.size.z),
            normal: -Vec3::Y,
            uv: Vec2::new(1.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, -self.size.y, -self.size.z),
            normal: -Vec3::Y,
            uv: Vec2::new(0.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, -self.size.y, -self.size.z),
            normal: -Vec3::Y,
            uv: Vec2::new(1.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });

        mesh.indices.push(4);
        mesh.indices.push(5);
        mesh.indices.push(6);
        mesh.indices.push(6);
        mesh.indices.push(5);
        mesh.indices.push(7);

        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, self.size.y, self.size.z),
            normal: Vec3::X,
            uv: Vec2::new(0.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, -self.size.y, self.size.z),
            normal: Vec3::X,
            uv: Vec2::new(1.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, self.size.y, -self.size.z),
            normal: Vec3::X,
            uv: Vec2::new(0.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, -self.size.y, -self.size.z),
            normal: Vec3::X,
            uv: Vec2::new(1.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });

        mesh.indices.push(8);
        mesh.indices.push(9);
        mesh.indices.push(10);
        mesh.indices.push(10);
        mesh.indices.push(9);
        mesh.indices.push(11);

        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, self.size.y, self.size.z),
            normal: -Vec3::X,
            uv: Vec2::new(0.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, self.size.y, -self.size.z),
            normal: -Vec3::X,
            uv: Vec2::new(1.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, -self.size.y, self.size.z),
            normal: -Vec3::X,
            uv: Vec2::new(0.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, -self.size.y, -self.size.z),
            normal: -Vec3::X,
            uv: Vec2::new(1.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });

        mesh.indices.push(12);
        mesh.indices.push(13);
        mesh.indices.push(14);
        mesh.indices.push(14);
        mesh.indices.push(13);
        mesh.indices.push(15);

        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, self.size.y, self.size.z),
            normal: Vec3::Z,
            uv: Vec2::new(0.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, self.size.y, self.size.z),
            normal: Vec3::Z,
            uv: Vec2::new(1.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, -self.size.y, self.size.z),
            normal: Vec3::Z,
            uv: Vec2::new(0.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, -self.size.y, self.size.z),
            normal: Vec3::Z,
            uv: Vec2::new(1.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });

        mesh.indices.push(16);
        mesh.indices.push(17);
        mesh.indices.push(18);
        mesh.indices.push(18);
        mesh.indices.push(17);
        mesh.indices.push(19);

        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, self.size.y, -self.size.z),
            normal: -Vec3::Z,
            uv: Vec2::new(0.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(self.size.x, -self.size.y, -self.size.z),
            normal: -Vec3::Z,
            uv: Vec2::new(1.0, 0.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, self.size.y, -self.size.z),
            normal: -Vec3::Z,
            uv: Vec2::new(0.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });
        mesh.vertices.push(Vertex3d {
            position: Vec3::new(-self.size.x, -self.size.y, -self.size.z),
            normal: -Vec3::Z,
            uv: Vec2::new(1.0, 1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        });

        mesh.indices.push(20);
        mesh.indices.push(21);
        mesh.indices.push(22);
        mesh.indices.push(22);
        mesh.indices.push(21);
        mesh.indices.push(23);

        mesh
    }
}
