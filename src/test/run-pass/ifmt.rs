// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

struct A;
struct B;

#[fmt="foo"]
impl fmt::Signed for A {
    fn fmt(_: &A, f: &mut fmt::Formatter) { f.buf.write("aloha".as_bytes()); }
}
impl fmt::Signed for B {
    fn fmt(_: &B, f: &mut fmt::Formatter) { f.buf.write("adios".as_bytes()); }
}

macro_rules! t(($a:expr, $b:expr) => { assert_eq!($a, $b.to_owned()) })

pub fn main() {

    // Make sure there's a poly formatter that takes anything
    t!(format!("{:?}", 1), "1");
    t!(format!("{:?}", A), "{}");
    t!(format!("{:?}", ()), "()");
    t!(format!("{:?}", @(~1, "foo")), "@(~1, \"foo\")");

    // Various edge cases without formats
    t!(format!(""), "");
    t!(format!("hello"), "hello");
    t!(format!("hello \\{"), "hello {");

    // default formatters should work
    t!(format!("{}", 1i), "1");
    t!(format!("{}", 1i8), "1");
    t!(format!("{}", 1i16), "1");
    t!(format!("{}", 1i32), "1");
    t!(format!("{}", 1i64), "1");
    t!(format!("{}", 1u), "1");
    t!(format!("{}", 1u8), "1");
    t!(format!("{}", 1u16), "1");
    t!(format!("{}", 1u32), "1");
    t!(format!("{}", 1u64), "1");
    t!(format!("{}", 1.0f), "1");
    t!(format!("{}", 1.0f32), "1");
    t!(format!("{}", 1.0f64), "1");
    t!(format!("{}", "a"), "a");
    t!(format!("{}", ~"a"), "a");
    t!(format!("{}", @"a"), "a");
    t!(format!("{}", false), "false");
    t!(format!("{}", 'a'), "a");

    // At least exercise all the formats
    t!(format!("{:b}", true), "true");
    t!(format!("{:c}", '☃'), "☃");
    t!(format!("{:d}", 10), "10");
    t!(format!("{:i}", 10), "10");
    t!(format!("{:u}", 10u), "10");
    t!(format!("{:o}", 10u), "12");
    t!(format!("{:x}", 10u), "a");
    t!(format!("{:X}", 10u), "A");
    t!(format!("{:s}", "foo"), "foo");
    t!(format!("{:s}", ~"foo"), "foo");
    t!(format!("{:s}", @"foo"), "foo");
    t!(format!("{:p}", 0x1234 as *int), "0x1234");
    t!(format!("{:p}", 0x1234 as *mut int), "0x1234");
    t!(format!("{:d}", A), "aloha");
    t!(format!("{:d}", B), "adios");
    t!(format!("foo {:s} ☃☃☃☃☃☃", "bar"), "foo bar ☃☃☃☃☃☃");
    t!(format!("{1} {0}", 0, 1), "1 0");
    t!(format!("{foo} {bar}", foo=0, bar=1), "0 1");
    t!(format!("{foo} {1} {bar} {0}", 0, 1, foo=2, bar=3), "2 1 3 0");
    t!(format!("{} {0:s}", "a"), "a a");
    t!(format!("{} {0}", "a"), "a a");

    // Methods should probably work
    t!(format!("{0, plural, =1{a#} =2{b#} zero{c#} other{d#}}", 0u), "c0");
    t!(format!("{0, plural, =1{a#} =2{b#} zero{c#} other{d#}}", 1u), "a1");
    t!(format!("{0, plural, =1{a#} =2{b#} zero{c#} other{d#}}", 2u), "b2");
    t!(format!("{0, plural, =1{a#} =2{b#} zero{c#} other{d#}}", 3u), "d3");
    t!(format!("{0, select, a{a#} b{b#} c{c#} other{d#}}", "a"), "aa");
    t!(format!("{0, select, a{a#} b{b#} c{c#} other{d#}}", "b"), "bb");
    t!(format!("{0, select, a{a#} b{b#} c{c#} other{d#}}", "c"), "cc");
    t!(format!("{0, select, a{a#} b{b#} c{c#} other{d#}}", "d"), "dd");
    t!(format!("{1, select, a{#{0:s}} other{#{1}}}", "b", "a"), "ab");
    t!(format!("{1, select, a{#{0}} other{#{1}}}", "c", "b"), "bb");

    // Formatting strings and their arguments
    t!(format!("{:s}", "a"), "a");
    t!(format!("{:4s}", "a"), "a   ");
    t!(format!("{:>4s}", "a"), "   a");
    t!(format!("{:<4s}", "a"), "a   ");
    t!(format!("{:.4s}", "a"), "a");
    t!(format!("{:4.4s}", "a"), "a   ");
    t!(format!("{:4.4s}", "aaaaaaaaaaaaaaaaaa"), "aaaa");
    t!(format!("{:<4.4s}", "aaaaaaaaaaaaaaaaaa"), "aaaa");
    t!(format!("{:>4.4s}", "aaaaaaaaaaaaaaaaaa"), "aaaa");
    t!(format!("{:>10.4s}", "aaaaaaaaaaaaaaaaaa"), "aaaa");
    t!(format!("{:2.4s}", "aaaaa"), "aaaa");
    t!(format!("{:2.4s}", "aaaa"), "aaaa");
    t!(format!("{:2.4s}", "aaa"), "aaa");
    t!(format!("{:2.4s}", "aa"), "aa");
    t!(format!("{:2.4s}", "a"), "a ");
    t!(format!("{:0>2s}", "a"), "0a");
    t!(format!("{:.*s}", 4, "aaaaaaaaaaaaaaaaaa"), "aaaa");
    t!(format!("{:.1$s}", "aaaaaaaaaaaaaaaaaa", 4), "aaaa");
    t!(format!("{:1$s}", "a", 4), "a   ");
    t!(format!("{:-#s}", "a"), "a");
    t!(format!("{:+#s}", "a"), "a");

    // Formatting integers should select the right implementation based off the
    // type of the argument. Also, hex/octal/binary should be defined for
    // integers, but they shouldn't emit the negative sign.
    t!(format!("{:d}", -1i), "-1");
    t!(format!("{:d}", -1i8), "-1");
    t!(format!("{:d}", -1i16), "-1");
    t!(format!("{:d}", -1i32), "-1");
    t!(format!("{:d}", -1i64), "-1");
    t!(format!("{:t}", 1i), "1");
    t!(format!("{:t}", 1i8), "1");
    t!(format!("{:t}", 1i16), "1");
    t!(format!("{:t}", 1i32), "1");
    t!(format!("{:t}", 1i64), "1");
    t!(format!("{:x}", 1i), "1");
    t!(format!("{:x}", 1i8), "1");
    t!(format!("{:x}", 1i16), "1");
    t!(format!("{:x}", 1i32), "1");
    t!(format!("{:x}", 1i64), "1");
    t!(format!("{:X}", 1i), "1");
    t!(format!("{:X}", 1i8), "1");
    t!(format!("{:X}", 1i16), "1");
    t!(format!("{:X}", 1i32), "1");
    t!(format!("{:X}", 1i64), "1");
    t!(format!("{:o}", 1i), "1");
    t!(format!("{:o}", 1i8), "1");
    t!(format!("{:o}", 1i16), "1");
    t!(format!("{:o}", 1i32), "1");
    t!(format!("{:o}", 1i64), "1");

    t!(format!("{:u}", 1u), "1");
    t!(format!("{:u}", 1u8), "1");
    t!(format!("{:u}", 1u16), "1");
    t!(format!("{:u}", 1u32), "1");
    t!(format!("{:u}", 1u64), "1");
    t!(format!("{:t}", 1u), "1");
    t!(format!("{:t}", 1u8), "1");
    t!(format!("{:t}", 1u16), "1");
    t!(format!("{:t}", 1u32), "1");
    t!(format!("{:t}", 1u64), "1");
    t!(format!("{:x}", 1u), "1");
    t!(format!("{:x}", 1u8), "1");
    t!(format!("{:x}", 1u16), "1");
    t!(format!("{:x}", 1u32), "1");
    t!(format!("{:x}", 1u64), "1");
    t!(format!("{:X}", 1u), "1");
    t!(format!("{:X}", 1u8), "1");
    t!(format!("{:X}", 1u16), "1");
    t!(format!("{:X}", 1u32), "1");
    t!(format!("{:X}", 1u64), "1");
    t!(format!("{:o}", 1u), "1");
    t!(format!("{:o}", 1u8), "1");
    t!(format!("{:o}", 1u16), "1");
    t!(format!("{:o}", 1u32), "1");
    t!(format!("{:o}", 1u64), "1");

    // Test the flags for formatting integers
    t!(format!("{:3d}", 1),  "  1");
    t!(format!("{:>3d}", 1),  "  1");
    t!(format!("{:>+3d}", 1), " +1");
    t!(format!("{:<3d}", 1), "1  ");
    t!(format!("{:#d}", 1), "1");
    t!(format!("{:#x}", 10), "0xa");
    t!(format!("{:#X}", 10), "0xA");
    t!(format!("{:#5x}", 10), "  0xa");
    t!(format!("{:#o}", 10), "0o12");
    t!(format!("{:08x}", 10),  "0000000a");
    t!(format!("{:8x}", 10),   "       a");
    t!(format!("{:<8x}", 10),  "a       ");
    t!(format!("{:>8x}", 10),  "       a");
    t!(format!("{:#08x}", 10), "0x00000a");
    t!(format!("{:08d}", -10), "-0000010");
    t!(format!("{:x}", -1u8), "ff");
    t!(format!("{:X}", -1u8), "FF");
    t!(format!("{:t}", -1u8), "11111111");
    t!(format!("{:o}", -1u8), "377");
    t!(format!("{:#x}", -1u8), "0xff");
    t!(format!("{:#X}", -1u8), "0xFF");
    t!(format!("{:#t}", -1u8), "0b11111111");
    t!(format!("{:#o}", -1u8), "0o377");

    // Signed combinations
    t!(format!("{:+5d}", 1),  "   +1");
    t!(format!("{:+5d}", -1), "   -1");
    t!(format!("{:05d}", 1),   "00001");
    t!(format!("{:05d}", -1),  "-0001");
    t!(format!("{:+05d}", 1),  "+0001");
    t!(format!("{:+05d}", -1), "-0001");

    // Some float stuff
    t!(format!("{:f}", 1.0f), "1");
    t!(format!("{:f}", 1.0f32), "1");
    t!(format!("{:f}", 1.0f64), "1");
    t!(format!("{:.3f}", 1.0f), "1.000");
    t!(format!("{:10.3f}", 1.0f),   "     1.000");
    t!(format!("{:+10.3f}", 1.0f),  "    +1.000");
    t!(format!("{:+10.3f}", -1.0f), "    -1.000");

    test_write();
    test_print();
}

// Basic test to make sure that we can invoke the `write!` macro with an
// io::Writer instance.
fn test_write() {
    use std::rt::io::Decorator;
    use std::rt::io::mem::MemWriter;
    use std::rt::io;
    use std::str;

    let mut buf = MemWriter::new();
    write!(&mut buf as &mut io::Writer, "{}", 3);
    {
        let w = &mut buf as &mut io::Writer;
        write!(w, "{foo}", foo=4);
        write!(w, "{:s}", "hello");
        writeln!(w, "{}", "line");
        writeln!(w, "{foo}", foo="bar");
    }

    let s = str::from_bytes_owned(buf.inner());
    t!(s, "34helloline\nbar\n");
}

// Just make sure that the macros are defined, there's not really a lot that we
// can do with them just yet (to test the output)
fn test_print() {
    print!("hi");
    print!("{:?}", ~[0u8]);
    println!("hello");
    println!("this is a {}", "test");
    println!("{foo}", foo="bar");
}
