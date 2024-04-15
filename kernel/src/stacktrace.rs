pub fn print_stacktrace() {
    crate::serial_println!("Stacktrace:");
    unsafe {
        backtrace::trace_unsynchronized(|frame| {
            backtrace::resolve_frame_unsynchronized(frame, |symbol| {
                crate::serial_println!("  Frame:");
                if let Some(name) = symbol.name() {
                    crate::serial_print!("    Name: {name}");
                }
                if let Some(filename) = symbol.filename_raw() {
                    crate::serial_println!("    Filename: ");
                    match filename {
                        backtrace::BytesOrWideString::Bytes(b) => {
                            for b in b.iter() {
                                crate::serial_print!("{}", *b as char);
                            }
                        },
                        backtrace::BytesOrWideString::Wide(s) => {
                            for s in s.iter() {
                                crate::serial_print!("{}", char::from_u32_unchecked(*s as u32));
                            }
                        },
                    }
                    crate::serial_println!();
                }
                if let Some(lineno) = symbol.lineno() {
                    crate::serial_println!("    Line: {lineno}");
                }
                if let Some(colno) = symbol.colno() {
                    crate::serial_println!("    Column: {colno}");
                }
            });

            true
        });
    }
}
