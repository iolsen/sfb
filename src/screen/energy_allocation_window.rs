use crate::ship::EnergyAllocation;
use imgui::*;

pub struct EnergyAllocationWindow<'a> {
    alloc: &'a EnergyAllocation
}

impl EnergyAllocationWindow<'_> {
    pub fn new(e: &EnergyAllocation) -> EnergyAllocationWindow {
        EnergyAllocationWindow {
            alloc: e,
        }
    }

    pub fn show<'a>(&self, ui: &Ui<'a>) {
        Window::new(im_str!("Energy Allocation"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            // .opened(opened)
            .build(&ui, || {
                ui.text(format!("Warp Power Available: {}", self.alloc.warp_available));
                ui.text(format!("Impulse Power Available: {}", self.alloc.impulse_available));
                ui.text(format!("Reactor Power Available: {}", self.alloc.reactor_available));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                        ));
            })
    }
}
