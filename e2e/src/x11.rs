use std::error::Error;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, AtomEnum, ClientMessageEvent, ConfigureWindowAux, ConnectionExt, EventMask, MapState, Window};
use x11rb::rust_connection::RustConnection;

type X11Result<T> = Result<T, Box<dyn Error>>;

pub struct X11 {
    conn: RustConnection,
    root: Window,
    net_wm_pid: Atom,
    wm_protocols: Atom,
    wm_delete_window: Atom,
}

impl X11 {
    pub fn connect(display: &str) -> X11Result<Self> {
        let (conn, screen_number) = x11rb::connect(Some(display))?;
        let root = conn.setup().roots[screen_number].root;
        let atom = |name: &[u8]| -> X11Result<Atom> { Ok(conn.intern_atom(false, name)?.reply()?.atom) };
        Ok(Self {
            net_wm_pid: atom(b"_NET_WM_PID")?,
            wm_protocols: atom(b"WM_PROTOCOLS")?,
            wm_delete_window: atom(b"WM_DELETE_WINDOW")?,
            conn,
            root,
        })
    }

    /// Visible top level window owned by the process, if there is one
    pub fn find_window_by_pid(&self, pid: u32) -> X11Result<Option<Window>> {
        for window in self.conn.query_tree(self.root)?.reply()?.children {
            let reply = self.conn.get_property(false, window, self.net_wm_pid, AtomEnum::CARDINAL, 0, 1)?.reply()?;
            let owner = reply.value32().and_then(|mut values| values.next());
            if owner == Some(pid) && self.is_visible(window)? {
                return Ok(Some(window));
            }
        }
        Ok(None)
    }

    pub fn size(&self, window: Window) -> X11Result<(u32, u32)> {
        let geometry = self.conn.get_geometry(window)?.reply()?;
        Ok((geometry.width.into(), geometry.height.into()))
    }

    pub fn resize(&self, window: Window, width: u32, height: u32) -> X11Result<()> {
        self.conn.configure_window(window, &ConfigureWindowAux::new().width(width).height(height))?;
        self.conn.flush()?;
        Ok(())
    }

    /// Same request as clicking the X button, so the app can shut down cleanly and save its settings.
    /// There is no window manager on Xvfb and no CLI tool sends this message without one.
    pub fn request_close(&self, window: Window) -> X11Result<()> {
        let event = ClientMessageEvent::new(32, window, self.wm_protocols, [self.wm_delete_window, 0, 0, 0, 0]);
        self.conn.send_event(false, window, EventMask::NO_EVENT, event)?;
        self.conn.flush()?;
        Ok(())
    }

    fn is_visible(&self, window: Window) -> X11Result<bool> {
        Ok(self.conn.get_window_attributes(window)?.reply()?.map_state == MapState::VIEWABLE)
    }
}
