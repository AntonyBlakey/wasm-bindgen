#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "wasm-bindgen" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBMutableFile , typescript_type = "IDBMutableFile")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbMutableFile` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub type IdbMutableFile;
    # [wasm_bindgen (structural , method , getter , js_class = "IDBMutableFile" , js_name = name)]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub fn name(this: &IdbMutableFile) -> String;
    # [wasm_bindgen (structural , method , getter , js_class = "IDBMutableFile" , js_name = type)]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub fn type_(this: &IdbMutableFile) -> String;
    #[cfg(feature = "IdbDatabase")]
    # [wasm_bindgen (structural , method , getter , js_class = "IDBMutableFile" , js_name = database)]
    #[doc = "Getter for the `database` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/database)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbMutableFile`*"]
    pub fn database(this: &IdbMutableFile) -> IdbDatabase;
    # [wasm_bindgen (structural , method , getter , js_class = "IDBMutableFile" , js_name = onabort)]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub fn onabort(this: &IdbMutableFile) -> Option<::js_sys::Function>;
    # [wasm_bindgen (structural , method , setter , js_class = "IDBMutableFile" , js_name = onabort)]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub fn set_onabort(this: &IdbMutableFile, value: Option<&::js_sys::Function>);
    # [wasm_bindgen (structural , method , getter , js_class = "IDBMutableFile" , js_name = onerror)]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub fn onerror(this: &IdbMutableFile) -> Option<::js_sys::Function>;
    # [wasm_bindgen (structural , method , setter , js_class = "IDBMutableFile" , js_name = onerror)]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    pub fn set_onerror(this: &IdbMutableFile, value: Option<&::js_sys::Function>);
    #[cfg(feature = "DomRequest")]
    # [wasm_bindgen (catch , method , structural , js_class = "IDBMutableFile" , js_name = getFile)]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`, `IdbMutableFile`*"]
    pub fn get_file(this: &IdbMutableFile) -> Result<DomRequest, JsValue>;
    #[cfg(feature = "IdbFileHandle")]
    # [wasm_bindgen (catch , method , structural , js_class = "IDBMutableFile" , js_name = open)]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    pub fn open(this: &IdbMutableFile) -> Result<IdbFileHandle, JsValue>;
}
