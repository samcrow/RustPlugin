
pub struct PluginInfo<'a, 'b, 'c> {
    pub name: &'a str,
    pub signature: &'b str,
    pub description: &'c str,
}

pub trait Plugin: Sized {
    /// Does initialization
    /// On success, returns a plugin
    fn start() -> Option<Self>;
    /// Called when the plugin is enabled
    fn enable(&mut self);
    /// Called when the plugin is disabled
    fn disable(&mut self);

    /// Returns information on this plugin
    fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c>;

    // Called when the plugin is stopped
    fn stop(&mut self);
}
