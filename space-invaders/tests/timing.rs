use space_invaders::frame::{new_frame, Drawable};
use space_invaders::invaders::Army;
use space_invaders::shot::Shot;
use std::time::Duration;

fn millis(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

mod shot {
    use super::*;

    #[test]
    fn stays_put_until_its_step_interval_elapses() {
        let mut shot = Shot::new(5, 10);

        shot.update(millis(49));

        assert_eq!(shot.y, 10);
    }

    #[test]
    fn rises_one_row_per_step_interval() {
        let mut shot = Shot::new(5, 10);

        shot.update(millis(50));
        shot.update(millis(50));

        assert_eq!(shot.y, 8);
    }

    #[test]
    fn dies_only_once_its_explosion_has_finished() {
        let mut shot = Shot::new(5, 10);
        shot.explode();

        shot.update(millis(249));
        let alive_before = !shot.is_dead();
        shot.update(millis(1));

        assert!(alive_before);
        assert!(shot.is_dead());
    }

    #[test]
    fn does_not_rise_while_exploding() {
        let mut shot = Shot::new(5, 10);
        shot.explode();

        shot.update(millis(300));

        assert_eq!(shot.y, 10);
    }
}

mod army {
    use super::*;

    fn leftmost_x(army: &Army) -> usize {
        army.invaders.iter().map(|invader| invader.x).min().unwrap()
    }

    fn glyph_of_first_invader(army: &Army) -> &'static str {
        let mut frame = new_frame();
        army.draw(&mut frame);
        let invader = &army.invaders[0];
        frame[invader.x][invader.y]
    }

    #[test]
    fn holds_position_until_its_move_interval_elapses() {
        let mut army = Army::new();
        let start = leftmost_x(&army);

        let moved = army.try_to_move(millis(1999));

        assert!(!moved);
        assert_eq!(leftmost_x(&army), start);
    }

    #[test]
    fn marches_right_once_its_move_interval_elapses() {
        let mut army = Army::new();
        let start = leftmost_x(&army);

        let moved = army.try_to_move(millis(2000));

        assert!(moved);
        assert_eq!(leftmost_x(&army), start + 1);
    }

    #[test]
    fn restarts_its_move_interval_after_each_move() {
        let mut army = Army::new();
        army.try_to_move(millis(2000));

        let moved_again_early = army.try_to_move(millis(1999));
        let moved_again_on_time = army.try_to_move(millis(1));

        assert!(!moved_again_early);
        assert!(moved_again_on_time);
    }

    #[test]
    fn flips_its_glyph_halfway_through_each_move_interval() {
        let mut army = Army::new();
        let early = glyph_of_first_invader(&army);

        army.try_to_move(millis(1000));
        let halfway = glyph_of_first_invader(&army);
        army.try_to_move(millis(1));
        let late = glyph_of_first_invader(&army);

        assert_eq!(early, "x");
        assert_eq!(halfway, "x");
        assert_eq!(late, "X");
    }
}
