#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use nalgebra::distance;
    use robusta_core::point::Point;

    #[test]
    fn sensible_default() {
        let p1 = Point::default();
        // dbg!(distance);
        assert_eq!(p1, Point::new(0., 0., 0.));
        assert_eq!(p1, Point::origin());
    }

    #[test]
    fn distance_btwn_pts1() {
        let p1 = Point::origin();
        let p2 = Point::new(3., 4., 0.);
        let distance = distance(&p1.coordinates, &p2.coordinates);
        // dbg!(distance);
        assert_relative_eq!(distance, 5.);
    }

    #[test]
    fn distance_btwn_pts2() {
        let p1 = Point::origin();
        let p2 = Point::new(2., 2., 0.);
        let distance = distance(&p1.coordinates, &p2.coordinates);
        // dbg!(distance);
        assert_relative_eq!(distance, 8.0_f64.sqrt());
    }
}
