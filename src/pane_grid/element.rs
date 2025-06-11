use cosmic::iced_core::event::{self, Event};
use cosmic::iced_core::id::Id;
use cosmic::iced_core::layout;
use cosmic::iced_core::mouse;
use cosmic::iced_core::overlay;
use cosmic::iced_core::renderer;
use cosmic::iced_core::widget;
use cosmic::iced_core::widget::tree::{self, Tree};
use cosmic::iced_core::{
    Clipboard, Layout, Length, Rectangle, Shell, Size, Vector,
    Widget,
};
//use cosmic::iced::widget::container::Catalog;
use crate::pane_grid::container::Catalog;

//use crate::pane_grid::{self, Catalog, Pane, PaneGrid};


use std::borrow::{Borrow, BorrowMut};

/// A generic [`Widget`].
///
/// It is useful to build composable user interfaces that do not leak
/// implementation details in their __view logic__.
///
/// If you have a [built-in widget], you should be able to use `Into<Element>`
/// to turn it into an [`Element`].
///
/// [built-in widget]: crate::widget
#[allow(missing_debug_implementations)]
pub struct Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer + 'a,
{
    widget: crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer> 
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    /// Creates a new [`Element`] containing the given [`Widget`].
    pub fn new(widget: crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>) -> Self
    where
        Theme: Catalog + 'a,
        Renderer: cosmic::iced_core::Renderer,
    {
        Self {
            widget: widget.into(),
        }
    }

    /// Returns a reference to the [`Widget`] of the [`Element`],
    pub fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer> {
        &self.widget
    }

    /// Returns a mutable reference to the [`Widget`] of the [`Element`],
    pub fn as_widget_mut(
        &mut self,
    ) -> &mut dyn Widget<Message, Theme, Renderer> {
        &mut self.widget
    }

    pub fn as_pane_grid(&self) -> &crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer> {
        &self.widget
    }

    pub fn as_pane_grid_mut(&mut self,) -> &mut crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer> {
        &mut self.widget
    }

    /*

    /// Applies a transformation to the produced message of the [`Element`].
    ///
    /// This method is useful when you want to decouple different parts of your
    /// UI and make them __composable__.
    ///
    /// # Example
    /// Imagine we want to use [our counter](index.html#usage). But instead of
    /// showing a single counter, we want to display many of them. We can reuse
    /// the `Counter` type as it is!
    ///
    /// We use composition to model the __state__ of our new application:
    ///
    /// ```
    /// # mod counter {
    /// #     pub struct Counter;
    /// # }
    /// use counter::Counter;
    ///
    /// struct ManyCounters {
    ///     counters: Vec<Counter>,
    /// }
    /// ```
    ///
    /// We can store the state of multiple counters now. However, the
    /// __messages__ we implemented before describe the user interactions
    /// of a __single__ counter. Right now, we need to also identify which
    /// counter is receiving user interactions. Can we use composition again?
    /// Yes.
    ///
    /// ```
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// # }
    /// #[derive(Debug, Clone, Copy)]
    /// pub enum Message {
    ///     Counter(usize, counter::Message)
    /// }
    /// ```
    ///
    /// We compose the previous __messages__ with the index of the counter
    /// producing them. Let's implement our __view logic__ now:
    ///
    /// ```no_run
    /// # mod iced {
    /// #     pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
    /// #
    /// #     pub mod widget {
    /// #         pub fn row<'a, Message>(iter: impl IntoIterator<Item = super::Element<'a, Message>>) -> super::Element<'a, Message> {
    /// #             unimplemented!()
    /// #         }
    /// #     }
    /// # }
    /// #
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// #     pub struct Counter;
    /// #
    /// #     pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
    /// #
    /// #     impl Counter {
    /// #         pub fn view(&self) -> Element<Message> {
    /// #             unimplemented!()
    /// #         }
    /// #     }
    /// # }
    /// #
    /// use counter::Counter;
    ///
    /// use iced::widget::row;
    /// use iced::Element;
    ///
    /// struct ManyCounters {
    ///     counters: Vec<Counter>,
    /// }
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// pub enum Message {
    ///     Counter(usize, counter::Message),
    /// }
    ///
    /// impl ManyCounters {
    ///     pub fn view(&self) -> Element<Message> {
    ///         // We can quickly populate a `row` by mapping our counters
    ///         row(
    ///             self.counters
    ///                 .iter()
    ///                 .map(Counter::view)
    ///                 .enumerate()
    ///                 .map(|(index, counter)| {
    ///                     // Here we turn our `Element<counter::Message>` into
    ///                     // an `Element<Message>` by combining the `index` and the
    ///                     // message of the `element`.
    ///                     counter.map(move |message| Message::Counter(index, message))
    ///                 }),
    ///         )
    ///         .into()
    ///     }
    /// }
    /// ```
    ///
    /// Finally, our __update logic__ is pretty straightforward: simple
    /// delegation.
    ///
    /// ```
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// #     pub struct Counter;
    /// #
    /// #     impl Counter {
    /// #         pub fn update(&mut self, _message: Message) {}
    /// #     }
    /// # }
    /// #
    /// # use counter::Counter;
    /// #
    /// # struct ManyCounters {
    /// #     counters: Vec<Counter>,
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy)]
    /// # pub enum Message {
    /// #    Counter(usize, counter::Message)
    /// # }
    /// impl ManyCounters {
    ///     pub fn update(&mut self, message: Message) {
    ///         match message {
    ///             Message::Counter(index, counter_msg) => {
    ///                 if let Some(counter) = self.counters.get_mut(index) {
    ///                     counter.update(counter_msg);
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }

    pub fn map<B>(
        self,
        f: impl Fn(Message) -> B + 'a,
    ) -> Element<'a, B, Theme, Renderer>
    where
        Message: 'a,
        Theme: 'a,
        Renderer: cosmic::iced_core::Renderer + 'a,
        B: 'a,
    {
        Element::new(Map::new(Box::new(self.widget), f))
    }
    */
    /*
    /// Marks the [`Element`] as _to-be-explained_.
    ///
    /// The [`Renderer`] will explain the layout of the [`Element`] graphically.
    /// This can be very useful for debugging your layout!
    ///
    /// [`Renderer`]: crate::Renderer
    pub fn explain<C: Into<Color>>(
        self,
        color: C,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Message: 'a,
        Theme: Catalog + crate::pane_grid::Catalog + 'a,
        Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
    {
        Element {
            widget: Explain::new(self, color.into()),
        }
    }
    */

}

impl<'a, Message, Theme, Renderer>
    Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow()
    }
}

impl<'a, Message, Theme, Renderer>
    Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for &Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow()
    }
}

impl<'a, Message, Theme, Renderer>
    Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for &mut Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow()
    }
}

impl<'a, Message, Theme, Renderer>
    BorrowMut<dyn Widget<Message, Theme, Renderer> + 'a>
    for &mut Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn borrow_mut(
        &mut self,
    ) -> &mut (dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow_mut()
    }
}

impl<'a, Message, Theme, Renderer>
    BorrowMut<dyn Widget<Message, Theme, Renderer> + 'a>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn borrow_mut(
        &mut self,
    ) -> &mut (dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow_mut()
    }
}

struct Map<'a, A, B, Theme, Renderer> 
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, A, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, A, Theme>> + 'a,
{
    widget: Box<dyn Widget<A, Theme, Renderer> + 'a>,
    mapper: Box<dyn Fn(A) -> B + 'a>,
}

impl<'a, A, B, Theme, Renderer> Map<'a, A, B, Theme, Renderer> 
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, A, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, A, Theme>> + 'a,
{
    pub fn new<F>(
        widget: Box<dyn Widget<A, Theme, Renderer> + 'a>,
        mapper: F,
    ) -> Map<'a, A, B, Theme, Renderer>
    where
        F: 'a + Fn(A) -> B,
    {
        Map {
            widget,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, A, B, Theme, Renderer> Widget<B, Theme, Renderer>
    for Map<'a, A, B, Theme, Renderer>
//where
//    Renderer: cosmic::iced_core::Renderer + 'a,
//    A: 'a,
//    B: 'a,
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, A, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, A, Theme>> + 'a,
{
    fn tag(&self) -> tree::Tag {
        self.widget.tag()
    }

    fn state(&self) -> tree::State {
        self.widget.state()
    }

    fn children(&self) -> Vec<Tree> {
        self.widget.children()
    }

    fn diff(&mut self, tree: &mut Tree) {
        self.widget.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.widget.size_hint()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.widget.layout(tree, renderer, limits)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.widget.operate(tree, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, B>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut local_messages = Vec::new();
        let mut local_shell = Shell::new(&mut local_messages);

        let status = self.widget.on_event(
            tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            &mut local_shell,
            viewport,
        );

        shell.merge(local_shell, &self.mapper);

        status
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.widget
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.widget
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, B, Theme, Renderer>> {
        let mapper = &self.mapper;

        self.widget
            .overlay(tree, layout, renderer, translation)
            .map(move |overlay| overlay.map(mapper))
    }

    fn id(&self) -> Option<Id> {
        self.widget.id()
    }

    fn set_id(&mut self, id: Id) {
        self.widget.set_id(id);
    }

    fn drag_destinations(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        dnd_rectangles: &mut cosmic::iced_core::clipboard::DndDestinationRectangles,
    ) {
        self.widget
            .drag_destinations(state, layout, renderer, dnd_rectangles);
    }
}

/*
struct Explain<'a, Message, Theme, Renderer: cosmic::iced_core::Renderer> 
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    element: Element<'a, Message, Theme, Renderer>,
    color: Color,
}

impl<'a, Message, Theme, Renderer> Explain<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn new(
        element: Element<'a, Message, Theme, Renderer>,
        color: Color,
    ) -> Self {
        Explain { element, color }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Explain<'a, Message, Theme, Renderer>
where
    Theme: Catalog + crate::pane_grid::Catalog + 'a,
    Renderer: cosmic::iced_core::Renderer, crate::pane_grid::PaneGrid<'a, Message, Theme, Renderer>: std::convert::From<crate::pane_grid::PaneGrid<'a, Message, Theme>> + 'a,
{
    fn size(&self) -> Size<Length> {
        self.element.widget.size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.element.widget.size_hint()
    }

    fn tag(&self) -> tree::Tag {
        self.element.widget.tag()
    }

    fn state(&self) -> tree::State {
        self.element.widget.state()
    }

    fn children(&self) -> Vec<Tree> {
        self.element.widget.children()
    }

    fn diff(&mut self, tree: &mut Tree) {
        self.element.widget.diff(tree);
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.element.widget.layout(tree, renderer, limits)
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.element
            .widget
            .operate(state, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.element.widget.on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        fn explain_layout<Renderer: cosmic::iced_core::Renderer>(
            renderer: &mut Renderer,
            color: Color,
            layout: Layout<'_>,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        color,
                        width: 1.0,
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );

            for child in layout.children() {
                explain_layout(renderer, color, child);
            }
        }

        self.element
            .widget
            .draw(state, renderer, theme, style, layout, cursor, viewport);

        explain_layout(renderer, self.color, layout);
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element
            .widget
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.element
            .widget
            .overlay(state, layout, renderer, translation)
    }

    fn id(&self) -> Option<Id> {
        self.element.widget.id()
    }

    fn set_id(&mut self, id: Id) {
        self.element.widget.set_id(id);
    }

    fn drag_destinations(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        dnd_rectangles: &mut cosmic::iced_core::clipboard::DndDestinationRectangles,
    ) {
        self.element.widget.drag_destinations(
            state,
            layout,
            renderer,
            dnd_rectangles,
        );
    }
    // TODO maybe a11y_nodes
}
*/