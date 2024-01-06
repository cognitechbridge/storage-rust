#[macro_export]

macro_rules! map_anyhow_io {
    ($expr:expr, $msg:expr) => {
        $expr.map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: {}", $msg, e)
        ))
    };
}