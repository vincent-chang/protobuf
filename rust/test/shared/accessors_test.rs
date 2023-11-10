// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

//! Tests covering accessors for singular bool, int32, int64, and bytes fields.

use googletest::prelude::*;
use matchers::{is_set, is_unset};
use paste::paste;
use protobuf::Optional;
use unittest_proto::proto2_unittest::{TestAllTypes, TestAllTypes_};

#[test]
fn test_default_accessors() {
    let msg = TestAllTypes::new();
    assert_that!(
        msg,
        matches_pattern!(TestAllTypes{
            default_int32(): eq(41),
            default_int64(): eq(42),
            default_uint32(): eq(43),
            default_uint64(): eq(44),
            default_sint32(): eq(-45),
            default_sint64(): eq(46),
            default_fixed32(): eq(47),
            default_fixed64(): eq(48),
            default_sfixed32(): eq(49),
            default_sfixed64(): eq(-50),
            default_float(): eq(51.5),
            default_double(): eq(52000.0),
            default_bool(): eq(true),
        })
    );
}

#[test]
fn test_optional_fixed32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_fixed32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_fixed32(), eq(0));

    msg.optional_fixed32_set(Some(99));
    assert_that!(msg.optional_fixed32_opt(), eq(Optional::Set(99)));
    assert_that!(msg.optional_fixed32(), eq(99));

    msg.optional_fixed32_set(None);
    assert_that!(msg.optional_fixed32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_fixed32(), eq(0));
}

#[test]
fn test_default_fixed32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_fixed32(), eq(47));
    assert_that!(msg.default_fixed32_mut().get(), eq(47));
    assert_that!(msg.default_fixed32_mut().is_set(), eq(false));
    assert_that!(msg.default_fixed32_opt(), eq(Optional::Unset(47)));

    msg.default_fixed32_mut().set(999);
    assert_that!(msg.default_fixed32(), eq(999));
    assert_that!(msg.default_fixed32_mut().get(), eq(999));
    assert_that!(msg.default_fixed32_mut().is_set(), eq(true));
    assert_that!(msg.default_fixed32_opt(), eq(Optional::Set(999)));

    msg.default_fixed32_mut().clear();
    assert_that!(msg.default_fixed32(), eq(47));
    assert_that!(msg.default_fixed32_mut().get(), eq(47));
    assert_that!(msg.default_fixed32_mut().is_set(), eq(false));
    assert_that!(msg.default_fixed32_opt(), eq(Optional::Unset(47)));

    msg.default_fixed32_mut().or_default();
    assert_that!(msg.default_fixed32(), eq(47));
    assert_that!(msg.default_fixed32_mut().get(), eq(47));
    assert_that!(msg.default_fixed32_mut().is_set(), eq(true));
    assert_that!(msg.default_fixed32_opt(), eq(Optional::Set(47)));
}

#[test]
fn test_optional_fixed64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_fixed64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_fixed64(), eq(0));

    msg.optional_fixed64_set(Some(2000));
    assert_that!(msg.optional_fixed64_opt(), eq(Optional::Set(2000)));
    assert_that!(msg.optional_fixed64(), eq(2000));

    msg.optional_fixed64_set(None);
    assert_that!(msg.optional_fixed64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_fixed64(), eq(0));
}

#[test]
fn test_default_fixed64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_fixed64(), eq(48));
    assert_that!(msg.default_fixed64_mut().get(), eq(48));
    assert_that!(msg.default_fixed64_mut().is_set(), eq(false));
    assert_that!(msg.default_fixed64_opt(), eq(Optional::Unset(48)));

    msg.default_fixed64_mut().set(999);
    assert_that!(msg.default_fixed64(), eq(999));
    assert_that!(msg.default_fixed64_mut().get(), eq(999));
    assert_that!(msg.default_fixed64_mut().is_set(), eq(true));
    assert_that!(msg.default_fixed64_opt(), eq(Optional::Set(999)));

    msg.default_fixed64_mut().clear();
    assert_that!(msg.default_fixed64(), eq(48));
    assert_that!(msg.default_fixed64_mut().get(), eq(48));
    assert_that!(msg.default_fixed64_mut().is_set(), eq(false));
    assert_that!(msg.default_fixed64_opt(), eq(Optional::Unset(48)));

    msg.default_fixed64_mut().or_default();
    assert_that!(msg.default_fixed64(), eq(48));
    assert_that!(msg.default_fixed64_mut().get(), eq(48));
    assert_that!(msg.default_fixed64_mut().is_set(), eq(true));
    assert_that!(msg.default_fixed64_opt(), eq(Optional::Set(48)));
}

#[test]
fn test_optional_int32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_int32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_int32(), eq(0));

    msg.optional_int32_set(Some(1));
    assert_that!(msg.optional_int32_opt(), eq(Optional::Set(1)));
    assert_that!(msg.optional_int32(), eq(1));

    msg.optional_int32_set(None);
    assert_that!(msg.optional_int32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_int32(), eq(0));
}

#[test]
fn test_default_int32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_int32(), eq(41));
    assert_that!(msg.default_int32_mut().get(), eq(41));
    assert_that!(msg.default_int32_mut().is_set(), eq(false));
    assert_that!(msg.default_int32_opt(), eq(Optional::Unset(41)));

    msg.default_int32_mut().set(999);
    assert_that!(msg.default_int32(), eq(999));
    assert_that!(msg.default_int32_mut().get(), eq(999));
    assert_that!(msg.default_int32_mut().is_set(), eq(true));
    assert_that!(msg.default_int32_opt(), eq(Optional::Set(999)));

    msg.default_int32_mut().clear();
    assert_that!(msg.default_int32(), eq(41));
    assert_that!(msg.default_int32_mut().get(), eq(41));
    assert_that!(msg.default_int32_mut().is_set(), eq(false));
    assert_that!(msg.default_int32_opt(), eq(Optional::Unset(41)));

    msg.default_int32_mut().or_default();
    assert_that!(msg.default_int32(), eq(41));
    assert_that!(msg.default_int32_mut().get(), eq(41));
    assert_that!(msg.default_int32_mut().is_set(), eq(true));
    assert_that!(msg.default_int32_opt(), eq(Optional::Set(41)));
}

#[test]
fn test_optional_int64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_int64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_int64(), eq(0));

    msg.optional_int64_set(Some(42));
    assert_that!(msg.optional_int64_opt(), eq(Optional::Set(42)));
    assert_that!(msg.optional_int64(), eq(42));

    msg.optional_int64_set(None);
    assert_that!(msg.optional_int64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_int64(), eq(0));
}

#[test]
fn test_default_int64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_int64(), eq(42));
    assert_that!(msg.default_int64_mut().get(), eq(42));
    assert_that!(msg.default_int64_mut().is_set(), eq(false));
    assert_that!(msg.default_int64_opt(), eq(Optional::Unset(42)));

    msg.default_int64_mut().set(999);
    assert_that!(msg.default_int64(), eq(999));
    assert_that!(msg.default_int64_mut().get(), eq(999));
    assert_that!(msg.default_int64_mut().is_set(), eq(true));
    assert_that!(msg.default_int64_opt(), eq(Optional::Set(999)));

    msg.default_int64_mut().clear();
    assert_that!(msg.default_int64(), eq(42));
    assert_that!(msg.default_int64_mut().get(), eq(42));
    assert_that!(msg.default_int64_mut().is_set(), eq(false));
    assert_that!(msg.default_int64_opt(), eq(Optional::Unset(42)));

    msg.default_int64_mut().or_default();
    assert_that!(msg.default_int64(), eq(42));
    assert_that!(msg.default_int64_mut().get(), eq(42));
    assert_that!(msg.default_int64_mut().is_set(), eq(true));
    assert_that!(msg.default_int64_opt(), eq(Optional::Set(42)));
}

#[test]
fn test_optional_sint32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_sint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_sint32(), eq(0));

    msg.optional_sint32_set(Some(-22));
    assert_that!(msg.optional_sint32_opt(), eq(Optional::Set(-22)));
    assert_that!(msg.optional_sint32(), eq(-22));

    msg.optional_sint32_set(None);
    assert_that!(msg.optional_sint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_sint32(), eq(0));
}

#[test]
fn test_default_sint32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_sint32(), eq(-45));
    assert_that!(msg.default_sint32_mut().get(), eq(-45));
    assert_that!(msg.default_sint32_mut().is_set(), eq(false));
    assert_that!(msg.default_sint32_opt(), eq(Optional::Unset(-45)));

    msg.default_sint32_mut().set(999);
    assert_that!(msg.default_sint32(), eq(999));
    assert_that!(msg.default_sint32_mut().get(), eq(999));
    assert_that!(msg.default_sint32_mut().is_set(), eq(true));
    assert_that!(msg.default_sint32_opt(), eq(Optional::Set(999)));

    msg.default_sint32_mut().clear();
    assert_that!(msg.default_sint32(), eq(-45));
    assert_that!(msg.default_sint32_mut().get(), eq(-45));
    assert_that!(msg.default_sint32_mut().is_set(), eq(false));
    assert_that!(msg.default_sint32_opt(), eq(Optional::Unset(-45)));

    msg.default_sint32_mut().or_default();
    assert_that!(msg.default_sint32(), eq(-45));
    assert_that!(msg.default_sint32_mut().get(), eq(-45));
    assert_that!(msg.default_sint32_mut().is_set(), eq(true));
    assert_that!(msg.default_sint32_opt(), eq(Optional::Set(-45)));
}

#[test]
fn test_optional_sint64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_sint64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_sint64(), eq(0));

    msg.optional_sint64_set(Some(7000));
    assert_that!(msg.optional_sint64_opt(), eq(Optional::Set(7000)));
    assert_that!(msg.optional_sint64(), eq(7000));

    msg.optional_sint64_set(None);
    assert_that!(msg.optional_sint64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_sint64(), eq(0));
}

#[test]
fn test_default_sint64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_sint64(), eq(46));
    assert_that!(msg.default_sint64_mut().get(), eq(46));
    assert_that!(msg.default_sint64_mut().is_set(), eq(false));
    assert_that!(msg.default_sint64_opt(), eq(Optional::Unset(46)));

    msg.default_sint64_mut().set(999);
    assert_that!(msg.default_sint64(), eq(999));
    assert_that!(msg.default_sint64_mut().get(), eq(999));
    assert_that!(msg.default_sint64_mut().is_set(), eq(true));
    assert_that!(msg.default_sint64_opt(), eq(Optional::Set(999)));

    msg.default_sint64_mut().clear();
    assert_that!(msg.default_sint64(), eq(46));
    assert_that!(msg.default_sint64_mut().get(), eq(46));
    assert_that!(msg.default_sint64_mut().is_set(), eq(false));
    assert_that!(msg.default_sint64_opt(), eq(Optional::Unset(46)));

    msg.default_sint64_mut().or_default();
    assert_that!(msg.default_sint64(), eq(46));
    assert_that!(msg.default_sint64_mut().get(), eq(46));
    assert_that!(msg.default_sint64_mut().is_set(), eq(true));
    assert_that!(msg.default_sint64_opt(), eq(Optional::Set(46)));
}

#[test]
fn test_optional_uint32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_uint32(), eq(0));

    msg.optional_uint32_set(Some(9001));
    assert_that!(msg.optional_uint32_opt(), eq(Optional::Set(9001)));
    assert_that!(msg.optional_uint32(), eq(9001));

    msg.optional_uint32_set(None);
    assert_that!(msg.optional_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_uint32(), eq(0));
}

#[test]
fn test_default_uint32_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_uint32(), eq(43));
    assert_that!(msg.default_uint32_mut().get(), eq(43));
    assert_that!(msg.default_uint32_mut().is_set(), eq(false));
    assert_that!(msg.default_uint32_opt(), eq(Optional::Unset(43)));

    msg.default_uint32_mut().set(999);
    assert_that!(msg.default_uint32(), eq(999));
    assert_that!(msg.default_uint32_mut().get(), eq(999));
    assert_that!(msg.default_uint32_mut().is_set(), eq(true));
    assert_that!(msg.default_uint32_opt(), eq(Optional::Set(999)));

    msg.default_uint32_mut().clear();
    assert_that!(msg.default_uint32(), eq(43));
    assert_that!(msg.default_uint32_mut().get(), eq(43));
    assert_that!(msg.default_uint32_mut().is_set(), eq(false));
    assert_that!(msg.default_uint32_opt(), eq(Optional::Unset(43)));

    msg.default_uint32_mut().or_default();
    assert_that!(msg.default_uint32(), eq(43));
    assert_that!(msg.default_uint32_mut().get(), eq(43));
    assert_that!(msg.default_uint32_mut().is_set(), eq(true));
    assert_that!(msg.default_uint32_opt(), eq(Optional::Set(43)));
}

#[test]
fn test_optional_uint64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_uint64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_uint64(), eq(0));

    msg.optional_uint64_set(Some(42));
    assert_that!(msg.optional_uint64_opt(), eq(Optional::Set(42)));
    assert_that!(msg.optional_uint64(), eq(42));

    msg.optional_uint64_set(None);
    assert_that!(msg.optional_uint64_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.optional_uint64(), eq(0));
}

#[test]
fn test_default_uint64_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_uint64(), eq(44));
    assert_that!(msg.default_uint64_mut().get(), eq(44));
    assert_that!(msg.default_uint64_mut().is_set(), eq(false));
    assert_that!(msg.default_uint64_opt(), eq(Optional::Unset(44)));

    msg.default_uint64_mut().set(999);
    assert_that!(msg.default_uint64(), eq(999));
    assert_that!(msg.default_uint64_mut().get(), eq(999));
    assert_that!(msg.default_uint64_mut().is_set(), eq(true));
    assert_that!(msg.default_uint64_opt(), eq(Optional::Set(999)));

    msg.default_uint64_mut().clear();
    assert_that!(msg.default_uint64(), eq(44));
    assert_that!(msg.default_uint64_mut().get(), eq(44));
    assert_that!(msg.default_uint64_mut().is_set(), eq(false));
    assert_that!(msg.default_uint64_opt(), eq(Optional::Unset(44)));

    msg.default_uint64_mut().or_default();
    assert_that!(msg.default_uint64(), eq(44));
    assert_that!(msg.default_uint64_mut().get(), eq(44));
    assert_that!(msg.default_uint64_mut().is_set(), eq(true));
    assert_that!(msg.default_uint64_opt(), eq(Optional::Set(44)));
}

#[test]
fn test_optional_float_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_float_opt(), eq(Optional::Unset(0.0)));
    assert_that!(msg.optional_float(), eq(0.0));

    msg.optional_float_set(Some(std::f32::consts::PI));
    assert_that!(msg.optional_float_opt(), eq(Optional::Set(std::f32::consts::PI)));
    assert_that!(msg.optional_float(), eq(std::f32::consts::PI));

    msg.optional_float_set(None);
    assert_that!(msg.optional_float_opt(), eq(Optional::Unset(0.0)));
    assert_that!(msg.optional_float(), eq(0.0));
}

#[test]
fn test_default_float_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_float(), eq(51.5));
    assert_that!(msg.default_float_mut().get(), eq(51.5));
    assert_that!(msg.default_float_mut().is_set(), eq(false));
    assert_that!(msg.default_float_opt(), eq(Optional::Unset(51.5)));

    msg.default_float_mut().set(999.9);
    assert_that!(msg.default_float(), eq(999.9));
    assert_that!(msg.default_float_mut().get(), eq(999.9));
    assert_that!(msg.default_float_mut().is_set(), eq(true));
    assert_that!(msg.default_float_opt(), eq(Optional::Set(999.9)));

    msg.default_float_mut().clear();
    assert_that!(msg.default_float(), eq(51.5));
    assert_that!(msg.default_float_mut().get(), eq(51.5));
    assert_that!(msg.default_float_mut().is_set(), eq(false));
    assert_that!(msg.default_float_opt(), eq(Optional::Unset(51.5)));

    msg.default_float_mut().or_default();
    assert_that!(msg.default_float(), eq(51.5));
    assert_that!(msg.default_float_mut().get(), eq(51.5));
    assert_that!(msg.default_float_mut().is_set(), eq(true));
    assert_that!(msg.default_float_opt(), eq(Optional::Set(51.5)));
}

#[test]
fn test_optional_double_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_double_opt(), eq(Optional::Unset(0.0)));
    assert_that!(msg.optional_double(), eq(0.0));

    msg.optional_double_set(Some(-10.99));
    assert_that!(msg.optional_double_opt(), eq(Optional::Set(-10.99)));
    assert_that!(msg.optional_double(), eq(-10.99));

    msg.optional_double_set(None);
    assert_that!(msg.optional_double_opt(), eq(Optional::Unset(0.0)));
    assert_that!(msg.optional_double(), eq(0.0));
}

#[test]
fn test_default_double_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_double(), eq(52e3));
    assert_that!(msg.default_double_mut().get(), eq(52e3));
    assert_that!(msg.default_double_mut().is_set(), eq(false));
    assert_that!(msg.default_double_opt(), eq(Optional::Unset(52e3)));

    msg.default_double_mut().set(999.9);
    assert_that!(msg.default_double(), eq(999.9));
    assert_that!(msg.default_double_mut().get(), eq(999.9));
    assert_that!(msg.default_double_mut().is_set(), eq(true));
    assert_that!(msg.default_double_opt(), eq(Optional::Set(999.9)));

    msg.default_double_mut().clear();
    assert_that!(msg.default_double(), eq(52e3));
    assert_that!(msg.default_double_mut().get(), eq(52e3));
    assert_that!(msg.default_double_mut().is_set(), eq(false));
    assert_that!(msg.default_double_opt(), eq(Optional::Unset(52e3)));

    msg.default_double_mut().or_default();
    assert_that!(msg.default_double(), eq(52e3));
    assert_that!(msg.default_double_mut().get(), eq(52e3));
    assert_that!(msg.default_double_mut().is_set(), eq(true));
    assert_that!(msg.default_double_opt(), eq(Optional::Set(52e3)));
}

#[test]
fn test_optional_bool_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_bool_opt(), eq(Optional::Unset(false)));

    msg.optional_bool_set(Some(true));
    assert_that!(msg.optional_bool_opt(), eq(Optional::Set(true)));

    msg.optional_bool_set(None);
    assert_that!(msg.optional_bool_opt(), eq(Optional::Unset(false)));
}

#[test]
fn test_default_bool_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_bool(), eq(true));
    assert_that!(msg.default_bool_mut().get(), eq(true));
    assert_that!(msg.default_bool_mut().is_set(), eq(false));
    assert_that!(msg.default_bool_opt(), eq(Optional::Unset(true)));

    msg.default_bool_mut().set(false);
    assert_that!(msg.default_bool(), eq(false));
    assert_that!(msg.default_bool_mut().get(), eq(false));
    assert_that!(msg.default_bool_mut().is_set(), eq(true));
    assert_that!(msg.default_bool_opt(), eq(Optional::Set(false)));

    msg.default_bool_mut().clear();
    assert_that!(msg.default_bool(), eq(true));
    assert_that!(msg.default_bool_mut().get(), eq(true));
    assert_that!(msg.default_bool_mut().is_set(), eq(false));
    assert_that!(msg.default_bool_opt(), eq(Optional::Unset(true)));

    msg.default_bool_mut().or_default();
    assert_that!(msg.default_bool(), eq(true));
    assert_that!(msg.default_bool_mut().get(), eq(true));
    assert_that!(msg.default_bool_mut().is_set(), eq(true));
    assert_that!(msg.default_bool_opt(), eq(Optional::Set(true)));
}

#[test]
fn test_optional_bytes_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Unset(&b""[..])));
    assert_that!(*msg.optional_bytes_mut().get(), empty());
    assert_that!(msg.optional_bytes_mut(), is_unset());

    {
        let s = Vec::from(&b"hello world"[..]);
        msg.optional_bytes_mut().set(&s[..]);
    }
    assert_that!(msg.optional_bytes(), eq(b"hello world"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"hello world"[..])));
    assert_that!(msg.optional_bytes_mut(), is_set());
    assert_that!(msg.optional_bytes_mut().get(), eq(b"hello world"));

    msg.optional_bytes_mut().or_default().set(b"accessors_test");
    assert_that!(msg.optional_bytes(), eq(b"accessors_test"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"accessors_test"[..])));
    assert_that!(msg.optional_bytes_mut(), is_set());
    assert_that!(msg.optional_bytes_mut().get(), eq(b"accessors_test"));
    assert_that!(msg.optional_bytes_mut().or_default().get(), eq(b"accessors_test"));

    msg.optional_bytes_mut().clear();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Unset(&b""[..])));
    assert_that!(msg.optional_bytes_mut(), is_unset());

    msg.optional_bytes_mut().set(b"");
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b""[..])));

    msg.optional_bytes_mut().clear();
    msg.optional_bytes_mut().or_default();
    assert_that!(*msg.optional_bytes(), empty());
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b""[..])));

    msg.optional_bytes_mut().or_default().set(b"\xffbinary\x85non-utf8");
    assert_that!(msg.optional_bytes(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.optional_bytes_opt(), eq(Optional::Set(&b"\xffbinary\x85non-utf8"[..])));
    assert_that!(msg.optional_bytes_mut(), is_set());
    assert_that!(msg.optional_bytes_mut().get(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.optional_bytes_mut().or_default().get(), eq(b"\xffbinary\x85non-utf8"));
}

#[test]
fn test_nonempty_default_bytes_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_bytes(), eq(b"world"));
    assert_that!(msg.default_bytes_opt(), eq(Optional::Unset(&b"world"[..])));
    assert_that!(msg.default_bytes_mut().get(), eq(b"world"));
    assert_that!(msg.default_bytes_mut(), is_unset());

    {
        let s = String::from("hello world");
        msg.default_bytes_mut().set(s.as_bytes());
    }
    assert_that!(msg.default_bytes(), eq(b"hello world"));
    assert_that!(msg.default_bytes_opt(), eq(Optional::Set(&b"hello world"[..])));
    assert_that!(msg.default_bytes_mut(), is_set());
    assert_that!(msg.default_bytes_mut().get(), eq(b"hello world"));

    msg.default_bytes_mut().or_default().set(b"accessors_test");
    assert_that!(msg.default_bytes(), eq(b"accessors_test"));
    assert_that!(msg.default_bytes_opt(), eq(Optional::Set(&b"accessors_test"[..])));
    assert_that!(msg.default_bytes_mut(), is_set());
    assert_that!(msg.default_bytes_mut().get(), eq(b"accessors_test"));
    assert_that!(msg.default_bytes_mut().or_default().get(), eq(b"accessors_test"));

    msg.default_bytes_mut().clear();
    assert_that!(msg.default_bytes(), eq(b"world"));
    assert_that!(msg.default_bytes_opt(), eq(Optional::Unset(&b"world"[..])));
    assert_that!(msg.default_bytes_mut(), is_unset());

    msg.default_bytes_mut().set(b"");
    assert_that!(*msg.default_bytes(), empty());
    assert_that!(msg.default_bytes_opt(), eq(Optional::Set(&b""[..])));

    msg.default_bytes_mut().clear();
    msg.default_bytes_mut().or_default();
    assert_that!(msg.default_bytes(), eq(b"world"));
    assert_that!(msg.default_bytes_opt(), eq(Optional::Set(&b"world"[..])));

    msg.default_bytes_mut().or_default().set(b"\xffbinary\x85non-utf8");
    assert_that!(msg.default_bytes(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.default_bytes_opt(), eq(Optional::Set(&b"\xffbinary\x85non-utf8"[..])));
    assert_that!(msg.default_bytes_mut(), is_set());
    assert_that!(msg.default_bytes_mut().get(), eq(b"\xffbinary\x85non-utf8"));
    assert_that!(msg.default_bytes_mut().or_default().get(), eq(b"\xffbinary\x85non-utf8"));
}

#[test]
fn test_optional_string_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.optional_string(), eq(""));
    assert_that!(msg.optional_string_opt(), eq(Optional::Unset("".into())));
    assert_that!(msg.optional_string_mut().get(), eq(""));
    assert_that!(msg.optional_string_mut(), is_unset());

    {
        let s = String::from("hello world");
        msg.optional_string_mut().set(&s[..]);
    }
    assert_that!(msg.optional_string(), eq("hello world"));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("hello world".into())));
    assert_that!(msg.optional_string_mut(), is_set());
    assert_that!(msg.optional_string_mut().get(), eq("hello world"));

    msg.optional_string_mut().or_default().set("accessors_test");
    assert_that!(msg.optional_string(), eq("accessors_test"));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("accessors_test".into())));
    assert_that!(msg.optional_string_mut(), is_set());
    assert_that!(msg.optional_string_mut().get(), eq("accessors_test"));
    assert_that!(msg.optional_string_mut().or_default().get(), eq("accessors_test"));

    msg.optional_string_mut().clear();
    assert_that!(msg.optional_string(), eq(""));
    assert_that!(msg.optional_string_opt(), eq(Optional::Unset("".into())));
    assert_that!(msg.optional_string_mut(), is_unset());

    msg.optional_string_mut().set("");
    assert_that!(msg.optional_string(), eq(""));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("".into())));

    msg.optional_string_mut().clear();
    msg.optional_string_mut().or_default();
    assert_that!(msg.optional_string(), eq(""));
    assert_that!(msg.optional_string_opt(), eq(Optional::Set("".into())));
}

#[test]
fn test_nonempty_default_string_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.default_string(), eq("hello"));
    assert_that!(msg.default_string_opt(), eq(Optional::Unset("hello".into())));
    assert_that!(msg.default_string_mut().get(), eq("hello"));
    assert_that!(msg.default_string_mut(), is_unset());

    {
        let s = String::from("hello world");
        msg.default_string_mut().set(&s[..]);
    }
    assert_that!(msg.default_string(), eq("hello world"));
    assert_that!(msg.default_string_opt(), eq(Optional::Set("hello world".into())));
    assert_that!(msg.default_string_mut(), is_set());
    assert_that!(msg.default_string_mut().get(), eq("hello world"));

    msg.default_string_mut().or_default().set("accessors_test");
    assert_that!(msg.default_string(), eq("accessors_test"));
    assert_that!(msg.default_string_opt(), eq(Optional::Set("accessors_test".into())));
    assert_that!(msg.default_string_mut(), is_set());
    assert_that!(msg.default_string_mut().get(), eq("accessors_test"));
    assert_that!(msg.default_string_mut().or_default().get(), eq("accessors_test"));

    msg.default_string_mut().clear();
    assert_that!(msg.default_string(), eq("hello"));
    assert_that!(msg.default_string_opt(), eq(Optional::Unset("hello".into())));
    assert_that!(msg.default_string_mut(), is_unset());

    msg.default_string_mut().set("");
    assert_that!(msg.default_string(), eq(""));
    assert_that!(msg.default_string_opt(), eq(Optional::Set("".into())));

    msg.default_string_mut().clear();
    msg.default_string_mut().or_default();
    assert_that!(msg.default_string(), eq("hello"));
    assert_that!(msg.default_string_opt(), eq(Optional::Set("hello".into())));
}

#[test]
fn test_singular_msg_field() {
    use crate::TestAllTypes_::NestedMessageView;
    let msg = TestAllTypes::new();
    // TODO: fetch the inner integer `bb`
    // call should look like msg.optional_nested_message().bb()
    let _msg: NestedMessageView = msg.optional_nested_message();
}

#[test]
fn test_oneof_accessors() {
    use TestAllTypes_::OneofField::*;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_set(Some(7));
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Set(7)));
    assert_that!(msg.oneof_field(), matches_pattern!(OneofUint32(eq(7))));

    msg.oneof_uint32_set(None);
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Unset(0)));
    assert_that!(msg.oneof_field(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_set(Some(7));
    msg.oneof_bytes_mut().set(b"123");
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Unset(0)));

    assert_that!(msg.oneof_field(), matches_pattern!(OneofBytes(eq(b"123"))));
}

#[test]
fn test_oneof_mut_accessors() {
    use TestAllTypes_::OneofFieldMut::*;

    let mut msg = TestAllTypes::new();
    assert_that!(msg.oneof_field_mut(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_set(Some(7));

    match msg.oneof_field_mut() {
        OneofUint32(mut v) => {
            assert_eq!(v.get(), 7);
            v.set(8);
            assert_eq!(v.get(), 8);
        }
        f => panic!("unexpected field_mut type! {:?}", f),
    }

    // Confirm that the mut write above applies to both the field accessor and the
    // oneof view accessor.
    assert_that!(msg.oneof_uint32_opt(), eq(Optional::Set(8)));
    assert_that!(
        msg.oneof_field(),
        matches_pattern!(TestAllTypes_::OneofField::OneofUint32(eq(8)))
    );

    msg.oneof_uint32_set(None);
    assert_that!(msg.oneof_field_mut(), matches_pattern!(not_set(_)));

    msg.oneof_uint32_set(Some(7));
    msg.oneof_bytes_mut().set(b"123");
    assert_that!(msg.oneof_field_mut(), matches_pattern!(OneofBytes(_)));
}

macro_rules! generate_repeated_numeric_test {
    ($(($t: ty, $field: ident)),*) => {
        paste! { $(
            #[test]
            fn [< test_repeated_ $field _accessors >]() {
                let mut msg = TestAllTypes::new();
                assert_that!(msg.[< repeated_ $field >]().len(), eq(0));
                assert_that!(msg.[<repeated_ $field >]().get(0), none());

                let mut mutator = msg.[<repeated_ $field _mut >]();
                mutator.push(1 as $t);
                assert_that!(mutator.len(), eq(1));
                assert_that!(mutator.get(0), some(eq(1 as $t)));
                mutator.set(0, 2 as $t);
                assert_that!(mutator.get(0), some(eq(2 as $t)));
                mutator.push(1 as $t);

                mutator.push(3 as $t);
                assert_that!(mutator.get_mut(2).is_some(), eq(true));
                let mut mut_elem = mutator.get_mut(2).unwrap();
                mut_elem.set(4 as $t);
                assert_that!(mut_elem.get(), eq(4 as $t));
                mut_elem.clear();
                assert_that!(mut_elem.get(), eq(0 as $t));

                assert_that!(
                    mutator.iter().collect::<Vec<_>>(),
                    eq(vec![2 as $t, 1 as $t, 0 as $t])
                );
                assert_that!(
                    (*mutator).into_iter().collect::<Vec<_>>(),
                    eq(vec![2 as $t, 1 as $t, 0 as $t])
                );

                for mut mutable_elem in msg.[<repeated_ $field _mut >]() {
                    mutable_elem.set(0 as $t);
                }
                assert_that!(
                    msg.[<repeated_ $field _mut >]().iter().all(|v| v == (0 as $t)),
                    eq(true)
                );
            }

            #[test]
            fn [< test_repeated_ $field _set >]() {
                let mut msg = TestAllTypes::new();
                let mut mutator = msg.[<repeated_ $field _mut>]();
                let mut msg2 = TestAllTypes::new();
                let mut mutator2 = msg2.[<repeated_ $field _mut>]();
                for i in 0..5 {
                    mutator2.push(i as $t);
                }
                protobuf::MutProxy::set(&mut mutator, *mutator2);

                assert_that!(
                    mutator.iter().collect::<Vec<_>>(),
                    eq(mutator2.iter().collect::<Vec<_>>())
                );
            }
        )* }
    };
}

generate_repeated_numeric_test!(
    (i32, int32),
    (u32, uint32),
    (i64, int64),
    (u64, uint64),
    (f32, float),
    (f64, double)
);

#[test]
fn test_repeated_bool_accessors() {
    let mut msg = TestAllTypes::new();
    assert_that!(msg.repeated_bool().len(), eq(0));
    assert_that!(msg.repeated_bool().get(0), none());

    let mut mutator = msg.repeated_bool_mut();
    mutator.push(true);
    assert_that!(mutator.len(), eq(1));
    assert_that!(mutator.get(0), some(eq(true)));
    mutator.set(0, false);
    assert_that!(mutator.get(0), some(eq(false)));
    mutator.push(true);

    mutator.push(false);
    assert_that!(mutator.get_mut(2), pat!(Some(_)));
    let mut mut_elem = mutator.get_mut(2).unwrap();
    mut_elem.set(true);
    assert_that!(mut_elem.get(), eq(true));
    mut_elem.clear();
    assert_that!(mut_elem.get(), eq(false));

    assert_that!(mutator.iter().collect::<Vec<_>>(), eq(vec![false, true, false]));
    assert_that!((*mutator).into_iter().collect::<Vec<_>>(), eq(vec![false, true, false]));

    for mut mutable_elem in msg.repeated_bool_mut() {
        mutable_elem.set(false);
    }
    assert_that!(msg.repeated_bool().iter().all(|v| v), eq(false));
}

#[test]
fn test_repeated_bool_set() {
    let mut msg = TestAllTypes::new();
    let mut mutator = msg.repeated_bool_mut();
    let mut msg2 = TestAllTypes::new();
    let mut mutator2 = msg2.repeated_bool_mut();
    for _ in 0..5 {
        mutator2.push(true);
    }
    protobuf::MutProxy::set(&mut mutator, *mutator2);

    assert_that!(mutator.iter().collect::<Vec<_>>(), eq(mutator2.iter().collect::<Vec<_>>()));
}

#[test]
fn test_repeated_message() {
    use protobuf::ViewProxy;
    let mut msg = TestAllTypes::new();
    msg.repeated_nested_message_mut().push_default();
    msg.repeated_nested_message_mut()
        .set(0, TestAllTypes::new().repeated_nested_message_mut().push_default().as_view());
    msg.repeated_nested_message_mut()
        .push(TestAllTypes::new().repeated_nested_message_mut().push_default().as_view());

    assert_that!(msg.repeated_nested_message().len(), eq(2));
}
