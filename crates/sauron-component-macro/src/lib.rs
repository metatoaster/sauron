use quote::quote;

#[proc_macro_attribute]
pub fn custom_element(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let custom_tag: proc_macro2::Literal =
        syn::parse(attr).expect("must be a literal");
    let impl_item: syn::ItemImpl = syn::parse(item)
        .expect("Expecting custom_element macro to be used in impl trait");

    let (_, path, _) = &impl_item
        .trait_
        .as_ref()
        .expect("must have a trait implementation");

    let component: &syn::PathSegment =
        &path.segments.last().expect("must have a last segment");

    let component_ident = component.ident.to_string();

    match &*component_ident {
        "Component" => impl_component(&impl_item, &custom_tag, component),
        "Application" => impl_application(&impl_item, &custom_tag, component),
        _ => panic!("unsupported trait implementation: {}", component_ident),
    }
}

/// Generate the component registration for component that has a Component trait implementation
fn impl_component(
    impl_item: &syn::ItemImpl,
    custom_tag: &proc_macro2::Literal,
    component: &syn::PathSegment,
) -> proc_macro::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    let component_msg = get_msg_generic_ident(&component);
    let self_type = &impl_item.self_ty;
    if let syn::Type::Path(type_path) = self_type.as_ref() {
        let path_segment = &type_path.path.segments[0];
        let component = &path_segment.ident;
        let derive_component = proc_macro2::Ident::new(
            &format!("_{}__CustomElement", component),
            proc_macro2::Span::call_site(),
        );
        let derive_msg = proc_macro2::Ident::new(
            &format!("_{}__CustomMsg", component),
            proc_macro2::Span::call_site(),
        );

        let derive_component_str = derive_component.to_string();

        tokens.extend(quote! {

            #impl_item

            #[allow(non_camel_case_types)]
            pub struct #derive_msg(#component_msg);

            #[allow(non_camel_case_types)]
            #[wasm_bindgen]
            pub struct #derive_component{
                program: Program<#component<#derive_msg>, #derive_msg>,
            }

            #[wasm_bindgen]
            impl #derive_component {
                #[wasm_bindgen(constructor)]
                pub fn new(node: JsValue) -> Self {
                    use sauron::wasm_bindgen::JsCast;
                    log::info!("constructor..");
                    let mount_node: &web_sys::Node = node.unchecked_ref();
                    Self {
                        program: Program::new(
                            #component::default(),
                            mount_node,
                            false,
                            true,
                        ),
                    }
                }

                #[wasm_bindgen(method)]
                pub fn observed_attributes() -> JsValue {
                    JsValue::from_serde(&<#component::<#derive_msg> as Component<#component_msg, #derive_msg>>::observed_attributes())
                        .expect("must parse from serde")
                }

                #[wasm_bindgen(method)]
                pub fn attribute_changed_callback(&self) {
                    use std::ops::DerefMut;
                    use sauron::wasm_bindgen::JsCast;
                    log::info!("attribute changed...");
                    let mount_node = self.program.mount_node();
                    let mount_element: &web_sys::Element = mount_node.unchecked_ref();
                    let attribute_names = mount_element.get_attribute_names();
                    let len = attribute_names.length();
                    let mut attribute_values: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
                    for i in 0..len {
                        let name = attribute_names.get(i);
                        let attr_name =
                            name.as_string().expect("must be a string attribute");
                        if let Some(attr_value) = mount_element.get_attribute(&attr_name) {
                            attribute_values.insert(attr_name, attr_value);
                        }
                    }
                    <#component<#derive_msg> as Component<#component_msg, #derive_msg>>::attributes_changed(self.program.app.borrow_mut().deref_mut(), attribute_values);
                }

                #[wasm_bindgen(method)]
                pub fn connected_callback(&mut self) {
                    use std::ops::Deref;
                    self.program.mount();
                    log::info!("Component is connected..");
                    let component_style = <#component<#derive_msg> as Component<#component_msg, #derive_msg>>::style(self.program.app.borrow().deref());
                    self.program.inject_style_to_mount(&component_style);
                    self.program.update_dom();
                }
                #[wasm_bindgen(method)]
                pub fn disconnected_callback(&mut self) {
                    log::info!("Component is disconnected..");
                }
                #[wasm_bindgen(method)]
                pub fn adopted_callback(&mut self) {
                    log::info!("Component is adopted..");
                }

            }

            impl Application<#derive_msg> for #component<#derive_msg> {
                fn update(&mut self, msg: #derive_msg) -> Cmd<Self, #derive_msg> {
                    let mount_attributes = <Self as Component<#component_msg, #derive_msg>>::attributes_for_mount(self);
                    Cmd::batch([
                        Cmd::from(
                            <Self as Component<#component_msg, #derive_msg>>::update(
                                self, msg.0,
                            )
                            .localize(#derive_msg),
                        ),
                        Cmd::new(|program| {
                            program.update_mount_attributes(mount_attributes);
                        }),
                    ])
                }

                fn view(&self) -> Node<#derive_msg> {
                    <Self as Component<#component_msg, #derive_msg>>::view(self)
                        .map_msg(#derive_msg)
                }
            }

            #[wasm_bindgen]
            pub fn register(){
                sauron::register_custom_element(#custom_tag, #derive_component_str, "HTMLElement");
            }

        });
    } else {
        panic!("Expecting a Path");
    }
    tokens.into()
}

/// Generate the code for registering the application as a custom component
fn impl_application(
    impl_item: &syn::ItemImpl,
    custom_tag: &proc_macro2::Literal,
    component: &syn::PathSegment,
) -> proc_macro::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    let app_msg = get_msg_generic_ident(&component);
    let self_type = &impl_item.self_ty;
    if let syn::Type::Path(type_path) = self_type.as_ref() {
        let path_segment = &type_path.path.segments[0];
        let app = &path_segment.ident;
        let derive_component = proc_macro2::Ident::new(
            &format!("_{}__CustomElement", app),
            proc_macro2::Span::call_site(),
        );

        let derive_component_str = derive_component.to_string();

        tokens.extend(quote! {

            #impl_item

            #[allow(non_camel_case_types)]
            #[wasm_bindgen]
            pub struct #derive_component{
                program: Program<#app, #app_msg>,
            }

            #[wasm_bindgen]
            impl #derive_component {
                #[wasm_bindgen(constructor)]
                pub fn new(node: JsValue) -> Self {
                    use sauron::wasm_bindgen::JsCast;
                    log::info!("constructor..");
                    let mount_node: &web_sys::Node = node.unchecked_ref();
                    Self {
                        program: Program::new(
                            #app::default(),
                            mount_node,
                            false,
                            true,
                        ),
                    }
                }

                #[wasm_bindgen(method)]
                pub fn observed_attributes() -> JsValue {
                    JsValue::from_serde(&<#app as Application<#app_msg>>::observed_attributes())
                        .expect("must parse from serde")
                }

                #[wasm_bindgen(method)]
                pub fn attribute_changed_callback(&self) {
                    use sauron::wasm_bindgen::JsCast;
                    log::info!("attribute changed...");
                    let mount_node = self.program.mount_node();
                    let mount_element: &web_sys::Element = mount_node.unchecked_ref();
                    let attribute_names = mount_element.get_attribute_names();
                    let len = attribute_names.length();
                    let mut attribute_values: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
                    for i in 0..len {
                        let name = attribute_names.get(i);
                        let attr_name =
                            name.as_string().expect("must be a string attribute");
                        if let Some(attr_value) = mount_element.get_attribute(&attr_name) {
                            attribute_values.insert(attr_name, attr_value);
                        }
                    }
                    self.program
                        .app
                        .borrow_mut()
                        .attributes_changed(attribute_values);
                }

                #[wasm_bindgen(method)]
                pub fn connected_callback(&mut self) {
                    use std::ops::Deref;
                    self.program.mount();
                    log::info!("Application is connected..");
                    let app_style = self.program.app.borrow().style();
                    self.program.inject_style_to_mount(&app_style);
                    self.program.update_dom();
                }
                #[wasm_bindgen(method)]
                pub fn disconnected_callback(&mut self) {
                    log::info!("Application is disconnected..");
                }
                #[wasm_bindgen(method)]
                pub fn adopted_callback(&mut self) {
                    log::info!("Application is adopted..");
                }

            }

            #[wasm_bindgen]
            pub fn register_application(){
                sauron::register_custom_element(#custom_tag, #derive_component_str, "HTMLElement");
            }

        });
    } else {
        panic!("Expecting a Path");
    }
    tokens.into()
}

fn get_msg_generic_ident(component: &syn::PathSegment) -> proc_macro2::Ident {
    let component_msg =
        if let syn::PathArguments::AngleBracketed(component_msg) =
            &component.arguments
        {
            let first_arg_generics = &component_msg.args[0];
            if let syn::GenericArgument::Type(type_) = first_arg_generics {
                if let syn::Type::Path(type_path) = type_ {
                    let generic = type_path
                        .path
                        .segments
                        .last()
                        .expect("must have a generic path segment");
                    generic.ident.clone()
                } else {
                    panic!("expecting a type path");
                }
            } else {
                panic!("expecting a generic argument type");
            }
        } else {
            panic!("expecting a generic argument");
        };
    component_msg
}
