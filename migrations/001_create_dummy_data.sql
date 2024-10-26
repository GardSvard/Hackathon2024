CREATE TABLE IF NOT EXISTS snapshot (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL UNIQUE,
    battery INTEGER NOT NULL,
    solar_panel_wattage REAL NOT NULL,
    city TEXT NOT NULL 
);

INSERT INTO snapshot (date, battery, solar_panel_wattage, city)
    SELECT DATETIME(), 50, 243.6, "Trondheim"
    WHERE NOT EXISTS (SELECT 1 FROM snapshot);