// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `number_type_option.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct NumberTypeOption {
    // message fields
    pub format: super::format::NumberFormat,
    pub scale: u32,
    pub symbol: ::std::string::String,
    pub sign_positive: bool,
    pub name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NumberTypeOption {
    fn default() -> &'a NumberTypeOption {
        <NumberTypeOption as ::protobuf::Message>::default_instance()
    }
}

impl NumberTypeOption {
    pub fn new() -> NumberTypeOption {
        ::std::default::Default::default()
    }

    // .NumberFormat format = 1;


    pub fn get_format(&self) -> super::format::NumberFormat {
        self.format
    }
    pub fn clear_format(&mut self) {
        self.format = super::format::NumberFormat::Number;
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: super::format::NumberFormat) {
        self.format = v;
    }

    // uint32 scale = 2;


    pub fn get_scale(&self) -> u32 {
        self.scale
    }
    pub fn clear_scale(&mut self) {
        self.scale = 0;
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: u32) {
        self.scale = v;
    }

    // string symbol = 3;


    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }
    pub fn clear_symbol(&mut self) {
        self.symbol.clear();
    }

    // Param is passed by value, moved
    pub fn set_symbol(&mut self, v: ::std::string::String) {
        self.symbol = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symbol(&mut self) -> &mut ::std::string::String {
        &mut self.symbol
    }

    // Take field
    pub fn take_symbol(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.symbol, ::std::string::String::new())
    }

    // bool sign_positive = 4;


    pub fn get_sign_positive(&self) -> bool {
        self.sign_positive
    }
    pub fn clear_sign_positive(&mut self) {
        self.sign_positive = false;
    }

    // Param is passed by value, moved
    pub fn set_sign_positive(&mut self, v: bool) {
        self.sign_positive = v;
    }

    // string name = 5;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for NumberTypeOption {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.format, 1, &mut self.unknown_fields)?
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scale = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.symbol)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.sign_positive = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.format != super::format::NumberFormat::Number {
            my_size += ::protobuf::rt::enum_size(1, self.format);
        }
        if self.scale != 0 {
            my_size += ::protobuf::rt::value_size(2, self.scale, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.symbol.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.symbol);
        }
        if self.sign_positive != false {
            my_size += 2;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.format != super::format::NumberFormat::Number {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.format))?;
        }
        if self.scale != 0 {
            os.write_uint32(2, self.scale)?;
        }
        if !self.symbol.is_empty() {
            os.write_string(3, &self.symbol)?;
        }
        if self.sign_positive != false {
            os.write_bool(4, self.sign_positive)?;
        }
        if !self.name.is_empty() {
            os.write_string(5, &self.name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NumberTypeOption {
        NumberTypeOption::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::format::NumberFormat>>(
                "format",
                |m: &NumberTypeOption| { &m.format },
                |m: &mut NumberTypeOption| { &mut m.format },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "scale",
                |m: &NumberTypeOption| { &m.scale },
                |m: &mut NumberTypeOption| { &mut m.scale },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "symbol",
                |m: &NumberTypeOption| { &m.symbol },
                |m: &mut NumberTypeOption| { &mut m.symbol },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "sign_positive",
                |m: &NumberTypeOption| { &m.sign_positive },
                |m: &mut NumberTypeOption| { &mut m.sign_positive },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &NumberTypeOption| { &m.name },
                |m: &mut NumberTypeOption| { &mut m.name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<NumberTypeOption>(
                "NumberTypeOption",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static NumberTypeOption {
        static instance: ::protobuf::rt::LazyV2<NumberTypeOption> = ::protobuf::rt::LazyV2::INIT;
        instance.get(NumberTypeOption::new)
    }
}

impl ::protobuf::Clear for NumberTypeOption {
    fn clear(&mut self) {
        self.format = super::format::NumberFormat::Number;
        self.scale = 0;
        self.symbol.clear();
        self.sign_positive = false;
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NumberTypeOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NumberTypeOption {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18number_type_option.proto\x1a\x0cformat.proto\"\xa0\x01\n\x10Number\
    TypeOption\x12%\n\x06format\x18\x01\x20\x01(\x0e2\r.NumberFormatR\x06for\
    mat\x12\x14\n\x05scale\x18\x02\x20\x01(\rR\x05scale\x12\x16\n\x06symbol\
    \x18\x03\x20\x01(\tR\x06symbol\x12#\n\rsign_positive\x18\x04\x20\x01(\
    \x08R\x0csignPositive\x12\x12\n\x04name\x18\x05\x20\x01(\tR\x04nameb\x06\
    proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
