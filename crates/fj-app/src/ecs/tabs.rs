use bevy::prelude::*;

enum view_tab {
    Welcome(WelcomeTab),
    Model(ModelTab),
    Settings(SettingsTab),
}
