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

// Take the payload of a successful result without cloning it: the callbacks
// hand back a freshly allocated `Either`, so an unshared `Rc` yields the value
// itself and the record insert can mutate in place.
fn take_right(result: Rc<Purs_Data_Either::Either>) -> crate::UnknownType {
    match Rc::try_unwrap(result) {
        Ok(Purs_Data_Either::Either::Right(value)) => value,
        Ok(Purs_Data_Either::Either::Left(_)) => unreachable!("record step expected a success"),
        Err(shared) => match shared.as_ref() {
            Purs_Data_Either::Either::Right(value) => value.clone(),
            Purs_Data_Either::Either::Left(_) => unreachable!("record step expected a success"),
        },
    }
}

// The ordinary fallback re-reads the field the `step` closure already decoded
// and rebuilds the record through the PureScript insert path. Decode the field
// once, then decode the tail and insert the value natively: same order, same
// errors, one lookup per field.
pub fn Data_Argonaut_Decode_Internal_Record_recordConsImpl(
    reflect: purust_core::Func1<(), String>,
    step: purust_core::Func2<String, Rc<Purs_Foreign_Object::Object>, Rc<Purs_Data_Either::Either>>,
    tail: purust_core::Func2<Rc<Purs_Foreign_Object::Object>, crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    _fallback: purust_core::Func2<Rc<Purs_Foreign_Object::Object>, crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    _is_right: purust_core::Func1<Rc<Purs_Data_Either::Either>, bool>,
    _right_value: purust_core::Func1<Rc<Purs_Data_Either::Either>, crate::UnknownType>,
    right: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
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
    let field = reflect(());
    let field_result = step(field.clone(), object.clone());
    if matches!(field_result.as_ref(), Purs_Data_Either::Either::Left(_)) {
        return field_result;
    }
    let tail_result = tail(object, proxy);
    if matches!(tail_result.as_ref(), Purs_Data_Either::Either::Left(_)) {
        return tail_result;
    }
    let value = take_right(field_result);
    let record = take_right(tail_result);
    right(record.__purust_set_field(&field, value))
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
    // A successful decode needs no AtKey wrapper; anything else keeps the
    // exact generic accessor, missing keys included.
    match object.get(&key) {
        Some(json) => {
            let result = decoder(json);
            if matches!(result.as_ref(), Purs_Data_Either::Either::Right(_)) {
                result
            } else {
                fallback(decoder, object, key)
            }
        }
        None => fallback(decoder, object, key),
    }
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
    // Missing keys are `Right Nothing`; a present value keeps the decoder's
    // result wrapped in `Just`, and every failure keeps the generic accessor
    // (which owns the AtKey wrapping).
    match object.get(&key) {
        None => Rc::new(Purs_Data_Either::Either::Right(purust_maybe_nothing())),
        Some(json) => {
            let result = decoder(json);
            if matches!(result.as_ref(), Purs_Data_Either::Either::Right(_)) {
                Rc::new(Purs_Data_Either::Either::Right(purust_maybe_just(
                    purust_take_right(result),
                )))
            } else {
                fallback(decoder, object, key)
            }
        }
    }
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
    // This accessor treats null and missing alike as `Nothing`.
    match object.get(&key) {
        None => Rc::new(Purs_Data_Either::Either::Right(purust_maybe_nothing())),
        Some(json) => {
            if matches!(json.resolve(), crate::Value::Null) {
                return Rc::new(Purs_Data_Either::Either::Right(purust_maybe_nothing()));
            }
            let result = decoder(json);
            if matches!(result.as_ref(), Purs_Data_Either::Either::Right(_)) {
                Rc::new(Purs_Data_Either::Either::Right(purust_maybe_just(
                    purust_take_right(result),
                )))
            } else {
                fallback(decoder, object, key)
            }
        }
    }
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

pub fn Data_Argonaut_Decode_Internal_Record_schemaDecoderABI2() -> i64 {
    2
}

pub fn Data_Argonaut_Decode_Internal_Record_schemaTextDecoderABI2() -> i64 {
    2
}

pub use Purs_Data_Argonaut_Core::PurustJsonDocument as SchemaText;
impl<'a> SchemaInput for Purs_Data_Argonaut_Core::PurustJsonCursor<'a> {
    type Items = Purs_Data_Argonaut_Core::PurustJsonCursorItems<'a>;
    fn is_null(&self) -> bool { self.kind() == b'n' }
    fn scalar(self, kind: &str) -> Option<crate::UnknownType> { self.scalar(kind) }
    fn fields<const N: usize>(self, keys: [&str; N]) -> Option<[Option<Self>; N]> { self.fields(keys) }
    fn array(self) -> Option<Self::Items> { self.array() }
}

// A single generated worker is monomorphized over a DOM or text cursor. Its
// success result uses an owned runtime representation and is fully built
// before crossing the public Either/FFI boundary.
pub trait SchemaInput: Clone + Sized {
    type Items: std::iter::ExactSizeIterator<Item = Self>;
    fn is_null(&self) -> bool;
    fn scalar(self, kind: &str) -> Option<crate::UnknownType>;
    fn fields<const N: usize>(self, keys: [&str; N]) -> Option<[Option<Self>; N]>;
    fn array(self) -> Option<Self::Items>;
}

#[derive(Clone)]
pub struct SchemaDom(pub crate::UnknownType);

pub struct SchemaDomItems {
    values: purust_core::ArrayItems,
}

impl std::iter::Iterator for SchemaDomItems {
    type Item = SchemaDom;
    fn next(&mut self) -> Option<Self::Item> {
        self.values.next().map(SchemaDom)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }
}
impl std::iter::ExactSizeIterator for SchemaDomItems {}

impl SchemaInput for SchemaDom {
    type Items = SchemaDomItems;
    fn is_null(&self) -> bool { matches!(self.0.resolve(), crate::Value::Null) }
    fn scalar(self, kind: &str) -> Option<crate::UnknownType> {
        if kind == "Json" { return Some(self.0); }
        match (kind, self.0) {
            ("String", value @ crate::Value::String(_)) => Some(value),
            ("Boolean", value @ crate::Value::Bool(_)) => Some(value),
            ("Number", value @ crate::Value::Number(_)) => Some(value),
            ("Number", crate::Value::Int(number)) => Some(crate::Value::Number(number as f64)),
            ("Int", crate::Value::Number(number)) if purust_valid_int(number) => Some(crate::Value::Int(number as i64)),
            ("Int", value @ crate::Value::Int(number)) if (-2147483648..=2147483647).contains(&number) => Some(value),
            _ => None,
        }
    }
    fn fields<const N: usize>(self, keys: [&str; N]) -> Option<[Option<Self>; N]> {
        let native = match self.0.resolve() { crate::Value::Class(native) => native, _ => return None };
        let object = native.downcast_ref::<Rc<purust_core::SharedRecord>>()?;
        Some(object.get_many(keys).map(|value| value.map(SchemaDom)))
    }
    fn array(self) -> Option<Self::Items> {
        self.0.is_array().then(|| SchemaDomItems { values: self.0.array_iter() })
    }
}

// ---------------------------------------------------------------------------
// Native construction plans.
//
// A plan is an immutable description of one record row: for every field, the
// field name, its native kind and the exact generic step of that field. The
// runner decodes a field natively while the value has the declared shape and
// otherwise calls the step, so anything unrecognized keeps the ordinary
// behavior, errors included.

struct PurustRecordField {
    name: Rc<str>,
    spec: Rc<PurustFieldSpec>,
    step: purust_core::Func2<String, Rc<Purs_Foreign_Object::Object>, Rc<Purs_Data_Either::Either>>,
    tail: Rc<PurustRecordPlan>,
}

struct PurustRecordPlan {
    head: Option<Rc<PurustRecordField>>,
    len: usize,
}

enum PurustFieldSpec {
    Int,
    Number,
    String,
    Boolean,
    Json,
    Maybe(Rc<PurustFieldSpec>),
    Array(Rc<PurustFieldSpec>),
    Record(Rc<PurustRecordPlan>),
    Custom,
}

fn purust_box_spec(spec: PurustFieldSpec) -> crate::UnknownType {
    crate::Value::Class(Rc::new(Rc::new(spec)))
}

fn purust_unbox_spec(value: &crate::UnknownType) -> Rc<PurustFieldSpec> {
    value.unwrap_class::<Rc<PurustFieldSpec>>().clone()
}

fn purust_box_plan(plan: PurustRecordPlan) -> crate::UnknownType {
    crate::Value::Class(Rc::new(Rc::new(plan)))
}

fn purust_unbox_plan(value: &crate::UnknownType) -> Rc<PurustRecordPlan> {
    value.unwrap_class::<Rc<PurustRecordPlan>>().clone()
}

pub fn purust_maybe_just(value: crate::UnknownType) -> crate::UnknownType {
    crate::Value::Class(Rc::new(Rc::new(Purs_Data_Maybe::Maybe::Just(value))))
}

pub fn purust_maybe_nothing() -> crate::UnknownType {
    crate::Value::Class(Rc::new(Rc::new(Purs_Data_Maybe::Maybe::Nothing)))
}

// Mirrors Data.Int.fromNumber: finite, integral and inside the Int range.
fn purust_valid_int(number: f64) -> bool {
    number.is_finite()
        && number.fract() == 0.0
        && number >= (-2147483648.0)
        && number <= 2147483647.0
}

fn purust_native_field(
    spec: &PurustFieldSpec,
    value: Option<crate::UnknownType>,
) -> Option<crate::UnknownType> {
    match spec {
        PurustFieldSpec::Custom => None,
        PurustFieldSpec::Json => value,
        PurustFieldSpec::Boolean => match value?.resolve() {
            crate::Value::Bool(flag) => Some(crate::Value::Bool(*flag)),
            _ => None,
        },
        PurustFieldSpec::String => {
            // The value is already owned here: move it into the record
            // instead of copying the string a second time.
            let value = value?;
            if matches!(value.resolve(), crate::Value::String(_)) {
                Some(value)
            } else {
                None
            }
        }
        PurustFieldSpec::Number => match value?.resolve() {
            crate::Value::Number(number) => Some(crate::Value::Number(*number)),
            crate::Value::Int(number) => Some(crate::Value::Number(*number as f64)),
            _ => None,
        },
        PurustFieldSpec::Int => match value?.resolve() {
            crate::Value::Int(number) => Some(crate::Value::Int(*number)),
            crate::Value::Number(number) if purust_valid_int(*number) => {
                Some(crate::Value::Int(*number as i64))
            }
            _ => None,
        },
        PurustFieldSpec::Maybe(inner) => {
            let value = value?;
            if matches!(value.resolve(), crate::Value::Null) {
                return Some(purust_maybe_nothing());
            }
            purust_native_field(inner, Some(value)).map(purust_maybe_just)
        }
        PurustFieldSpec::Array(inner) => {
            let value = value?;
            if !value.is_array() { return None; }
            let mut decoded = Vec::with_capacity(value.array_len());
            for item in value.array_iter() {
                decoded.push(purust_native_field(inner, Some(item))?);
            }
            Some(crate::Value::Array(Rc::new(decoded)))
        }
        PurustFieldSpec::Record(plan) => {
            let value = value?;
            let object = match value.resolve() {
                crate::Value::Class(native) => native
                    .downcast_ref::<Rc<Purs_Foreign_Object::Object>>()
                    .cloned(),
                _ => None,
            }?;
            match purust_run_plan(plan, object).as_ref() {
                Purs_Data_Either::Either::Right(record) => Some(record.clone()),
                Purs_Data_Either::Either::Left(_) => None,
            }
        }
    }
}

fn purust_run_plan(
    plan: &PurustRecordPlan,
    object: Rc<Purs_Foreign_Object::Object>,
) -> Rc<Purs_Data_Either::Either> {
    // Row labels are unique by construction, so fields append without a
    // duplicate scan.
    let mut fields = purust_core::RecordFields::with_capacity(plan.len);
    let mut node = plan.head.clone();
    while let Some(field) = node {
        match purust_native_field(&field.spec, object.get(&field.name)) {
            Some(value) => fields.push(field.name.clone(), value),
            None => {
                let result = (field.step)(field.name.to_string(), object.clone());
                if matches!(result.as_ref(), Purs_Data_Either::Either::Left(_)) {
                    return result;
                }
                fields.push(field.name.clone(), purust_take_right(result));
            }
        }
        node = field.tail.head.clone();
    }
    Rc::new(Purs_Data_Either::Either::Right(
        crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(fields)),
    ))
}

fn purust_take_right(result: Rc<Purs_Data_Either::Either>) -> crate::UnknownType {
    match Rc::try_unwrap(result) {
        Ok(Purs_Data_Either::Either::Right(value)) => value,
        Ok(Purs_Data_Either::Either::Left(_)) => unreachable!("plan step expected a success"),
        Err(shared) => match shared.as_ref() {
            Purs_Data_Either::Either::Right(value) => value.clone(),
            Purs_Data_Either::Either::Left(_) => unreachable!("plan step expected a success"),
        },
    }
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldInt() -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Int)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldNumber() -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Number)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldString() -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::String)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldBoolean() -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Boolean)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldJson() -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Json)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldCustom() -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Custom)
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldMaybe(inner: crate::UnknownType) -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Maybe(purust_unbox_spec(&inner)))
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldArray(inner: crate::UnknownType) -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Array(purust_unbox_spec(&inner)))
}

pub fn Data_Argonaut_Decode_Internal_Record_fieldRecord(plan: crate::UnknownType) -> crate::UnknownType {
    purust_box_spec(PurustFieldSpec::Record(purust_unbox_plan(&plan)))
}

pub fn Data_Argonaut_Decode_Internal_Record_planNil() -> crate::UnknownType {
    purust_box_plan(PurustRecordPlan { head: None, len: 0 })
}

pub fn Data_Argonaut_Decode_Internal_Record_planCons(
    name: String,
    spec: crate::UnknownType,
    step: purust_core::Func2<String, Rc<Purs_Foreign_Object::Object>, Rc<Purs_Data_Either::Either>>,
    tail: crate::UnknownType,
) -> crate::UnknownType {
    let tail = purust_unbox_plan(&tail);
    let len = tail.len + 1;
    purust_box_plan(PurustRecordPlan {
        head: Some(Rc::new(PurustRecordField {
            name: Rc::from(name),
            spec: purust_unbox_spec(&spec),
            step,
            tail,
        })),
        len,
    })
}

pub fn Data_Argonaut_Decode_Internal_Record_runRecordPlan(
    plan: crate::UnknownType,
    object: Rc<Purs_Foreign_Object::Object>,
) -> Rc<Purs_Data_Either::Either> {
    purust_run_plan(&purust_unbox_plan(&plan), object)
}

// ---------------------------------------------------------------------------
// Native container decoding.

pub fn Data_Argonaut_Decode_Internal_Record_nativeMaybe(
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    // `decodeMaybe` maps the decoder result; no error wrapper is involved, so
    // the decoder's own failure is the exact generic result.
    if matches!(json.resolve(), crate::Value::Null) {
        return Rc::new(Purs_Data_Either::Either::Right(purust_maybe_nothing()));
    }
    let result = decoder(json);
    if matches!(result.as_ref(), Purs_Data_Either::Either::Left(_)) {
        return result;
    }
    Rc::new(Purs_Data_Either::Either::Right(purust_maybe_just(
        purust_take_right(result),
    )))
}

pub fn Data_Argonaut_Decode_Internal_Record_nativeArray(
    fallback: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    // The generic traversal owns the Named/AtIndex wrapping, so any
    // complication goes back through it.
    if !json.is_array() { return fallback(json); }
    let items = json.array_iter();
    let mut decoded = Vec::with_capacity(items.len());
    for item in items {
        let result = decoder(item);
        if matches!(result.as_ref(), Purs_Data_Either::Either::Left(_)) {
            return fallback(json);
        }
        decoded.push(purust_take_right(result));
    }
    Rc::new(Purs_Data_Either::Either::Right(crate::Value::Array(
        Rc::new(decoded),
    )))
}

pub fn Data_Argonaut_Decode_Internal_Record_nativeObject(
    fallback: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    decoder: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Either::Either>>,
    json: crate::UnknownType,
) -> Rc<Purs_Data_Either::Either> {
    let shared = match json.resolve() {
        crate::Value::Class(native) => native
            .downcast_ref::<Rc<purust_core::SharedRecord>>()
            .cloned(),
        _ => None,
    };
    let shared = match shared {
        Some(shared) => shared,
        None => return fallback(json),
    };
    let mut entries = Vec::new();
    for (key, value) in shared.entries_unsorted_shared() {
        let result = decoder(value);
        if matches!(result.as_ref(), Purs_Data_Either::Either::Left(_)) {
            return fallback(json);
        }
        entries.push((key, purust_take_right(result)));
    }
    Rc::new(Purs_Data_Either::Either::Right(crate::Value::Class(
        Rc::new(Rc::new(purust_core::SharedRecord::from_entries_shared(entries))),
    )))
}
