#[derive(Debug, PartialEq, Clone, Default)]
pub struct PIController {
    pub cfg: PIConfig,
    pub integral: f64,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PIConfig {
    pub kp: f64,
    pub ti: f64,
    pub tt: f64,
    pub period: f64,
    pub min: f64,
    pub max: f64,
}

impl PIController {
    pub fn new(cfg: PIConfig) -> Self {
        Self {
            cfg,
            ..Default::default()
        }
    }

    pub fn update(
        &mut self,
        input: f64,
        set_point: f64,
        raw_output: f64,
        output: f64,
    ) -> Result<f64, &'static str> {
        if self.cfg.ti == 0.0 || self.cfg.tt == 0.0 {
            return Err("`tt` or `ti` cannot be zero");
        }

        let diff: f64 = set_point - input;
        let integral_update = (self.cfg.kp * self.cfg.period / self.cfg.ti) * diff
            + (self.cfg.period / self.cfg.tt) * (output - raw_output);

        // Update the integral term, with clamping to avoid windup
        self.integral = (self.integral + integral_update).clamp(self.cfg.min, self.cfg.max);

        Ok(self.integral)
    }

    pub fn output(&mut self, input: f64, set_point: f64) -> (f64, f64) {
        let prop: f64 = self.cfg.kp * (set_point - input);
        let raw_output: f64 = prop + self.integral;

        let max = self.cfg.max;
        let min = self.cfg.min;
        let output: f64 = raw_output.clamp(min, max);

        (output, prop)
    }

    pub fn next(&mut self, input: f64, set_point: f64) -> f64 {
        let (output, prop) = self.output(input, set_point);
        self.integral += prop * self.cfg.period;
        output
    }
}

impl PIConfig {
    pub fn new() -> Self {
        Default::default()
    }
}
