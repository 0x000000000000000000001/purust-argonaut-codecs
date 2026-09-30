use std::rc::Rc;

// The generic record path mirrors the JavaScript driver: the native schema
// tags are identities and the ordinary PureScript tail/fallback builds the
// records. Only `recordNilImpl` produces the empty record itself.

pub fn Data_Argonaut_Decode_Internal_Record_recordNilImpl(
    _left: purust_core::Func1<
        Rc<Purs_Data_Argonaut_Decode_Error::JsonDecodeError>,
        Rc<Purs_Data_Either::Either>,
    >,
    _support: crate::UnknownType,
    right: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    _object: Rc<Purs_Foreign_Object::Object>,
    _proxy: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    let empty = crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(
        purust_core::RecordFields::new(),
    ));
    right(empty)
}

pub fn Data_Argonaut_Decode_Internal_Record_recordConsImpl(
    _reflect: purust_core::Func1<(), String>,
    _step: purust_core::Func2<String, Rc<Purs_Foreign_Object::Object>, Rc<Purs_Data_Either::Either>>,
    _tail: purust_core::Func2<Rc<Purs_Foreign_Object::Object>, crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    fallback: purust_core::Func2<Rc<Purs_Foreign_Object::Object>, crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    _is_right: purust_core::Func1<Rc<Purs_Data_Either::Either>, bool>,
    _right_value: purust_core::Func1<Rc<Purs_Data_Either::Either>, crate::UnknownType>,
    _right: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    _left: purust_core::Func1<
        Rc<Purs_Data_Argonaut_Decode_Error::JsonDecodeError>,
        Rc<Purs_Data_Either::Either>,
    >,
    _support: crate::UnknownType,
    _nothing: Rc<Purs_Data_Maybe::Maybe>,
    _just: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Maybe::Maybe>>,
    object: Rc<Purs_Foreign_Object::Object>,
    proxy: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    fallback(object, proxy)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedInt(
    _support: crate::UnknownType,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedJson(
    _support: crate::UnknownType,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedNumber(
    _support: crate::UnknownType,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedString(
    _support: crate::UnknownType,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedBoolean(
    _support: crate::UnknownType,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedMaybe(
    _support: crate::UnknownType,
    _inner: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedArray(
    _support: crate::UnknownType,
    _inner: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedRecord(
    _support: crate::UnknownType,
    _row: purust_core::Func2<Rc<Purs_Foreign_Object::Object>, crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedObject(
    _inner: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    _right: purust_core::Func1<Rc<Purs_Foreign_Object::Object>, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedFieldId(
    _inner: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    impl_: purust_core::Func1<Rc<Purs_Data_Maybe::Maybe>, Rc<Purs_Data_Maybe::Maybe>>,
    maybe: Rc<Purs_Data_Maybe::Maybe>,
) -> Rc<Purs_Data_Maybe::Maybe> {
    impl_(maybe)
}

pub fn Data_Argonaut_Decode_Internal_Record_typedFieldMaybe(
    _inner: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    impl_: purust_core::Func1<Rc<Purs_Data_Maybe::Maybe>, Rc<Purs_Data_Maybe::Maybe>>,
    maybe: Rc<Purs_Data_Maybe::Maybe>,
) -> Rc<Purs_Data_Maybe::Maybe> {
    impl_(maybe)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldStep(
    _field: purust_core::Func1<Rc<Purs_Data_Maybe::Maybe>, Rc<Purs_Data_Maybe::Maybe>>,
    step: purust_core::Func2<String, Rc<Purs_Foreign_Object::Object>, Rc<Purs_Data_Either::Either>>,
    key: String,
    object: Rc<Purs_Foreign_Object::Object>,
) -> Rc<Purs_Data_Either::Either> {
    step(key, object)
}

pub fn Data_Argonaut_Decode_Internal_Record_decodeFieldFast(
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_getFieldImpl(
    fallback: purust_core::Func3<
        purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
        Rc<Purs_Foreign_Object::Object>,
        String,
        Rc<Purs_Data_Either::Either>,
    >,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    object: Rc<Purs_Foreign_Object::Object>,
    key: String,
) -> Rc<Purs_Data_Either::Either> {
    fallback(decoder, object, key)
}

pub fn Data_Argonaut_Decode_Internal_Record_getFieldOptionalImpl(
    fallback: purust_core::Func3<
        purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
        Rc<Purs_Foreign_Object::Object>,
        String,
        Rc<Purs_Data_Either::Either>,
    >,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    object: Rc<Purs_Foreign_Object::Object>,
    key: String,
) -> Rc<Purs_Data_Either::Either> {
    fallback(decoder, object, key)
}

pub fn Data_Argonaut_Decode_Internal_Record_getFieldOptionalNullableImpl(
    fallback: purust_core::Func3<
        purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
        Rc<Purs_Foreign_Object::Object>,
        String,
        Rc<Purs_Data_Either::Either>,
    >,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    object: Rc<Purs_Foreign_Object::Object>,
    key: String,
) -> Rc<Purs_Data_Either::Either> {
    fallback(decoder, object, key)
}

pub fn Data_Argonaut_Decode_Internal_Record_borrowObject(
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    decoder(json)
}

pub fn Data_Argonaut_Decode_Internal_Record_schemaDecoderABI1() -> i64 {
    1
}
