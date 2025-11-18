use std::ffi::OsStr;
use std::path::Path;
use std::process::Stdio;

use derive_new::new;
use tokio::process;

use crate::{CommandMultiplexer, Handle};

/// A [Command] which spawns within a [CommandMultiplexer]'s scope
#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Command<'a> {
    cmux: &'a mut CommandMultiplexer,
    handle: Handle,
    inner: process::Command,
}

impl<'a> Command<'a> {
    /// Spawn this command, providing a [Handle]
    pub fn spawn(self) -> std::io::Result<Handle> {
        self.cmux.spawn_inner(self.handle, self.inner)
    }

    /// Similar to [tokio::process::Command::arg]
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command<'a> {
        self.inner.arg(arg);
        self
    }

    /// Similar to [tokio::process::Command::args]
    pub fn args<I, S>(&mut self, args: I) -> &mut Command<'a>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    /// Similar to [tokio::process::Command::env]
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command<'a>
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.inner.env(key, val);
        self
    }

    /// Similar to [tokio::process::Command::envs]
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Command<'a>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.inner.envs(vars);
        self
    }

    /// Similar to [tokio::process::Command::env_remove]
    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Command<'a> {
        self.inner.env_remove(key);
        self
    }

    /// Similar to [tokio::process::Command::env_clear]
    pub fn env_clear(&mut self) -> &mut Command<'a> {
        self.inner.env_clear();
        self
    }

    /// Similar to [tokio::process::Command::current_dir]
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command<'a> {
        self.inner.current_dir(dir);
        self
    }

    /// Similar to [tokio::process::Command::stdin]
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command<'a> {
        self.inner.stdin(cfg);
        self
    }

    /// Similar to [tokio::process::Command::stdout]
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command<'a> {
        self.inner.stdout(cfg);
        self
    }

    /// Similar to [tokio::process::Command::stderr]
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command<'a> {
        self.inner.stderr(cfg);
        self
    }

    /// Similar to [tokio::process::Command::kill_on_drop]
    pub fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Command<'a> {
        self.inner.kill_on_drop(kill_on_drop);
        self
    }

    /// Similar to [tokio::process::Command::uid]
    pub fn uid(&mut self, id: u32) -> &mut Command<'a> {
        self.inner.uid(id);
        self
    }

    /// Similar to [tokio::process::Command::gid]
    pub fn gid(&mut self, id: u32) -> &mut Command<'a> {
        self.inner.gid(id);
        self
    }

    /// Similar to [tokio::process::Command::arg0]
    pub fn arg0<S>(&mut self, arg: S) -> &mut Command<'a>
    where
        S: AsRef<OsStr>,
    {
        self.inner.arg0(arg);
        self
    }

    /// Similar to [tokio::process::Command::pre_exec]
    ///
    /// # Safety
    ///
    /// The safety requirements are identical to [tokio::process::Command::pre_exec]
    pub unsafe fn pre_exec<F>(&mut self, f: F) -> &mut Command<'a>
    where
        F: FnMut() -> std::io::Result<()> + Send + Sync + 'static,
    {
        unsafe { self.inner.pre_exec(f) };
        self
    }

    /// Similar to [tokio::process::Command::process_group]
    pub fn process_group(&mut self, pgroup: i32) -> &mut Command<'a> {
        self.inner.process_group(pgroup);
        self
    }

    /// Similar to [tokio::process::Command::get_kill_on_drop]
    pub fn get_kill_on_drop(&self) -> bool {
        self.inner.get_kill_on_drop()
    }
}
