use cosmic::iced::clipboard::dnd::DndAction;
use cosmic::widget::{
    dnd_destination::DragId,
    //pane_grid::{self, Pane, PaneGrid},
    segmented_button,
};
use std::collections::{BTreeMap, HashMap};

use crate::app::PaneType;
use crate::pane_grid;

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
    pub mimes: Vec<String>,
    pub first_pane: pane_grid::Pane,
    pub _drag_pane: Option<pane_grid::Pane>,
    pub _drag_id: Option<DragId>,
    pub dnd_pane: Option<pane_grid::Pane>,
    pub dnd_pane_id: Option<DragId>,
    pub dnd_action: Option<DndAction>,
    pub dnd_pos_x: f64,
    pub dnd_pos_y: f64,
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
            mimes: Vec::new(),
            first_pane: pane,
            _drag_pane: None,
            _drag_id: None,
            dnd_pane: None,
            dnd_pane_id: None,
            dnd_action: None,
            dnd_pos_x: 0.0,
            dnd_pos_y: 0.0,
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
            self.panes_created += 1;
            self.drag_id_by_pane.insert(pane, drag_id);
            self.pane_by_type.insert(pane_type, pane);
            self.type_by_pane.insert(pane, pane_type);
            self.entity_by_pane.insert(pane, entity);
            self.entity_by_type.insert(pane_type, entity);
            self.pane_by_entity.insert(entity, pane);
            self.type_by_entity.insert(entity, pane_type);
        }
    }

    pub fn _set_focus(&mut self, pane_type: PaneType) {
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

    pub fn _focussed(&self) -> PaneType {
        return self.type_by_pane[&self.focus];
    }

    pub fn _drop_target(&self, drag_id: DragId) -> PaneType {
        for p in self.panes.iter() {
            if self.drag_id_by_pane[p] == drag_id {
                return self.type_by_pane[p];
            }
        }
        PaneType::LeftPane
    }
}
