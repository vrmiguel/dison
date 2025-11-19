use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
    str::from_utf8_unchecked,
};

use serde::Serialize;

use crate::{Json, JsonPretty};

/// Display this value as JSON.
impl<T: Serialize> Display for Json<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write_serialized::<_, false>(fmt, &self.0)
    }
}

/// Display this value as pretty JSON.
impl<T: Serialize> Display for JsonPretty<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write_serialized::<_, true>(fmt, &self.0)
    }
}

impl<T: Debug + Serialize> Debug for JsonPretty<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "JsonPretty({:?})", self.0)
    }
}

impl<T: Debug + Serialize> Debug for Json<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Json({:?})", self.0)
    }
}

#[inline]
fn write_serialized<T: Serialize, const IS_PRETTY: bool>(
    fmt: &mut Formatter<'_>,
    to_serialize: &T,
) -> fmt::Result {
    struct IoBridge<'a, 'b>(&'a mut Formatter<'b>);

    impl<'a, 'b> io::Write for IoBridge<'a, 'b> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            // Safety: `buf` _should_ always be valid-UTF8 since
            //         it's always called by `serde_json`'s Serializer,
            //         which works on valid UTF-8
            let utf8 = unsafe { from_utf8_unchecked(buf) };

            self.0
                .write_str(utf8)
                .map(|_| buf.len())
                .map_err(|_| io::Error::from(io::ErrorKind::Other))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let writer = IoBridge(fmt);

    if IS_PRETTY {
        serde_json::to_writer_pretty(writer, to_serialize).map_err(|_| fmt::Error)
    } else {
        serde_json::to_writer(writer, to_serialize).map_err(|_| fmt::Error)
    }
}
