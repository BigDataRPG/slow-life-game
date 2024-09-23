use rand::Rng;

pub fn is_critical_hit(rng: &mut rand::rngs::ThreadRng, critical_rate: f32) -> bool {
    rng.gen::<f32>() < critical_rate
}
