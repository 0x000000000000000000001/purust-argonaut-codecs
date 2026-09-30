// Preserve the ordinary PureScript implementation on JavaScript. The native
// backend uses the same interface to select its shared construction plan.
export const recordNilImpl = _left => _support => right => _object => _proxy => right({});
export const recordConsImpl = _reflect => _step => _tail => fallback => _isRight => _rightValue => _right => _left => _support => _nothing => _just => fallback;

// Schema tags are a native-backend optimisation. JavaScript keeps the ordinary
// decoder and ignores the tag; behaviour is unchanged.
export const typedInt = _support => decoder => decoder;
export const typedJson = _support => decoder => decoder;
export const typedNumber = _support => decoder => decoder;
export const typedString = _support => decoder => decoder;
export const typedBoolean = _support => decoder => decoder;
export const typedMaybe = _support => _inner => decoder => decoder;
export const typedArray = _support => _inner => decoder => decoder;
export const typedRecord = _support => _row => decoder => decoder;
export const typedObject = _inner => decoder => _right => decoder;
export const typedFieldId = _inner => impl => impl;
export const typedFieldMaybe = _inner => impl => impl;
export const fieldStep = _field => step => step;
export const decodeFieldFast = decoder => json => decoder(json);
export const getFieldImpl = fallback => fallback;
export const getFieldOptionalImpl = fallback => fallback;
export const getFieldOptionalNullableImpl = fallback => fallback;
export const borrowObject = decoder => json => decoder(json);
export const schemaDecoderABI1 = 1;
export const schemaDecoderABI2 = 2;
export const schemaTextDecoderABI2 = 2;

// Native construction plans are provided by the purust runtime FFI. The
// JavaScript backend keeps the ordinary decoding path, so these markers only
// satisfy the foreign-module contract.
const nativePlanUnsupported = () => {
  throw new Error("Data.Argonaut.Decode.Internal.Record: native plans require the purust runtime");
};
export const fieldInt = nativePlanUnsupported;
export const fieldNumber = nativePlanUnsupported;
export const fieldString = nativePlanUnsupported;
export const fieldBoolean = nativePlanUnsupported;
export const fieldJson = nativePlanUnsupported;
export const fieldCustom = nativePlanUnsupported;
export const fieldMaybe = nativePlanUnsupported;
export const fieldArray = nativePlanUnsupported;
export const fieldRecord = nativePlanUnsupported;
export const planNil = nativePlanUnsupported;
export const planCons = nativePlanUnsupported;
export const runRecordPlan = nativePlanUnsupported;
export const nativeMaybe = nativePlanUnsupported;
export const nativeArray = nativePlanUnsupported;
export const nativeObject = nativePlanUnsupported;
