fn get_ports(is_full: bool) -> Box<dyn Iterator<Item = u16>> {
    if is_full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(crate::ports::MOST_COMMON_PORTS.to_owned().into_iter())
    }
}
