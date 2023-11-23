#[cfg(test)]
mod tests {
    use gc_simulator::controller::PIConfig;
    use gc_simulator::controller::PIController;

    #[test]
    fn test_create_default_pi_controller() {
        let controller = PIController::new(PIConfig::new());
        assert_eq!(controller.cfg, PIConfig::new());
        assert_eq!(controller.integral, 0.0);
    }

    #[test]
    fn test_pi_controller_initialization() {
        let cfg = PIConfig {
            kp: 2.0,
            ti: 1.0,
            tt: 0.5,
            period: 1.0,
            min: -10.0,
            max: 10.0,
        };
        let controller = PIController::new(cfg.clone());
        assert_eq!(controller.cfg, cfg);
        assert_eq!(controller.integral, 0.0);
    }

    #[test]
    fn test_new_controller() {
        let cfg = PIConfig {
            kp: 1.0,
            ti: 1.0,
            tt: 1.0,
            period: 1.0,
            min: 0.0,
            max: 10.0,
        };
        let controller = PIController::new(cfg.clone());
        assert_eq!(controller.cfg, cfg);
        assert_eq!(controller.integral, 0.0);
    }

    #[test]
    fn test_update_with_valid_parameters() {
        let mut controller = PIController::new(PIConfig {
            kp: 1.0,
            ti: 1.0,
            tt: 1.0,
            period: 1.0,
            min: -10.0,
            max: 10.0,
        });
        assert_eq!(controller.update(5.0, 10.0, 0.0, 0.0), Ok(5.0));
    }

    #[test]
    fn test_update_with_zero_ti_tt() {
        let mut controller = PIController::new(PIConfig {
            kp: 1.0,
            ti: 0.0,
            tt: 0.0,
            period: 1.0,
            min: -10.0,
            max: 10.0,
        });
        assert_eq!(controller.update(5.0, 10.0, 0.0, 0.0), Err("`tt` or `ti` cannot be zero"));
    }

    #[test]
    fn test_output() {
        let mut controller = PIController::new(PIConfig {
            kp: 1.0,
            ti: 1.0,
            tt: 1.0,
            period: 1.0,
            min: 0.0,
            max: 10.0,
        });
        controller.integral = 5.0;
        let output = controller.output(5.0, 10.0);
        assert_eq!(output.0, 10.0);
    }

    #[test]
    fn test_next() {
        let mut controller = PIController::new(PIConfig {
            kp: 1.0,
            ti: 1.0,
            tt: 1.0,
            period: 1.0,
            min: 0.0,
            max: 10.0,
        });
        let output = controller.next(5.0, 10.0);
        assert!(output >= 0.0 && output <= 10.0);
    }
}
