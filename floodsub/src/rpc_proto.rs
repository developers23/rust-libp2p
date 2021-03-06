// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RPC {
    // message fields
    subscriptions: ::protobuf::RepeatedField<RPC_SubOpts>,
    publish: ::protobuf::RepeatedField<Message>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RPC {}

impl RPC {
    pub fn new() -> RPC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RPC {
        static mut instance: ::protobuf::lazy::Lazy<RPC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RPC,
        };
        unsafe {
            instance.get(RPC::new)
        }
    }

    // repeated .floodsub.pb.RPC.SubOpts subscriptions = 1;

    pub fn clear_subscriptions(&mut self) {
        self.subscriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_subscriptions(&mut self, v: ::protobuf::RepeatedField<RPC_SubOpts>) {
        self.subscriptions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subscriptions(&mut self) -> &mut ::protobuf::RepeatedField<RPC_SubOpts> {
        &mut self.subscriptions
    }

    // Take field
    pub fn take_subscriptions(&mut self) -> ::protobuf::RepeatedField<RPC_SubOpts> {
        ::std::mem::replace(&mut self.subscriptions, ::protobuf::RepeatedField::new())
    }

    pub fn get_subscriptions(&self) -> &[RPC_SubOpts] {
        &self.subscriptions
    }

    fn get_subscriptions_for_reflect(&self) -> &::protobuf::RepeatedField<RPC_SubOpts> {
        &self.subscriptions
    }

    fn mut_subscriptions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RPC_SubOpts> {
        &mut self.subscriptions
    }

    // repeated .floodsub.pb.Message publish = 2;

    pub fn clear_publish(&mut self) {
        self.publish.clear();
    }

    // Param is passed by value, moved
    pub fn set_publish(&mut self, v: ::protobuf::RepeatedField<Message>) {
        self.publish = v;
    }

    // Mutable pointer to the field.
    pub fn mut_publish(&mut self) -> &mut ::protobuf::RepeatedField<Message> {
        &mut self.publish
    }

    // Take field
    pub fn take_publish(&mut self) -> ::protobuf::RepeatedField<Message> {
        ::std::mem::replace(&mut self.publish, ::protobuf::RepeatedField::new())
    }

    pub fn get_publish(&self) -> &[Message] {
        &self.publish
    }

    fn get_publish_for_reflect(&self) -> &::protobuf::RepeatedField<Message> {
        &self.publish
    }

    fn mut_publish_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Message> {
        &mut self.publish
    }
}

impl ::protobuf::Message for RPC {
    fn is_initialized(&self) -> bool {
        for v in &self.subscriptions {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.publish {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.subscriptions)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.publish)?;
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
        for value in &self.subscriptions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.publish {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.subscriptions {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.publish {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RPC {
    fn new() -> RPC {
        RPC::new()
    }

    fn descriptor_static(_: ::std::option::Option<RPC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RPC_SubOpts>>(
                    "subscriptions",
                    RPC::get_subscriptions_for_reflect,
                    RPC::mut_subscriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message>>(
                    "publish",
                    RPC::get_publish_for_reflect,
                    RPC::mut_publish_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RPC>(
                    "RPC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RPC {
    fn clear(&mut self) {
        self.clear_subscriptions();
        self.clear_publish();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RPC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RPC {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RPC_SubOpts {
    // message fields
    subscribe: ::std::option::Option<bool>,
    topicid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RPC_SubOpts {}

impl RPC_SubOpts {
    pub fn new() -> RPC_SubOpts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RPC_SubOpts {
        static mut instance: ::protobuf::lazy::Lazy<RPC_SubOpts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RPC_SubOpts,
        };
        unsafe {
            instance.get(RPC_SubOpts::new)
        }
    }

    // optional bool subscribe = 1;

    pub fn clear_subscribe(&mut self) {
        self.subscribe = ::std::option::Option::None;
    }

    pub fn has_subscribe(&self) -> bool {
        self.subscribe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscribe(&mut self, v: bool) {
        self.subscribe = ::std::option::Option::Some(v);
    }

    pub fn get_subscribe(&self) -> bool {
        self.subscribe.unwrap_or(false)
    }

    fn get_subscribe_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.subscribe
    }

    fn mut_subscribe_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.subscribe
    }

    // optional string topicid = 2;

    pub fn clear_topicid(&mut self) {
        self.topicid.clear();
    }

    pub fn has_topicid(&self) -> bool {
        self.topicid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topicid(&mut self, v: ::std::string::String) {
        self.topicid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topicid(&mut self) -> &mut ::std::string::String {
        if self.topicid.is_none() {
            self.topicid.set_default();
        }
        self.topicid.as_mut().unwrap()
    }

    // Take field
    pub fn take_topicid(&mut self) -> ::std::string::String {
        self.topicid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_topicid(&self) -> &str {
        match self.topicid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_topicid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.topicid
    }

    fn mut_topicid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.topicid
    }
}

impl ::protobuf::Message for RPC_SubOpts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.subscribe = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.topicid)?;
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
        if let Some(v) = self.subscribe {
            my_size += 2;
        }
        if let Some(ref v) = self.topicid.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.subscribe {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.topicid.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RPC_SubOpts {
    fn new() -> RPC_SubOpts {
        RPC_SubOpts::new()
    }

    fn descriptor_static(_: ::std::option::Option<RPC_SubOpts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "subscribe",
                    RPC_SubOpts::get_subscribe_for_reflect,
                    RPC_SubOpts::mut_subscribe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "topicid",
                    RPC_SubOpts::get_topicid_for_reflect,
                    RPC_SubOpts::mut_topicid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RPC_SubOpts>(
                    "RPC_SubOpts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RPC_SubOpts {
    fn clear(&mut self) {
        self.clear_subscribe();
        self.clear_topicid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RPC_SubOpts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RPC_SubOpts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    from: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    seqno: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    topicIDs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // optional bytes from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: ::std::vec::Vec<u8>) {
        self.from = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.from.is_none() {
            self.from.set_default();
        }
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> ::std::vec::Vec<u8> {
        self.from.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_from(&self) -> &[u8] {
        match self.from.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_from_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.from
    }

    fn mut_from_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.from
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }

    // optional bytes seqno = 3;

    pub fn clear_seqno(&mut self) {
        self.seqno.clear();
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: ::std::vec::Vec<u8>) {
        self.seqno = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_seqno(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.seqno.is_none() {
            self.seqno.set_default();
        }
        self.seqno.as_mut().unwrap()
    }

    // Take field
    pub fn take_seqno(&mut self) -> ::std::vec::Vec<u8> {
        self.seqno.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_seqno(&self) -> &[u8] {
        match self.seqno.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_seqno_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.seqno
    }

    // repeated string topicIDs = 4;

    pub fn clear_topicIDs(&mut self) {
        self.topicIDs.clear();
    }

    // Param is passed by value, moved
    pub fn set_topicIDs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.topicIDs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_topicIDs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.topicIDs
    }

    // Take field
    pub fn take_topicIDs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.topicIDs, ::protobuf::RepeatedField::new())
    }

    pub fn get_topicIDs(&self) -> &[::std::string::String] {
        &self.topicIDs
    }

    fn get_topicIDs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.topicIDs
    }

    fn mut_topicIDs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.topicIDs
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.from)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.seqno)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.topicIDs)?;
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
        if let Some(ref v) = self.from.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.seqno.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        for value in &self.topicIDs {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.from.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.seqno.as_ref() {
            os.write_bytes(3, &v)?;
        }
        for v in &self.topicIDs {
            os.write_string(4, &v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "from",
                    Message::get_from_for_reflect,
                    Message::mut_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Message::get_data_for_reflect,
                    Message::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "seqno",
                    Message::get_seqno_for_reflect,
                    Message::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "topicIDs",
                    Message::get_topicIDs_for_reflect,
                    Message::mut_topicIDs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_from();
        self.clear_data();
        self.clear_seqno();
        self.clear_topicIDs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopicDescriptor {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    auth: ::protobuf::SingularPtrField<TopicDescriptor_AuthOpts>,
    enc: ::protobuf::SingularPtrField<TopicDescriptor_EncOpts>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopicDescriptor {}

impl TopicDescriptor {
    pub fn new() -> TopicDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopicDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<TopicDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopicDescriptor,
        };
        unsafe {
            instance.get(TopicDescriptor::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .floodsub.pb.TopicDescriptor.AuthOpts auth = 2;

    pub fn clear_auth(&mut self) {
        self.auth.clear();
    }

    pub fn has_auth(&self) -> bool {
        self.auth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth(&mut self, v: TopicDescriptor_AuthOpts) {
        self.auth = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth(&mut self) -> &mut TopicDescriptor_AuthOpts {
        if self.auth.is_none() {
            self.auth.set_default();
        }
        self.auth.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth(&mut self) -> TopicDescriptor_AuthOpts {
        self.auth.take().unwrap_or_else(|| TopicDescriptor_AuthOpts::new())
    }

    pub fn get_auth(&self) -> &TopicDescriptor_AuthOpts {
        self.auth.as_ref().unwrap_or_else(|| TopicDescriptor_AuthOpts::default_instance())
    }

    fn get_auth_for_reflect(&self) -> &::protobuf::SingularPtrField<TopicDescriptor_AuthOpts> {
        &self.auth
    }

    fn mut_auth_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TopicDescriptor_AuthOpts> {
        &mut self.auth
    }

    // optional .floodsub.pb.TopicDescriptor.EncOpts enc = 3;

    pub fn clear_enc(&mut self) {
        self.enc.clear();
    }

    pub fn has_enc(&self) -> bool {
        self.enc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enc(&mut self, v: TopicDescriptor_EncOpts) {
        self.enc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_enc(&mut self) -> &mut TopicDescriptor_EncOpts {
        if self.enc.is_none() {
            self.enc.set_default();
        }
        self.enc.as_mut().unwrap()
    }

    // Take field
    pub fn take_enc(&mut self) -> TopicDescriptor_EncOpts {
        self.enc.take().unwrap_or_else(|| TopicDescriptor_EncOpts::new())
    }

    pub fn get_enc(&self) -> &TopicDescriptor_EncOpts {
        self.enc.as_ref().unwrap_or_else(|| TopicDescriptor_EncOpts::default_instance())
    }

    fn get_enc_for_reflect(&self) -> &::protobuf::SingularPtrField<TopicDescriptor_EncOpts> {
        &self.enc
    }

    fn mut_enc_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TopicDescriptor_EncOpts> {
        &mut self.enc
    }
}

impl ::protobuf::Message for TopicDescriptor {
    fn is_initialized(&self) -> bool {
        for v in &self.auth {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enc {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.enc)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.auth.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.enc.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.auth.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.enc.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopicDescriptor {
    fn new() -> TopicDescriptor {
        TopicDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopicDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    TopicDescriptor::get_name_for_reflect,
                    TopicDescriptor::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TopicDescriptor_AuthOpts>>(
                    "auth",
                    TopicDescriptor::get_auth_for_reflect,
                    TopicDescriptor::mut_auth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TopicDescriptor_EncOpts>>(
                    "enc",
                    TopicDescriptor::get_enc_for_reflect,
                    TopicDescriptor::mut_enc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopicDescriptor>(
                    "TopicDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopicDescriptor {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_auth();
        self.clear_enc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopicDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopicDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopicDescriptor_AuthOpts {
    // message fields
    mode: ::std::option::Option<TopicDescriptor_AuthOpts_AuthMode>,
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopicDescriptor_AuthOpts {}

impl TopicDescriptor_AuthOpts {
    pub fn new() -> TopicDescriptor_AuthOpts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopicDescriptor_AuthOpts {
        static mut instance: ::protobuf::lazy::Lazy<TopicDescriptor_AuthOpts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopicDescriptor_AuthOpts,
        };
        unsafe {
            instance.get(TopicDescriptor_AuthOpts::new)
        }
    }

    // optional .floodsub.pb.TopicDescriptor.AuthOpts.AuthMode mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: TopicDescriptor_AuthOpts_AuthMode) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> TopicDescriptor_AuthOpts_AuthMode {
        self.mode.unwrap_or(TopicDescriptor_AuthOpts_AuthMode::NONE)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<TopicDescriptor_AuthOpts_AuthMode> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<TopicDescriptor_AuthOpts_AuthMode> {
        &mut self.mode
    }

    // repeated bytes keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }
}

impl ::protobuf::Message for TopicDescriptor_AuthOpts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.mode, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
            os.write_enum(1, v.value())?;
        }
        for v in &self.keys {
            os.write_bytes(2, &v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopicDescriptor_AuthOpts {
    fn new() -> TopicDescriptor_AuthOpts {
        TopicDescriptor_AuthOpts::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopicDescriptor_AuthOpts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TopicDescriptor_AuthOpts_AuthMode>>(
                    "mode",
                    TopicDescriptor_AuthOpts::get_mode_for_reflect,
                    TopicDescriptor_AuthOpts::mut_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    TopicDescriptor_AuthOpts::get_keys_for_reflect,
                    TopicDescriptor_AuthOpts::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopicDescriptor_AuthOpts>(
                    "TopicDescriptor_AuthOpts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopicDescriptor_AuthOpts {
    fn clear(&mut self) {
        self.clear_mode();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopicDescriptor_AuthOpts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopicDescriptor_AuthOpts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TopicDescriptor_AuthOpts_AuthMode {
    NONE = 0,
    KEY = 1,
    WOT = 2,
}

impl ::protobuf::ProtobufEnum for TopicDescriptor_AuthOpts_AuthMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TopicDescriptor_AuthOpts_AuthMode> {
        match value {
            0 => ::std::option::Option::Some(TopicDescriptor_AuthOpts_AuthMode::NONE),
            1 => ::std::option::Option::Some(TopicDescriptor_AuthOpts_AuthMode::KEY),
            2 => ::std::option::Option::Some(TopicDescriptor_AuthOpts_AuthMode::WOT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TopicDescriptor_AuthOpts_AuthMode] = &[
            TopicDescriptor_AuthOpts_AuthMode::NONE,
            TopicDescriptor_AuthOpts_AuthMode::KEY,
            TopicDescriptor_AuthOpts_AuthMode::WOT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<TopicDescriptor_AuthOpts_AuthMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TopicDescriptor_AuthOpts_AuthMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TopicDescriptor_AuthOpts_AuthMode {
}

impl ::protobuf::reflect::ProtobufValue for TopicDescriptor_AuthOpts_AuthMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TopicDescriptor_EncOpts {
    // message fields
    mode: ::std::option::Option<TopicDescriptor_EncOpts_EncMode>,
    keyHashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopicDescriptor_EncOpts {}

impl TopicDescriptor_EncOpts {
    pub fn new() -> TopicDescriptor_EncOpts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopicDescriptor_EncOpts {
        static mut instance: ::protobuf::lazy::Lazy<TopicDescriptor_EncOpts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopicDescriptor_EncOpts,
        };
        unsafe {
            instance.get(TopicDescriptor_EncOpts::new)
        }
    }

    // optional .floodsub.pb.TopicDescriptor.EncOpts.EncMode mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: TopicDescriptor_EncOpts_EncMode) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> TopicDescriptor_EncOpts_EncMode {
        self.mode.unwrap_or(TopicDescriptor_EncOpts_EncMode::NONE)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<TopicDescriptor_EncOpts_EncMode> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<TopicDescriptor_EncOpts_EncMode> {
        &mut self.mode
    }

    // repeated bytes keyHashes = 2;

    pub fn clear_keyHashes(&mut self) {
        self.keyHashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_keyHashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keyHashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keyHashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keyHashes
    }

    // Take field
    pub fn take_keyHashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keyHashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_keyHashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.keyHashes
    }

    fn get_keyHashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keyHashes
    }

    fn mut_keyHashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keyHashes
    }
}

impl ::protobuf::Message for TopicDescriptor_EncOpts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.mode, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keyHashes)?;
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        for value in &self.keyHashes {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
            os.write_enum(1, v.value())?;
        }
        for v in &self.keyHashes {
            os.write_bytes(2, &v)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopicDescriptor_EncOpts {
    fn new() -> TopicDescriptor_EncOpts {
        TopicDescriptor_EncOpts::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopicDescriptor_EncOpts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TopicDescriptor_EncOpts_EncMode>>(
                    "mode",
                    TopicDescriptor_EncOpts::get_mode_for_reflect,
                    TopicDescriptor_EncOpts::mut_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keyHashes",
                    TopicDescriptor_EncOpts::get_keyHashes_for_reflect,
                    TopicDescriptor_EncOpts::mut_keyHashes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopicDescriptor_EncOpts>(
                    "TopicDescriptor_EncOpts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopicDescriptor_EncOpts {
    fn clear(&mut self) {
        self.clear_mode();
        self.clear_keyHashes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopicDescriptor_EncOpts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopicDescriptor_EncOpts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TopicDescriptor_EncOpts_EncMode {
    NONE = 0,
    SHAREDKEY = 1,
    WOT = 2,
}

impl ::protobuf::ProtobufEnum for TopicDescriptor_EncOpts_EncMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TopicDescriptor_EncOpts_EncMode> {
        match value {
            0 => ::std::option::Option::Some(TopicDescriptor_EncOpts_EncMode::NONE),
            1 => ::std::option::Option::Some(TopicDescriptor_EncOpts_EncMode::SHAREDKEY),
            2 => ::std::option::Option::Some(TopicDescriptor_EncOpts_EncMode::WOT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TopicDescriptor_EncOpts_EncMode] = &[
            TopicDescriptor_EncOpts_EncMode::NONE,
            TopicDescriptor_EncOpts_EncMode::SHAREDKEY,
            TopicDescriptor_EncOpts_EncMode::WOT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<TopicDescriptor_EncOpts_EncMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TopicDescriptor_EncOpts_EncMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TopicDescriptor_EncOpts_EncMode {
}

impl ::protobuf::reflect::ProtobufValue for TopicDescriptor_EncOpts_EncMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\trpc.proto\x12\x0bfloodsub.pb\"\xb8\x01\n\x03RPC\x12>\n\rsubscription\
    s\x18\x01\x20\x03(\x0b2\x18.floodsub.pb.RPC.SubOptsR\rsubscriptions\x12.\
    \n\x07publish\x18\x02\x20\x03(\x0b2\x14.floodsub.pb.MessageR\x07publish\
    \x1aA\n\x07SubOpts\x12\x1c\n\tsubscribe\x18\x01\x20\x01(\x08R\tsubscribe\
    \x12\x18\n\x07topicid\x18\x02\x20\x01(\tR\x07topicid\"c\n\x07Message\x12\
    \x12\n\x04from\x18\x01\x20\x01(\x0cR\x04from\x12\x12\n\x04data\x18\x02\
    \x20\x01(\x0cR\x04data\x12\x14\n\x05seqno\x18\x03\x20\x01(\x0cR\x05seqno\
    \x12\x1a\n\x08topicIDs\x18\x04\x20\x03(\tR\x08topicIDs\"\xbe\x03\n\x0fTo\
    picDescriptor\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x129\n\x04au\
    th\x18\x02\x20\x01(\x0b2%.floodsub.pb.TopicDescriptor.AuthOptsR\x04auth\
    \x126\n\x03enc\x18\x03\x20\x01(\x0b2$.floodsub.pb.TopicDescriptor.EncOpt\
    sR\x03enc\x1a\x8a\x01\n\x08AuthOpts\x12B\n\x04mode\x18\x01\x20\x01(\x0e2\
    ..floodsub.pb.TopicDescriptor.AuthOpts.AuthModeR\x04mode\x12\x12\n\x04ke\
    ys\x18\x02\x20\x03(\x0cR\x04keys\"&\n\x08AuthMode\x12\x08\n\x04NONE\x10\
    \0\x12\x07\n\x03KEY\x10\x01\x12\x07\n\x03WOT\x10\x02\x1a\x96\x01\n\x07En\
    cOpts\x12@\n\x04mode\x18\x01\x20\x01(\x0e2,.floodsub.pb.TopicDescriptor.\
    EncOpts.EncModeR\x04mode\x12\x1c\n\tkeyHashes\x18\x02\x20\x03(\x0cR\tkey\
    Hashes\"+\n\x07EncMode\x12\x08\n\x04NONE\x10\0\x12\r\n\tSHAREDKEY\x10\
    \x01\x12\x07\n\x03WOT\x10\x02J\xc2\x10\n\x06\x12\x04\0\0.\x01\n\x08\n\
    \x01\x02\x12\x03\0\x08\x13\n\n\n\x02\x04\0\x12\x04\x02\0\n\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x02\x08\x0b\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x08+\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x08\x10\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03\x03\x11\x18\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x19&\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03)*\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x04\x08%\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x08\x10\n\x0c\n\
    \x05\x04\0\x02\x01\x06\x12\x03\x04\x11\x18\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x04\x19\x20\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04#$\n\x0c\n\
    \x04\x04\0\x03\0\x12\x04\x06\x08\t\t\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03\
    \x06\x10\x17\n(\n\x06\x04\0\x03\0\x02\0\x12\x03\x07\x10,\"\x19\x20subscr\
    ibe\x20or\x20unsubcribe\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x04\x12\x03\x07\
    \x10\x18\n\x0e\n\x07\x04\0\x03\0\x02\0\x05\x12\x03\x07\x19\x1d\n\x0e\n\
    \x07\x04\0\x03\0\x02\0\x01\x12\x03\x07\x1e'\n\x0e\n\x07\x04\0\x03\0\x02\
    \0\x03\x12\x03\x07*+\n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03\x08\x10,\n\
    \x0e\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03\x08\x10\x18\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x05\x12\x03\x08\x19\x1f\n\x0e\n\x07\x04\0\x03\0\x02\x01\
    \x01\x12\x03\x08\x20'\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03\x08*+\
    \n\n\n\x02\x04\x01\x12\x04\x0c\0\x11\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \x0c\x08\x0f\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x08\x20\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03\r\x08\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\r\
    \x11\x16\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\x17\x1b\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\r\x1e\x1f\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0e\
    \x08\x20\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x0e\x08\x10\n\x0c\n\x05\
    \x04\x01\x02\x01\x05\x12\x03\x0e\x11\x16\n\x0c\n\x05\x04\x01\x02\x01\x01\
    \x12\x03\x0e\x17\x1b\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0e\x1e\x1f\
    \n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x0f\x08!\n\x0c\n\x05\x04\x01\x02\
    \x02\x04\x12\x03\x0f\x08\x10\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x0f\
    \x11\x16\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0f\x17\x1c\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x03\x0f\x1f\x20\n\x0b\n\x04\x04\x01\x02\x03\x12\
    \x03\x10\x08%\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\x10\x08\x10\n\x0c\
    \n\x05\x04\x01\x02\x03\x05\x12\x03\x10\x11\x17\n\x0c\n\x05\x04\x01\x02\
    \x03\x01\x12\x03\x10\x18\x20\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x10\
    #$\nC\n\x02\x04\x02\x12\x04\x14\0.\x01\x1a7\x20topicID\x20=\x20hash(topi\
    cDescriptor);\x20(not\x20the\x20topic.name)\n\n\n\n\x03\x04\x02\x01\x12\
    \x03\x14\x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x15\x08!\n\x0c\n\x05\
    \x04\x02\x02\0\x04\x12\x03\x15\x08\x10\n\x0c\n\x05\x04\x02\x02\0\x05\x12\
    \x03\x15\x11\x17\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x15\x18\x1c\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03\x15\x1f\x20\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x16\x08#\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x16\x08\x10\n\
    \x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x16\x11\x19\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03\x16\x1a\x1e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\
    \x16!\"\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x17\x08!\n\x0c\n\x05\x04\x02\
    \x02\x02\x04\x12\x03\x17\x08\x10\n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03\
    \x17\x11\x18\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x17\x19\x1c\n\x0c\n\
    \x05\x04\x02\x02\x02\x03\x12\x03\x17\x1f\x20\n\x0c\n\x04\x04\x02\x03\0\
    \x12\x04\x19\x08\"\t\n\x0c\n\x05\x04\x02\x03\0\x01\x12\x03\x19\x10\x18\n\
    \r\n\x06\x04\x02\x03\0\x02\0\x12\x03\x1a\x10+\n\x0e\n\x07\x04\x02\x03\0\
    \x02\0\x04\x12\x03\x1a\x10\x18\n\x0e\n\x07\x04\x02\x03\0\x02\0\x06\x12\
    \x03\x1a\x19!\n\x0e\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03\x1a\"&\n\x0e\n\
    \x07\x04\x02\x03\0\x02\0\x03\x12\x03\x1a)*\n#\n\x06\x04\x02\x03\0\x02\
    \x01\x12\x03\x1b\x10(\"\x14\x20root\x20keys\x20to\x20trust\n\n\x0e\n\x07\
    \x04\x02\x03\0\x02\x01\x04\x12\x03\x1b\x10\x18\n\x0e\n\x07\x04\x02\x03\0\
    \x02\x01\x05\x12\x03\x1b\x19\x1e\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x01\
    \x12\x03\x1b\x1f#\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03\x1b&'\n\
    \x0e\n\x06\x04\x02\x03\0\x04\0\x12\x04\x1d\x10!\x11\n\x0e\n\x07\x04\x02\
    \x03\0\x04\0\x01\x12\x03\x1d\x15\x1d\n8\n\x08\x04\x02\x03\0\x04\0\x02\0\
    \x12\x03\x1e\x18!\"'\x20no\x20authentication,\x20anyone\x20can\x20publis\
    h\n\n\x10\n\t\x04\x02\x03\0\x04\0\x02\0\x01\x12\x03\x1e\x18\x1c\n\x10\n\
    \t\x04\x02\x03\0\x04\0\x02\0\x02\x12\x03\x1e\x1f\x20\nT\n\x08\x04\x02\
    \x03\0\x04\0\x02\x01\x12\x03\x1f\x18\x20\"C\x20only\x20messages\x20signe\
    d\x20by\x20keys\x20in\x20the\x20topic\x20descriptor\x20are\x20accepted\n\
    \n\x10\n\t\x04\x02\x03\0\x04\0\x02\x01\x01\x12\x03\x1f\x18\x1b\n\x10\n\t\
    \x04\x02\x03\0\x04\0\x02\x01\x02\x12\x03\x1f\x1e\x1f\nM\n\x08\x04\x02\
    \x03\0\x04\0\x02\x02\x12\x03\x20\x18\x20\"<\x20web\x20of\x20trust,\x20ce\
    rtificates\x20can\x20allow\x20publisher\x20set\x20to\x20grow\n\n\x10\n\t\
    \x04\x02\x03\0\x04\0\x02\x02\x01\x12\x03\x20\x18\x1b\n\x10\n\t\x04\x02\
    \x03\0\x04\0\x02\x02\x02\x12\x03\x20\x1e\x1f\n\x0c\n\x04\x04\x02\x03\x01\
    \x12\x04$\x08-\t\n\x0c\n\x05\x04\x02\x03\x01\x01\x12\x03$\x10\x17\n\r\n\
    \x06\x04\x02\x03\x01\x02\0\x12\x03%\x10*\n\x0e\n\x07\x04\x02\x03\x01\x02\
    \0\x04\x12\x03%\x10\x18\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x06\x12\x03%\
    \x19\x20\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x01\x12\x03%!%\n\x0e\n\x07\
    \x04\x02\x03\x01\x02\0\x03\x12\x03%()\n<\n\x06\x04\x02\x03\x01\x02\x01\
    \x12\x03&\x10-\"-\x20the\x20hashes\x20of\x20the\x20shared\x20keys\x20use\
    d\x20(salted)\n\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\x04\x12\x03&\x10\x18\
    \n\x0e\n\x07\x04\x02\x03\x01\x02\x01\x05\x12\x03&\x19\x1e\n\x0e\n\x07\
    \x04\x02\x03\x01\x02\x01\x01\x12\x03&\x1f(\n\x0e\n\x07\x04\x02\x03\x01\
    \x02\x01\x03\x12\x03&+,\n\x0e\n\x06\x04\x02\x03\x01\x04\0\x12\x04(\x10,\
    \x11\n\x0e\n\x07\x04\x02\x03\x01\x04\0\x01\x12\x03(\x15\x1c\n1\n\x08\x04\
    \x02\x03\x01\x04\0\x02\0\x12\x03)\x18!\"\x20\x20no\x20encryption,\x20any\
    one\x20can\x20read\n\n\x10\n\t\x04\x02\x03\x01\x04\0\x02\0\x01\x12\x03)\
    \x18\x1c\n\x10\n\t\x04\x02\x03\x01\x04\0\x02\0\x02\x12\x03)\x1f\x20\n9\n\
    \x08\x04\x02\x03\x01\x04\0\x02\x01\x12\x03*\x18&\"(\x20messages\x20are\
    \x20encrypted\x20with\x20shared\x20key\n\n\x10\n\t\x04\x02\x03\x01\x04\0\
    \x02\x01\x01\x12\x03*\x18!\n\x10\n\t\x04\x02\x03\x01\x04\0\x02\x01\x02\
    \x12\x03*$%\nM\n\x08\x04\x02\x03\x01\x04\0\x02\x02\x12\x03+\x18\x20\"<\
    \x20web\x20of\x20trust,\x20certificates\x20can\x20allow\x20publisher\x20\
    set\x20to\x20grow\n\n\x10\n\t\x04\x02\x03\x01\x04\0\x02\x02\x01\x12\x03+\
    \x18\x1b\n\x10\n\t\x04\x02\x03\x01\x04\0\x02\x02\x02\x12\x03+\x1e\x1f\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
