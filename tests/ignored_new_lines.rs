#![deny(warnings)]
use crate::mt_dom::TreePath;
use sauron::prelude::*;

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
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 0, 0, 0, 0,]),
                &text("1")
            ),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1, 0, 0, 0,]),
                vec![&div(
                    vec![class("grid__number__line")],
                    vec![
                        div(vec![class("grid__number")], vec![text(0)]),
                        div(
                            vec![class("grid__line")],
                            vec![div(vec![], vec![text("\n")])]
                        ),
                    ]
                )]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 0, 2, 0, 0,]),
                &text("3")
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1, 0, 0, 1,]),),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1, 0, 0, 2,]),
                vec![&div(
                    vec![class("grid__number__line")],
                    vec![
                        div(vec![class("grid__number")], vec![text(2)]),
                        div(
                            vec![class("grid__line")],
                            vec![div(vec![], vec![text("\n")])]
                        ),
                    ]
                )]
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 0, 3, 0, 0,]),
                &text("4")
            ),
            Patch::replace_node(
                None,
                TreePath::new(vec![1, 0, 1, 0,]),
                &text("line: 1, column: 0")
            ),
        ]
    );
}
