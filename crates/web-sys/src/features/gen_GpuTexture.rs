#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "wasm-bindgen" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUTexture , typescript_type = "GPUTexture")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTexture` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuTexture;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUTexture" , js_name = label)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuTexture) -> Option<String>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "GPUTexture" , js_name = label)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuTexture, value: Option<&str>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    # [wasm_bindgen (method , structural , js_class = "GPUTexture" , js_name = createView)]
    #[doc = "The `createView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_view(this: &GpuTexture) -> GpuTextureView;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuTextureView", feature = "GpuTextureViewDescriptor",))]
    # [wasm_bindgen (method , structural , js_class = "GPUTexture" , js_name = createView)]
    #[doc = "The `createView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`, `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_view_with_descriptor(
        this: &GpuTexture,
        descriptor: &GpuTextureViewDescriptor,
    ) -> GpuTextureView;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "GPUTexture" , js_name = destroy)]
    #[doc = "The `destroy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/destroy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn destroy(this: &GpuTexture);
}
