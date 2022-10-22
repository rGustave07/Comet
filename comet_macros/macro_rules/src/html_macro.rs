#[macro_export]
macro_rules! html_arr {
    // if
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    if
                        ($($predicate:tt)*)
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                    if
                        replace_self!($self, $($predicate)*)
                        { html! { $self, $f, $($e)* } }
                    else
                        { html! { $self, $f, span {}} }
                    }
                ]
            }
        }}
    };
    // for
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    for
                        $($predicate:ident),+ in ($($iter:tt)*)
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {

                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let elem = document.create_element("span").unwrap();

                        for ($($predicate),*) in replace_self!($self, $($iter)*) {
                            elem.append_child(&html! { $self, $f, $($e)* }).unwrap();
                        }

                        elem
                    }
                ]
            }
        }}
    };
    // tag
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    $tag:ident $(#$id_name:ident)? $(.$class_name:ident)*
                        $([$($attr_name:ident : {$($attr_value:tt)*} ),*])?
                        $($(@$ev:ident : {$($evcode:tt)*} ),+ )?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        use std::collections::BTreeMap;
                        use wasm_bindgen::JsCast;

                        {
                            let window = web_sys::window().expect("no global `window` exists");
                            let document = window.document().expect("should have a document on window");

                            let elem_str = stringify!($tag);

                            let elem = document.create_element(elem_str).unwrap();

                            if elem_str.starts_with("\"") {
                                elem.set_inner_html(elem_str);

                                elem
                            } else {
                                $(
                                    elem.set_id(&stringify!($id_name));
                                )?

                                $(
                                    elem.class_list().add_1(&stringify!($class_name)).unwrap();
                                )*


                                let children = html_arr!($self, $f, $($e)*);

                                 for child in children {
                                    elem.append_child(
                                        &child
                                    )
                                    .unwrap();
                                };

                                #[allow(unused_mut, unused_assignments)]
                                let mut attrs: BTreeMap<String, String> = BTreeMap::new();

                                $(
                                    attrs = [$((stringify!($attr_name).to_string(), replace_self!($self, $($attr_value)*).to_string())),*].into();

                                    elem.set_attribute("style", &attrs.iter().map(|(k, v)| format!("{}: {};", k, v)).collect::<Vec<_>>().join("")).unwrap();
                                )?

                                #[allow(unused_mut, unused_assignments)]
                                let mut evcode: BTreeMap<String, Msg> = BTreeMap::new();

                                $(
                                    evcode = [($(stringify!($ev).into(),
                                       gen_full_variant!($($evcode)*)
                                    ),+)].into();

                                    if let Some(event) = evcode.get("click") {
                                        let f = $f.clone();
                                        let event = event.clone();

                                        let closure = Closure::<dyn Fn()>::wrap(Box::new(move || {
                                            f(event.clone());
                                        }));

                                        elem.dyn_ref::<web_sys::HtmlElement>()
                                            .expect("#should be an `HtmlElement`")
                                            .set_onclick(Some(closure.as_ref().unchecked_ref()));

                                        // FIXME: leak
                                        closure.forget();
                                    }
                                )?

                                elem
                            }
                        }
                    }
                ]
            }
        }}
    };

    // Component
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    @{$($comp:tt)+}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let component_container = document.create_element("span").unwrap();

                        let component = replace_self!(
                            $self,
                            $($comp)+
                        ).clone();

                        comet::core::component::run_rec(component, &component_container);

                        component_container
                    }
                ]
            }
        }}
    };

    // Text
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    { $($code:tt)* }
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let elem = document.create_element("span").unwrap();

                        elem.set_inner_html(&replace_self!(
                            $self,
                            $($code)+
                        ).to_string());

                        elem
                    }
                ]
            }
        }}
    };



    // Empty rule, to handle the case where there is no children
    () => {
        vec![]
    };

    // Final case, where we return the vec with all the elements
    (
        $self:ident,
        $f:ident,
        {
            {
                {}
                [$($expanded:tt)*]
            }
        }
    ) => {
        vec![$($expanded),*]
    };

    // Entry point, base rule
    // This is defined last, else it causes an infinite recursion as it matches with itself right away
    (
        $self:ident,
        $f:ident,
        $( $e:tt )*
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $( $e )*
                }
                []
            }
        }}
    };
}

// Conveinience macro to get the root element of the defined dom
#[macro_export]
macro_rules! html {
    (
        $self:ident,
        $f:ident,
        $( $e:tt )*
    ) => {
        {
            let mut arr = html_arr! {
                $self,
                $f,
                $($e)*
            };

            if arr.len() != 1 {
                panic!("The html macro must have exactly one root element");
            }

            arr.pop().unwrap().into()
        }
    };
}
