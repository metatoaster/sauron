use log::*;
use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    mt_dom::patch::*,
    node,
    *,
};
use sauron_core::{
    body,
    html::div,
    Cmd,
    Component,
    Node,
    Program,
};
use std::{
    cell::RefCell,
    rc::Rc,
};
use wasm_bindgen_test::*;
use web_sys::InputEvent;

#[test]
fn new_lines_ignored() {
    let current_dom: Node<()> = node!(
    <div class="app">
       <h1>"Lines"</h1>
       <div>
          <div class="grid__wrapper">
             <div class="grid grid__number_wide1">
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"0"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" >
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="13363991169447556597">
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"S"</div>
                      <div>"v"</div>
                      <div>"g"</div>
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>"\n"</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 0, column: 0"</div>
          </div>
       </div>
    </div>
        );

    let target_dom: Node<()> = node!(
    <div class="app">
       <h1>"Lines"</h1>
       <div>
          <div class="grid__wrapper">
             <div class="grid grid__number_wide1">
                <div class="grid__number__line" >
                   <div class="grid__number">"0"</div>
                   <div class="grid__line">
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="623356695095054844">
                   <div class="grid__number">"1"</div>
                   <div class="grid__line">
                      <div>"C"</div>
                      <div>"J"</div>
                      <div>"K"</div>
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" >
                   <div class="grid__number">"2"</div>
                   <div class="grid__line">
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="13363991169447556597">
                   <div class="grid__number">"3"</div>
                   <div class="grid__line">
                      <div>"S"</div>
                      <div>"v"</div>
                      <div>"g"</div>
                      <div>"\n"</div>
                   </div>
                </div>
                <div class="grid__number__line" key="9824372840226575955">
                   <div class="grid__number">"4"</div>
                   <div class="grid__line">
                      <div>"T"</div>
                      <div>"h"</div>
                      <div>"e"</div>
                      <div>"\n"</div>
                   </div>
                </div>
             </div>
             <div class="grid__status">"line: 1, column: 0"</div>
          </div>
       </div>
    </div>
        );

    let patches = diff(&current_dom, &target_dom);
    dbg!(&patches);

    assert_eq!(
        patches,
        vec![
            ChangeText::new(8, "0", "1").into(),
            ChangeText::new(26, "2", "3").into(),
            ChangeText::new(38, "3", "4").into(),
            InsertNode::new(
                Some(&"div"),
                6,
                &div(
                    vec![class("grid__number__line")],
                    vec![
                        div(vec![class("grid__number")], vec![text(0)]),
                        div(
                            vec![class("grid__line")],
                            vec![div(vec![], vec![text("\n")])]
                        ),
                    ]
                )
            )
            .into(),
            InsertNode::new(
                Some(&"div"),
                24,
                &div(
                    vec![class("grid__number__line")],
                    vec![
                        div(vec![class("grid__number")], vec![text(2)]),
                        div(
                            vec![class("grid__line")],
                            vec![div(vec![], vec![text("\n")])]
                        ),
                    ]
                )
            )
            .into(),
            RemoveNode::new(Some(&"div"), 18).into(),
            ChangeText::new(49, "line: 0, column: 0", "line: 1, column: 0")
                .into(),
        ]
    );
}