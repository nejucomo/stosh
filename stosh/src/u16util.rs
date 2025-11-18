pub(crate) trait IntoU16 {
    fn into_u16(self) -> u16;
}

impl<N> IntoU16 for N
where
    N: TryInto<u16, Error = std::num::TryFromIntError>,
{
    fn into_u16(self) -> u16 {
        self.try_into().unwrap()
    }
}
