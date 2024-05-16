use crate::ui;

pub struct Focus {
    nodes: Vec<ui::NodeId>,
    current: Option<usize>,
}

impl Focus {
    pub fn new<Node>(nodes: impl IntoIterator<Item = Node>) -> Self
    where
        Node: Into<ui::NodeId>,
    {
        let nodes: Vec<_> = nodes.into_iter().map(|v| v.into()).collect();
        if nodes.len() == 0 {
            println!("ui warning: created KeyboardAccessible with length of 0");
        }
        Self {
            nodes,
            current: None,
        }
    }
    fn initialize_inner(&mut self, dom: &mut ui::Dom) {
        let current = 0;
        let Some(element) = dom.select_mut(self.nodes[current]) else {
            println!("ui warning: got None when cycling KeyboardAccessible");
            return;
        };
        element.set_focused(true);
        self.current = Some(current);
    }
    fn set_focused_node(&mut self, dom: &mut ui::Dom, focused: bool) {
        let Some(current) = self.current else {
            unreachable!()
        };
        let Some(element) = dom.select_mut(self.nodes[current]) else {
            println!("ui warning: got None when cycling KeyboardAccessible");
            return;
        };
        element.set_focused(focused);
    }
    fn step<F>(&mut self, dom: &mut ui::Dom, step_current: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        if self.nodes.len() == 0 {
            return;
        }
        let Some(current) = self.current else {
            self.initialize_inner(dom);
            return;
        };
        if dom.select(self.nodes[current]).is_some_and(|n| !n.visible) {
            return self.step(dom, step_current);
        }
        self.set_focused_node(dom, false);
        self.current = Some(step_current(current, self.nodes.len()));
        self.set_focused_node(dom, true);
    }
    pub fn update(&mut self, dom: &mut ui::Dom, ctx: &mut engine::Context) {
        if ctx.key_just_pressed(engine::Keycode::Tab) {
            if ctx.key_pressed(engine::Keycode::LShift) {
                self.previous(dom)
            } else {
                self.next(dom)
            }
        }
    }
    fn previous(&mut self, dom: &mut ui::Dom) {
        let step_current = |current, length| {
            if current == 0 {
                length - 1
            } else {
                current - 1
            }
        };
        self.step(dom, step_current);
    }
    fn next(&mut self, dom: &mut ui::Dom) {
        let step_current = |current, length| (current + 1) % length;
        self.step(dom, step_current);
    }
    pub fn focused_node(&self) -> Option<ui::NodeId> {
        let current = self.current?;
        Some(self.nodes[current])
    }
}
