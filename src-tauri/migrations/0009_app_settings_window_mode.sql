CREATE TABLE IF NOT EXISTS app_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    window_mode TEXT NOT NULL DEFAULT 'fullscreen'
        CHECK (window_mode IN ('fullscreen', 'maximized', 'windowed')),
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO app_settings (id, window_mode) VALUES (1, 'fullscreen');
