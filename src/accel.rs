use super::*;

pub struct Accel(rjit::VarRef);

impl Accel {
    pub fn create(desc: rjit::AccelDesc) -> Self {
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
        mask: UInt32,
        payload: &[UInt32],
    ) -> Vec<UInt32> {
        // self.0.trace_ray(payload, o.into(), )
        // self.0.trace_ray()
        // self.0.trace_ray()
        todo!()
    }
}
