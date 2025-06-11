use std::collections::{BTreeMap, HashMap};
use std::{
    borrow::Cow,
    //sync::atomic::{AtomicU64, Ordering},
};

use cosmic::iced::{
    clipboard::{
        dnd::{self, DndAction, DndDestinationRectangle, DndEvent, OfferEvent},
        mime::AllowedMimeTypes,
    },
    event,
    //id::Internal,
    mouse, 
    //overlay,
    //widget::{
    //    container,
        //pane_grid::{self, Catalog, Contents, Content, PaneGrid, Pane}
    //},
    Event, Length, Rectangle,
};
use cosmic::iced::id::Id;
use cosmic::iced_core::{
    self, layout,
    widget::{tree, Tree},
    Clipboard, Layout, Shell, Widget,
};
use cosmic::widget::{
    dnd_destination::DragId,
    segmented_button,
};

use crate::app::PaneType;
use crate::pane_grid::{self, Catalog, Pane, PaneGrid};

type TabModel = segmented_button::Model<segmented_button::SingleSelect>;

pub struct CommanderPaneGrid {
    pub panestates: pane_grid::State<TabModel>,
    pub panes_created: usize,
    pub focus: pane_grid::Pane,
    pub panes: Vec<pane_grid::Pane>,
    pub splits: Vec<pane_grid::Split>,
    pub drag_id_by_pane: BTreeMap<pane_grid::Pane, DragId>,
    pub entity_by_pane: BTreeMap<pane_grid::Pane, segmented_button::Entity>,
    pub entity_by_type: BTreeMap<PaneType, segmented_button::Entity>,
    pub pane_by_entity: BTreeMap<segmented_button::Entity, pane_grid::Pane>,
    pub pane_by_type: BTreeMap<PaneType, pane_grid::Pane>,
    pub type_by_entity: BTreeMap<segmented_button::Entity, PaneType>,
    pub type_by_pane: BTreeMap<pane_grid::Pane, PaneType>,
    pub first_pane: pane_grid::Pane,
    pub drag_pane: Option<pane_grid::Pane>,
    pub drag_id: Option<DragId>,
}

impl CommanderPaneGrid {
    pub fn new(model: TabModel, drag_id: DragId) -> Self {
        let (panestates, pane) = pane_grid::State::new(model);
        let mut terminal_ids = HashMap::new();
        terminal_ids.insert(pane, cosmic::widget::Id::unique());
        let mut v = Self {
            panestates,
            panes_created: 1,
            focus: pane,
            panes: vec![pane],
            splits: Vec::new(),
            drag_id_by_pane: BTreeMap::new(),
            entity_by_pane: BTreeMap::new(),
            entity_by_type: BTreeMap::new(),
            pane_by_entity: BTreeMap::new(),
            pane_by_type: BTreeMap::new(),
            type_by_entity: BTreeMap::new(),
            type_by_pane: BTreeMap::new(),
            first_pane: pane,
            drag_pane: None,
            drag_id: None,
        };
        v.drag_id_by_pane.insert(pane, drag_id);
        v.pane_by_type.insert(PaneType::LeftPane, pane);
        v.type_by_pane.insert(pane, PaneType::LeftPane);
        let entity;
        if let Some(tab_model) = v.active() {
            entity = tab_model.active();
        } else {
            return v;
        }
        v.entity_by_pane.insert(v.focus, entity);
        v.entity_by_type.insert(PaneType::LeftPane, entity);
        v.pane_by_entity.insert(entity, v.focus);
        v.type_by_entity.insert(entity, PaneType::LeftPane);

        v
    }
    pub fn active(&self) -> Option<&TabModel> {
        self.panestates.get(self.focus)
    }
    pub fn active_mut(&mut self) -> Option<&mut TabModel> {
        self.panestates.get_mut(self.focus)
    }

    pub fn insert(
        &mut self,
        pane_type: PaneType,
        pane: pane_grid::Pane,
        split: pane_grid::Split,
        drag_id: DragId,
    ) {
        if let Some(tab_model) = self.active_mut() {
            let title = match pane_type {
                PaneType::ButtonPane => "ButtonPane".to_string(),
                PaneType::TerminalPane => "TerminalPane".to_string(),
                PaneType::LeftPane => "LeftPane".to_string(),
                PaneType::RightPane => "RightPane".to_string(),
            };
            let entity = tab_model
                .insert()
                .text(title)
                //.closable()
                //.activate()
                .id();
            self.panes.push(pane);
            self.splits.push(split);
            self.focus = pane;
            self.drag_id_by_pane.insert(pane, drag_id);
            self.pane_by_type.insert(pane_type, pane);
            self.type_by_pane.insert(pane, pane_type);
            self.entity_by_pane.insert(pane, entity);
            self.entity_by_type.insert(pane_type, entity);
            self.pane_by_entity.insert(entity, pane);
            self.type_by_entity.insert(entity, pane_type);
        }
    }

    pub fn set_focus(&mut self, pane_type: PaneType) {
        if !self.pane_by_type.contains_key(&pane_type) {
            return;
        }
        let pane = self.pane_by_type[&pane_type];
        match pane_type {
            PaneType::ButtonPane => {
                let pane = self.pane_by_type[&PaneType::LeftPane];
                self.focus = pane;
            }
            PaneType::TerminalPane => self.focus = pane,
            PaneType::LeftPane => self.focus = pane,
            PaneType::RightPane => self.focus = pane,
        };
    }

    pub fn focussed(&self) -> PaneType {
        return self.type_by_pane[&self.focus];
    }

    pub fn drop_target(&self, drag_id: DragId) -> PaneType {
        for p in self.panes.iter() {
            if self.drag_id_by_pane[p] == drag_id {
                return self.type_by_pane[p];
            }
        }
        PaneType::LeftPane
    }
}

pub struct CommanderDndDestination<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: cosmic::iced_core::Renderer,
{
    id: Id,
    drag_id: Option<DragId>,
    preferred_action: DndAction,
    action: DndAction,
    //container: PaneGrid<'a, Message, Theme, Renderer>,
    container: pane_grid::element::Element<'a, Message, Theme, Renderer>,
    mime_types: Vec<Cow<'static, str>>,
    forward_drag_as_cursor: bool,
    on_hold: Option<Box<dyn Fn(f64, f64) -> Message>>,
    on_drop: Option<Box<dyn Fn(f64, f64) -> Message + 'static>>,
    on_enter: Option<Box<dyn Fn(f64, f64, Vec<String>) -> Message>>,
    on_leave: Option<Box<dyn Fn() -> Message>>,
    on_motion: Option<Box<dyn Fn(f64, f64) -> Message>>,
    on_action_selected: Option<Box<dyn Fn(DndAction) -> Message>>,
    on_data_received: Option<Box<dyn Fn(String, Vec<u8>) -> Message>>,
    on_finish: Option<Box<dyn Fn(String, Vec<u8>, DndAction, f64, f64) -> Message>>,
    //pub dnd_state: cosmic::widget::dnd_destination::State<Option<Pane>>,
    pub panes: Vec<Pane>,
    pub drag_id_by_pane: std::collections::BTreeMap<Pane, DragId>,
    pub dnd_pane: Option<Pane>,
    pub dnd_pane_id: Option<cosmic::widget::dnd_destination::DragId>,
    pub dnd_action: Option<DndAction>,
    pub dnd_pos_x: f64,
    pub dnd_pos_y: f64,
    pub mimetypes: Vec<String>,
}

impl<'a, Message: 'a, Theme, Renderer> CommanderDndDestination<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    pub fn new(child: PaneGrid<'a, Message, Theme, Renderer>, mimes: Vec<Cow<'static, str>>) -> Self
    where
        Theme: Catalog,
        Renderer: cosmic::iced_core::Renderer,
    {
        Self {
            id: Id::unique(),
            drag_id: None,
            mime_types: mimes,
            preferred_action: DndAction::Move,
            action: DndAction::Copy | DndAction::Move,
            //container: child,
            container: pane_grid::element::Element::new(child),
            forward_drag_as_cursor: false,
            on_hold: None,
            on_drop: None,
            on_enter: None,
            on_leave: None,
            on_motion: None,
            on_action_selected: None,
            on_data_received: None,
            on_finish: None,
            panes: Vec::new(),
            drag_id_by_pane: std::collections::BTreeMap::new(),
            dnd_pane: None,
            dnd_pane_id: None,
            dnd_action: None,
            dnd_pos_x: 0.0,
            dnd_pos_y: 0.0,
            mimetypes: Vec::new(),
        }
    }

    pub fn for_data<T: AllowedMimeTypes>(child: PaneGrid<'a, Message, Theme, Renderer>) -> Self
    where
        Renderer: cosmic::iced_core::Renderer,
    {
        Self {
            id: Id::unique(),
            drag_id: None,
            mime_types: T::allowed().iter().cloned().map(Cow::Owned).collect(),
            preferred_action: DndAction::Move,
            action: DndAction::Copy | DndAction::Move,
            container: pane_grid::element::Element::new(child),
            forward_drag_as_cursor: false,
            on_hold: None,
            on_drop: None,
            on_enter: None,
            on_leave: None,
            on_motion: None,
            on_action_selected: None,
            on_data_received: None,
            on_finish: None,
            panes: Vec::new(),
            drag_id_by_pane: std::collections::BTreeMap::new(),
            dnd_pane: None,
            dnd_pane_id: None,
            dnd_action: None,
            dnd_pos_x: 0.0,
            dnd_pos_y: 0.0,
            mimetypes: Vec::new(),
        }
    }

    #[must_use]
    pub fn data_received_for<T: AllowedMimeTypes>(
        mut self,
        f: impl Fn(Option<T>) -> Message + 'static,
    ) -> Self {
        self.on_data_received = Some(Box::new(
            move |mime, data| f(T::try_from((data, mime)).ok()),
        ));
        self
    }

    pub fn with_id(
        child: PaneGrid<'a, Message, Theme, Renderer>,
        id: Id,
        mimes: Vec<Cow<'static, str>>,
    ) -> Self
    where
        Renderer: cosmic::iced_core::Renderer,
    {
        Self {
            id,
            drag_id: None,
            mime_types: mimes,
            preferred_action: DndAction::Move,
            action: DndAction::Copy | DndAction::Move,
            container: pane_grid::element::Element::new(child),
            forward_drag_as_cursor: false,
            on_hold: None,
            on_drop: None,
            on_enter: None,
            on_leave: None,
            on_motion: None,
            on_action_selected: None,
            on_data_received: None,
            on_finish: None,
            panes: Vec::new(),
            drag_id_by_pane: std::collections::BTreeMap::new(),
            dnd_pane: None,
            dnd_pane_id: None,
            dnd_action: None,
            dnd_pos_x: 0.0,
            dnd_pos_y: 0.0,
            mimetypes: Vec::new(),
        }
    }

    pub fn drop_target_from_position(&self, x: f64, y: f64) -> (Pane, DragId) {
        let spacing = 5.0;
        let target = cosmic::iced_core::Point {
            x: x as f32,
            y: y as f32,
        };
        let mut id = DragId::new();
        let limits = cosmic::iced::core::layout::Limits::NONE
            .min_width(1.0)
            .min_height(1.0);
        let wsize = self.container.as_widget().size();
        let window_size = limits.resolve(wsize.width, wsize.height, cosmic::iced_core::Size::ZERO);
        //let pane_grid = self.container.as_widget().
        for (pane, rect) in self
            .container.as_pane_grid()
            .contents
            .layout()
            .pane_regions(spacing, window_size)
        {
            if rect.contains(target) {
                id = self.drag_id_by_pane[&pane];
                return (pane, id);
            }
        }
        (self.panes[0].to_owned(), id)
    }
    
    #[must_use]
    pub fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer>
    where
        Theme: Catalog + crate::pane_grid::Catalog + 'a,
        Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
    {
        self.container.as_widget()
    }

    #[must_use]
    pub fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Theme, Renderer>
    where
        Theme: Catalog + crate::pane_grid::Catalog + 'a,
        Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
    {
        self.container.as_widget_mut()
    }

    #[must_use]
    pub fn drag_id(mut self, id: DragId) -> Self {
        self.drag_id = Some(id);
        self
    }

    #[must_use]
    pub fn action(mut self, action: DndAction) -> Self {
        self.action = action;
        self
    }

    #[must_use]
    pub fn preferred_action(mut self, action: DndAction) -> Self {
        self.preferred_action = action;
        self
    }

    #[must_use]
    pub fn forward_drag_as_cursor(mut self, forward: bool) -> Self {
        self.forward_drag_as_cursor = forward;
        self
    }

    #[must_use]
    pub fn on_hold(mut self, f: impl Fn(f64, f64) -> Message + 'static) -> Self {
        self.on_hold = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn on_drop(mut self, f: impl Fn(f64, f64) -> Message + 'static) -> Self {
        self.on_drop = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn on_enter(mut self, f: impl Fn(f64, f64, Vec<String>) -> Message + 'static) -> Self {
        self.on_enter = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn on_leave(mut self, m: impl Fn() -> Message + 'static) -> Self {
        self.on_leave = Some(Box::new(m));
        self
    }

    #[must_use]
    pub fn on_finish(
        mut self,
        f: impl Fn(String, Vec<u8>, DndAction, f64, f64) -> Message + 'static,
    ) -> Self {
        self.on_finish = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn on_motion(mut self, f: impl Fn(f64, f64) -> Message + 'static) -> Self {
        self.on_motion = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn on_action_selected(mut self, f: impl Fn(DndAction) -> Message + 'static) -> Self {
        self.on_action_selected = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn on_data_received(mut self, f: impl Fn(String, Vec<u8>) -> Message + 'static) -> Self {
        self.on_data_received = Some(Box::new(f));
        self
    }

    /// Returns the drag id of the destination.
    ///
    /// # Panics
    /// Panics if the destination has been assigned a Set id, which is invalid.
    #[must_use]
    pub fn get_drag_id(&self) -> DragId {
        if self.drag_id.is_some() {
            return self.drag_id.unwrap();
        }
        DragId::new()
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for CommanderDndDestination<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn children(&self) -> Vec<Tree> {
        self.container.as_pane_grid().children()
    }

    fn tag(&self) -> iced_core::widget::tree::Tag {
        tree::Tag::of::<State<()>>()
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.children[0].diff(self.container.as_widget_mut());
    }

    fn state(&self) -> iced_core::widget::tree::State {
        tree::State::new(State::<()>::new())
    }

    fn size(&self) -> iced_core::Size<Length> {
        self.container.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.container
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<()>,
    ) {
        self.container
            .as_widget()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    #[allow(clippy::too_many_lines)]
    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let s = self.container.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        if matches!(s, event::Status::Captured) {
            return event::Status::Captured;
        }

        let state = tree.state.downcast_mut::<State<()>>();

        let my_id = self.get_drag_id().0;

        match event {
            Event::Dnd(DndEvent::Offer(
                id,
                OfferEvent::Enter {
                    x, y, mime_types, ..
                },
            )) if id == Some(my_id) => {
                if let Some(msg) = state.on_enter(
                    x,
                    y,
                    mime_types,
                    self.on_enter.as_ref().map(std::convert::AsRef::as_ref),
                    (),
                ) {
                    shell.publish(msg);
                }
                if self.forward_drag_as_cursor {
                    #[allow(clippy::cast_possible_truncation)]
                    let drag_cursor = mouse::Cursor::Available((x as f32, y as f32).into());
                    let event = Event::Mouse(mouse::Event::CursorMoved {
                        position: drag_cursor.position().unwrap(),
                    });
                    self.container.as_widget_mut().on_event(
                        &mut tree.children[0],
                        event,
                        layout,
                        drag_cursor,
                        renderer,
                        clipboard,
                        shell,
                        viewport,
                    );
                }
                return event::Status::Captured;
            }
            Event::Dnd(DndEvent::Offer(id, OfferEvent::Leave)) if id == Some(my_id) => {
                state.on_leave(self.on_leave.as_ref().map(std::convert::AsRef::as_ref));

                if self.forward_drag_as_cursor {
                    let drag_cursor = mouse::Cursor::Unavailable;
                    let event = Event::Mouse(mouse::Event::CursorLeft);
                    self.container.as_widget_mut().on_event(
                        &mut tree.children[0],
                        event,
                        layout,
                        drag_cursor,
                        renderer,
                        clipboard,
                        shell,
                        viewport,
                    );
                }
                return event::Status::Captured;
            }
            Event::Dnd(DndEvent::Offer(id, OfferEvent::Motion { x, y })) if id == Some(my_id) => {
                if let Some(msg) = state.on_motion(
                    x,
                    y,
                    self.on_motion.as_ref().map(std::convert::AsRef::as_ref),
                    self.on_enter.as_ref().map(std::convert::AsRef::as_ref),
                    (),
                ) {
                    shell.publish(msg);
                }

                if self.forward_drag_as_cursor {
                    #[allow(clippy::cast_possible_truncation)]
                    let drag_cursor = mouse::Cursor::Available((x as f32, y as f32).into());
                    let event = Event::Mouse(mouse::Event::CursorMoved {
                        position: drag_cursor.position().unwrap(),
                    });
                    self.container.as_widget_mut().on_event(
                        &mut tree.children[0],
                        event,
                        layout,
                        drag_cursor,
                        renderer,
                        clipboard,
                        shell,
                        viewport,
                    );
                }
                return event::Status::Captured;
            }
            Event::Dnd(DndEvent::Offer(id, OfferEvent::LeaveDestination)) if id == Some(my_id) => {
                if let Some(msg) =
                    state.on_leave(self.on_leave.as_ref().map(std::convert::AsRef::as_ref))
                {
                    shell.publish(msg);
                }
                return event::Status::Captured;
            }
            Event::Dnd(DndEvent::Offer(id, OfferEvent::Drop)) if id == Some(my_id) => {
                if let Some(msg) =
                    state.on_drop(self.on_drop.as_ref().map(std::convert::AsRef::as_ref))
                {
                    shell.publish(msg);
                }
                return event::Status::Captured;
            }
            Event::Dnd(DndEvent::Offer(id, OfferEvent::SelectedAction(action)))
                if id == Some(my_id) =>
            {
                if let Some(msg) = state.on_action_selected(
                    action,
                    self.on_action_selected
                        .as_ref()
                        .map(std::convert::AsRef::as_ref),
                ) {
                    shell.publish(msg);
                }
                return event::Status::Captured;
            }
            Event::Dnd(DndEvent::Offer(id, OfferEvent::Data { data, mime_type }))
                if id == Some(my_id) =>
            {
                dbg!("got data");
                if let (Some(msg), ret) = state.on_data_received(
                    mime_type,
                    data,
                    self.on_data_received
                        .as_ref()
                        .map(std::convert::AsRef::as_ref),
                    self.on_finish.as_ref().map(std::convert::AsRef::as_ref),
                ) {
                    shell.publish(msg);
                    return ret;
                }
                return event::Status::Captured;
            }
            _ => {}
        }
        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.container.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &iced_core::renderer::Style,
        layout: layout::Layout<'_>,
        cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.container.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        _tree: &'b mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _translation: cosmic::iced::Vector,
    ) -> Option<cosmic::iced_core::overlay::Element<'a, Message, Theme, Renderer>> {
        None
    }

    fn drag_destinations(
        &self,
        state: &Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        dnd_rectangles: &mut iced_core::clipboard::DndDestinationRectangles,
    ) {
        let bounds = layout.bounds();
        let my_id = self.get_drag_id();
        let my_dest = DndDestinationRectangle {
            id: my_id.0,
            rectangle: dnd::Rectangle {
                x: f64::from(bounds.x),
                y: f64::from(bounds.y),
                width: f64::from(bounds.width),
                height: f64::from(bounds.height),
            },
            mime_types: self.mime_types.clone(),
            actions: self.action,
            preferred: self.preferred_action,
        };
        dnd_rectangles.push(my_dest);

        self.container.as_widget().drag_destinations(
            &state.children[0],
            layout,
            renderer,
            dnd_rectangles,
        );
    }

    fn id(&self) -> Option<Id> {
        Some(self.id.clone())
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

#[derive(Default)]
pub struct State<T> {
    pub drag_offer: Option<DragOffer<T>>,
}

pub struct DragOffer<T> {
    pub x: f64,
    pub y: f64,
    pub dropped: bool,
    pub selected_action: DndAction,
    pub data: T,
}

impl<T> State<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { drag_offer: None }
    }

    pub fn on_enter<Message>(
        &mut self,
        x: f64,
        y: f64,
        mime_types: Vec<String>,
        on_enter: Option<impl Fn(f64, f64, Vec<String>) -> Message>,
        data: T,
    ) -> Option<Message> {
        self.drag_offer = Some(DragOffer {
            x,
            y,
            dropped: false,
            selected_action: DndAction::empty(),
            data,
        });
        on_enter.map(|f| f(x, y, mime_types))
    }

    pub fn on_leave<Message>(&mut self, on_leave: Option<&dyn Fn() -> Message>) -> Option<Message> {
        if self.drag_offer.as_ref().is_some_and(|d| !d.dropped) {
            self.drag_offer = None;
            on_leave.map(|f| f())
        } else {
            None
        }
    }

    pub fn on_motion<Message>(
        &mut self,
        x: f64,
        y: f64,
        on_motion: Option<impl Fn(f64, f64) -> Message>,
        on_enter: Option<impl Fn(f64, f64, Vec<String>) -> Message>,
        data: T,
    ) -> Option<Message> {
        if let Some(s) = self.drag_offer.as_mut() {
            s.x = x;
            s.y = y;
        } else {
            self.drag_offer = Some(DragOffer {
                x,
                y,
                dropped: false,
                selected_action: DndAction::empty(),
                data,
            });
            if let Some(f) = on_enter {
                return Some(f(x, y, vec![]));
            }
        }
        on_motion.map(|f| f(x, y))
    }

    pub fn on_drop<Message>(
        &mut self,
        on_drop: Option<impl Fn(f64, f64) -> Message>,
    ) -> Option<Message> {
        if let Some(offer) = self.drag_offer.as_mut() {
            offer.dropped = true;
            if let Some(f) = on_drop {
                return Some(f(offer.x, offer.y));
            }
        }
        None
    }

    pub fn on_action_selected<Message>(
        &mut self,
        action: DndAction,
        on_action_selected: Option<impl Fn(DndAction) -> Message>,
    ) -> Option<Message> {
        if let Some(s) = self.drag_offer.as_mut() {
            s.selected_action = action;
        }
        if let Some(f) = on_action_selected {
            f(action).into()
        } else {
            None
        }
    }

    pub fn on_data_received<Message>(
        &mut self,
        mime: String,
        data: Vec<u8>,
        on_data_received: Option<impl Fn(String, Vec<u8>) -> Message>,
        on_finish: Option<impl Fn(String, Vec<u8>, DndAction, f64, f64) -> Message>,
    ) -> (Option<Message>, event::Status) {
        dbg!("data received");
        let Some(dnd) = self.drag_offer.as_ref() else {
            self.drag_offer = None;
            return (None, event::Status::Ignored);
        };

        if dnd.dropped {
            let ret = (
                on_finish.map(|f| f(mime, data, dnd.selected_action, dnd.x, dnd.y)),
                event::Status::Captured,
            );
            self.drag_offer = None;
            ret
        } else if let Some(f) = on_data_received {
            (Some(f(mime, data)), event::Status::Captured)
        } else {
            (None, event::Status::Ignored)
        }
    }
}


impl<'a, Message, Theme, Renderer> From<CommanderDndDestination<'a, Message, Theme, Renderer>>
    for cosmic::iced_core::Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn from(wrapper: CommanderDndDestination<'a, Message, Theme, Renderer>) -> Self {
        cosmic::iced_core::Element::new(wrapper)
    }
}
