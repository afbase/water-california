pub trait ObservationInterpolation<T> {
    type Error;
    fn derive_daily_observations_from_monthly_observations(
        input: impl Iterator<Item = T>,
    ) -> Result<Vec<T>, Self::Error>;
    fn derive_daily_observations_from_mixed_time_observations(
        input: impl Iterator<Item = T>,
    ) -> Result<Vec<T>, Self::Error>;
}
