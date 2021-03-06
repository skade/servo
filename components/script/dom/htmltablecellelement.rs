/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::attr::{Attr, AttrValue};
use dom::bindings::codegen::Bindings::HTMLTableCellElementBinding::HTMLTableCellElementMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::LayoutJS;
use dom::document::Document;
use dom::element::{AttributeMutation, Element, RawLayoutElementHelpers};
use dom::htmlelement::HTMLElement;
use dom::htmltablerowelement::HTMLTableRowElement;
use dom::node::Node;
use dom::virtualmethods::VirtualMethods;
use std::cell::Cell;
use string_cache::Atom;
use util::str::{self, DOMString, LengthOrPercentageOrAuto};

const DEFAULT_COLSPAN: u32 = 1;

#[dom_struct]
pub struct HTMLTableCellElement {
    htmlelement: HTMLElement,
    width: Cell<LengthOrPercentageOrAuto>,
}

impl HTMLTableCellElement {
    pub fn new_inherited(tag_name: DOMString,
                         prefix: Option<DOMString>,
                         document: &Document)
                         -> HTMLTableCellElement {
        HTMLTableCellElement {
            htmlelement: HTMLElement::new_inherited(tag_name, prefix, document),
            width: Cell::new(LengthOrPercentageOrAuto::Auto),
        }
    }

    #[inline]
    pub fn htmlelement(&self) -> &HTMLElement {
        &self.htmlelement
    }
}

impl HTMLTableCellElementMethods for HTMLTableCellElement {
    // https://html.spec.whatwg.org/multipage/#dom-tdth-colspan
    make_uint_getter!(ColSpan, "colspan", DEFAULT_COLSPAN);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-colspan
    make_uint_setter!(SetColSpan, "colspan", DEFAULT_COLSPAN);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-bgcolor
    make_getter!(BgColor);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-bgcolor
    make_legacy_color_setter!(SetBgColor, "bgcolor");

    // https://html.spec.whatwg.org/multipage/#dom-tdth-cellindex
    fn CellIndex(&self) -> i32 {
        let self_node = self.upcast::<Node>();

        let parent_children = match self_node.GetParentNode() {
            Some(ref parent_node) if parent_node.is::<HTMLTableRowElement>() => {
                parent_node.children()
            },
            _ => return -1,
        };

        parent_children.filter(|c| c.is::<HTMLTableCellElement>())
                       .position(|c| c.r() == self_node)
                       .map(|p| p as i32).unwrap_or(-1)
    }
}


pub trait HTMLTableCellElementLayoutHelpers {
    fn get_background_color(&self) -> Option<RGBA>;
    fn get_colspan(&self) -> Option<u32>;
    fn get_width(&self) -> LengthOrPercentageOrAuto;
}

#[allow(unsafe_code)]
impl HTMLTableCellElementLayoutHelpers for LayoutJS<HTMLTableCellElement> {
    fn get_background_color(&self) -> Option<RGBA> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(""), &atom!("bgcolor"))
                .and_then(AttrValue::as_color)
                .cloned()
        }
    }

    fn get_colspan(&self) -> Option<u32> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(""), &atom!("colspan"))
                .map(AttrValue::as_uint)
        }
    }

    fn get_width(&self) -> LengthOrPercentageOrAuto {
        unsafe {
            (*self.unsafe_get()).width.get()
        }
    }
}

impl VirtualMethods for HTMLTableCellElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
        match *attr.local_name() {
            atom!(width) => {
                let width = mutation.new_value(attr).map(|value| {
                    str::parse_length(&value)
                });
                self.width.set(width.unwrap_or(LengthOrPercentageOrAuto::Auto));
            },
            _ => {},
        }
    }

    fn parse_plain_attribute(&self, local_name: &Atom, value: DOMString) -> AttrValue {
        match *local_name {
            atom!("colspan") => AttrValue::from_u32(value, DEFAULT_COLSPAN),
            atom!("bgcolor") => AttrValue::from_legacy_color(value),
            _ => self.super_type().unwrap().parse_plain_attribute(local_name, value),
        }
    }
}
