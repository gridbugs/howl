mod cardinal {
    use geometry::CardinalDirection::*;
    use geometry::OrdinalDirection::*;

    /// Test combinations of cardinal directions
    #[test]
    fn combinations() {
        assert_eq!(North.combine(East), Some(NorthEast));
        assert_eq!(North.combine(West), Some(NorthWest));
        assert_eq!(East.combine(South), Some(SouthEast));
        assert_eq!(East.combine(North), Some(NorthEast));
        assert_eq!(East.combine(West), None);
    }
}
