// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::BaselinePosition;
use crate::Box;
use crate::Buildable;
use crate::Container;
use crate::Orientable;
use crate::Orientation;
use crate::RecentChooser;
use crate::RecentFilter;
use crate::RecentManager;
use crate::RecentSortType;
use crate::ResizeMode;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkRecentChooserWidget")]
    pub struct RecentChooserWidget(Object<ffi::GtkRecentChooserWidget, ffi::GtkRecentChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, RecentChooser;

    match fn {
        type_ => || ffi::gtk_recent_chooser_widget_get_type(),
    }
}

impl RecentChooserWidget {
    pub const NONE: Option<&'static RecentChooserWidget> = None;

    #[doc(alias = "gtk_recent_chooser_widget_new")]
    pub fn new() -> RecentChooserWidget {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_recent_chooser_widget_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_recent_chooser_widget_new_for_manager")]
    #[doc(alias = "new_for_manager")]
    pub fn for_manager(manager: &impl IsA<RecentManager>) -> RecentChooserWidget {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_recent_chooser_widget_new_for_manager(
                manager.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`RecentChooserWidget`] objects.
    ///
    /// This method returns an instance of [`RecentChooserWidgetBuilder`](crate::builders::RecentChooserWidgetBuilder) which can be used to create [`RecentChooserWidget`] objects.
    pub fn builder() -> RecentChooserWidgetBuilder {
        RecentChooserWidgetBuilder::default()
    }
}

impl Default for RecentChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`RecentChooserWidget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct RecentChooserWidgetBuilder {
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
    filter: Option<RecentFilter>,
    limit: Option<i32>,
    local_only: Option<bool>,
    recent_manager: Option<RecentManager>,
    select_multiple: Option<bool>,
    show_icons: Option<bool>,
    show_not_found: Option<bool>,
    show_private: Option<bool>,
    show_tips: Option<bool>,
    sort_type: Option<RecentSortType>,
}

impl RecentChooserWidgetBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`RecentChooserWidgetBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`RecentChooserWidget`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> RecentChooserWidget {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref limit) = self.limit {
            properties.push(("limit", limit));
        }
        if let Some(ref local_only) = self.local_only {
            properties.push(("local-only", local_only));
        }
        if let Some(ref recent_manager) = self.recent_manager {
            properties.push(("recent-manager", recent_manager));
        }
        if let Some(ref select_multiple) = self.select_multiple {
            properties.push(("select-multiple", select_multiple));
        }
        if let Some(ref show_icons) = self.show_icons {
            properties.push(("show-icons", show_icons));
        }
        if let Some(ref show_not_found) = self.show_not_found {
            properties.push(("show-not-found", show_not_found));
        }
        if let Some(ref show_private) = self.show_private {
            properties.push(("show-private", show_private));
        }
        if let Some(ref show_tips) = self.show_tips {
            properties.push(("show-tips", show_tips));
        }
        if let Some(ref sort_type) = self.sort_type {
            properties.push(("sort-type", sort_type));
        }
        glib::Object::new::<RecentChooserWidget>(&properties)
            .expect("Failed to create an instance of RecentChooserWidget")
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &impl IsA<Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &impl IsA<Container>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn filter(mut self, filter: &RecentFilter) -> Self {
        self.filter = Some(filter.clone());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn local_only(mut self, local_only: bool) -> Self {
        self.local_only = Some(local_only);
        self
    }

    pub fn recent_manager(mut self, recent_manager: &impl IsA<RecentManager>) -> Self {
        self.recent_manager = Some(recent_manager.clone().upcast());
        self
    }

    pub fn select_multiple(mut self, select_multiple: bool) -> Self {
        self.select_multiple = Some(select_multiple);
        self
    }

    pub fn show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = Some(show_icons);
        self
    }

    pub fn show_not_found(mut self, show_not_found: bool) -> Self {
        self.show_not_found = Some(show_not_found);
        self
    }

    pub fn show_private(mut self, show_private: bool) -> Self {
        self.show_private = Some(show_private);
        self
    }

    pub fn show_tips(mut self, show_tips: bool) -> Self {
        self.show_tips = Some(show_tips);
        self
    }

    pub fn sort_type(mut self, sort_type: RecentSortType) -> Self {
        self.sort_type = Some(sort_type);
        self
    }
}

impl fmt::Display for RecentChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RecentChooserWidget")
    }
}
