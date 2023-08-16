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
