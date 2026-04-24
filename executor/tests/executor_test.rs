use executor::{Executor, Pose};

mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
        e.execute("M");
        let pose = e.query();
        assert_eq!(pose.x, 1);
        assert_eq!(pose.y, 0);
        assert_eq!(pose.heading, 'E');
    }

    #[test]
    fn should_return_y_minus_1_given_command_is_m_and_facing_is_s() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'S'));
        e.execute("M");
        let pose = e.query();
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, -1);
        assert_eq!(pose.heading, 'S');
    }

    #[test]
    fn should_return_x_minus_1_given_command_is_m_and_facing_is_w() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'W'));
        e.execute("M");
        let pose = e.query();
        assert_eq!(pose.x, -1);
        assert_eq!(pose.y, 0);
        assert_eq!(pose.heading, 'W');
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_m_and_facing_is_n() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'N'));
        e.execute("M");
        let pose = e.query();
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 1);
        assert_eq!(pose.heading, 'N');
    }
}

mod left_turn_tests {
    use super::*;

    #[test]
    fn should_return_heading_n_given_command_is_l_and_facing_is_e() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
        e.execute("L");
        let pose = e.query();
        assert_eq!(pose.heading, 'N');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }

    #[test]
    fn should_return_heading_e_given_command_is_l_and_facing_is_s() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'S'));
        e.execute("L");
        let pose = e.query();
        assert_eq!(pose.heading, 'E');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }

    #[test]
    fn should_return_heading_s_given_command_is_l_and_facing_is_w() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'W'));
        e.execute("L");
        let pose = e.query();
        assert_eq!(pose.heading, 'S');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }

    #[test]
    fn should_return_heading_w_given_command_is_l_and_facing_is_n() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'N'));
        e.execute("L");
        let pose = e.query();
        assert_eq!(pose.heading, 'W');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }
}

mod right_turn_tests {
    use super::*;

    #[test]
    fn should_return_heading_s_given_command_is_r_and_facing_is_e() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
        e.execute("R");
        let pose = e.query();
        assert_eq!(pose.heading, 'S');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }

    #[test]
    fn should_return_heading_w_given_command_is_r_and_facing_is_s() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'S'));
        e.execute("R");
        let pose = e.query();
        assert_eq!(pose.heading, 'W');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }

    #[test]
    fn should_return_heading_n_given_command_is_r_and_facing_is_w() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'W'));
        e.execute("R");
        let pose = e.query();
        assert_eq!(pose.heading, 'N');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }

    #[test]
    fn should_return_heading_e_given_command_is_r_and_facing_is_n() {
        let mut e = Executor::with_pose(Pose::new(0, 0, 'N'));
        e.execute("R");
        let pose = e.query();
        assert_eq!(pose.heading, 'E');
        assert_eq!(pose.x, 0);
        assert_eq!(pose.y, 0);
    }
}
