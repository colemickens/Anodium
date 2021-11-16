use smithay::{
    reexports::wayland_server::DispatchData, wayland::shell::wlr_layer::LayerShellRequest,
};

use crate::desktop_layout::LayerSurface;

use super::ShellEvent;

impl super::Inner {
    pub fn wlr_layer_shell_request(&mut self, request: LayerShellRequest, ddata: DispatchData) {
        match request {
            LayerShellRequest::NewLayerSurface {
                surface,
                output,
                layer,
                namespace,
            } => {
                let surface = LayerSurface::new(surface, layer);

                (self.cb)(
                    ShellEvent::LayerCreated {
                        surface,
                        output,
                        layer,
                        namespace,
                    },
                    ddata,
                );
            }
            LayerShellRequest::AckConfigure { surface, configure } => {
                (self.cb)(ShellEvent::LayerAckConfigure { surface, configure }, ddata);
            }
        }
    }
}
