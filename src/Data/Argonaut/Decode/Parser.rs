// The native text decoder is a marker in this port: `decodeJsonStringImpl`
// keeps the ordinary parse-then-decode composition, exactly like the
// JavaScript implementation.
pub fn Data_Argonaut_Decode_Parser_textDecoderABI1() -> i64 {
    1
}

pub fn Data_Argonaut_Decode_Parser_decodeJsonStringImpl(
    _abi: i64,
    fallback: purust_core::Func1<String, std::rc::Rc<Purs_Data_Either::Either>>,
    _decoder: purust_core::Func1<crate::UnknownType, std::rc::Rc<Purs_Data_Either::Either>>,
    text: String,
) -> std::rc::Rc<Purs_Data_Either::Either> {
    fallback(text)
}
