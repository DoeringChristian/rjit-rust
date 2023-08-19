use super::*;

pub struct Accel(rjit::VarRef);

pub use rjit::{HitGroupDesc, InstanceDesc, MissGroupDesc, ModuleDesc, SBTDesc};

pub enum GeometryDesc<'a> {
    Triangles {
        vertices: &'a Float32,
        indices: &'a UInt32,
    },
}

pub struct AccelDesc<'a> {
    pub sbt: SBTDesc<'a>,
    pub geometries: &'a [GeometryDesc<'a>],
    pub instances: &'a [InstanceDesc],
}

impl Accel {
    // TODO: AccelDesc in this crate
    pub fn create(desc: AccelDesc) -> Self {
        let geometries = desc
            .geometries
            .iter()
            .map(|g| match g {
                GeometryDesc::Triangles { vertices, indices } => rjit::GeometryDesc::Triangles {
                    vertices: vertices.internal(),
                    indices: indices.internal(),
                },
            })
            .collect::<Vec<_>>();
        let desc = rjit::AccelDesc {
            sbt: desc.sbt,
            geometries: &geometries,
            instances: desc.instances,
        };
        Self(TRACE.accel(desc).unwrap())
    }

    pub fn trace_ray(
        &self,
        o: Point3<f32>,
        d: Vec3<f32>,
        tmin: Float32,
        tmax: Float32,
        t: Float32,
        vis_mask: UInt32,
        ray_flags: UInt32,
        sbt_offset: UInt32,
        sbt_stride: UInt32,
        miss_sbt: UInt32,
        mask: Bool,
        payload: &[UInt32],
    ) -> Vec<UInt32> {
        let o = [o.x.internal(), o.y.internal(), o.z.internal()];
        let d = [d.x.internal(), d.y.internal(), d.z.internal()];
        let tmin = tmin.internal();
        let tmax = tmax.internal();
        let t = t.internal();
        let vis_mask = vis_mask.internal();
        let ray_flags = ray_flags.internal();
        let sbt_offset = sbt_offset.internal();
        let sbt_stride = sbt_stride.internal();
        let miss_sbt = miss_sbt.internal();
        let mask = mask.internal();
        let payload = payload
            .into_iter()
            .map(|p| p.internal())
            .collect::<Vec<_>>();
        let res = self
            .0
            .trace_ray(
                &payload,
                o,
                d,
                tmin,
                tmax,
                t,
                Some(vis_mask),
                Some(ray_flags),
                Some(sbt_offset),
                Some(sbt_stride),
                Some(miss_sbt),
                Some(mask),
            )
            .unwrap();
        res.into_iter()
            .map(|r| Var(r, std::marker::PhantomData))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::accel::*;
    use crate::*;
    #[test]
    fn trace_ray() {
        set_backend(["optix"]).unwrap();
        let miss_and_closesthit_ptx = r##"
.version 8.0
.target sm_86
.address_size 64

.entry __miss__ms() {
	.reg .b32 %r<6>;
	mov.b32 %r0, 0;
	mov.b32 %r1, 0;
        
	call _optix_set_payload, (%r0, %r1);
	ret;
}

.entry __closesthit__ch() {
	.reg .b32 %i<5>;
	.reg .b32 %v<5>;
	mov.b32 %i0, 0;
	mov.b32 %i1, 1;
	mov.b32 %i2, 2;
	mov.b32 %i3, 3;
	mov.b32 %i4, 4;

        mov.b32 %v0, 1;
	call _optix_set_payload, (%i0, %v0);
        
        call (%v1), _optix_read_primitive_idx, ();
	call _optix_set_payload, (%i1, %v1);
        
        call (%v2), _optix_read_instance_id, ();
	call _optix_set_payload, (%i2, %v2);
        
	.reg .f32 %f<2>;
        call (%f0, %f1), _optix_get_triangle_barycentrics, ();
        mov.b32 %v3, %f0;
        mov.b32 %v4, %f1;
	call _optix_set_payload, (%i3, %v3);
	call _optix_set_payload, (%i4, %v4);
        
	ret;
}
"##;
        let indices = array(&[0u32, 1, 2]);
        let vertices = array(&[1.0f32, 0., 1., 0., 1., 1., 1., 1., 1.]);

        let desc = AccelDesc {
            sbt: SBTDesc {
                hit_groups: &[HitGroupDesc {
                    closest_hit: ModuleDesc {
                        asm: miss_and_closesthit_ptx,
                        entry_point: "__closesthit__ch",
                    },
                    ..Default::default()
                }],
                miss_groups: &[MissGroupDesc {
                    miss: ModuleDesc {
                        asm: miss_and_closesthit_ptx,
                        entry_point: "__miss__ms",
                    },
                }],
            },
            geometries: &[GeometryDesc::Triangles {
                vertices: &vertices,
                indices: &indices,
            }],
            instances: &[InstanceDesc {
                hit_group: 0,
                geometry: 0,
                transform: [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.],
            }],
        };
        let accel = Accel::create(desc);

        let o = point3!([0.6f32, 0.6f32].as_slice(), 0.6f32, 0f32);
        let d = vec3!(0.0f32, 0.0f32, [1.0f32, -1.].as_slice());
        let payload = [literal(0), literal(0), literal(0), literal(0), literal(0)];
        let payload = accel.trace_ray(
            o,
            d,
            literal(0.),
            literal(100000.),
            literal(0.),
            literal(0xff),
            literal(0),
            literal(0),
            literal(0),
            literal(0),
            literal(true),
            payload.as_slice(),
        );

        let valid = payload[0].cast::<bool>();
        valid.schedule();
        eval();
        let valid: Vec<_> = valid.into();
        assert_eq!(valid, vec![true, false]);
    }
}
