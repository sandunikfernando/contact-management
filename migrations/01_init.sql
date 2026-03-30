CREATE TABLE IF NOT EXISTS persons (
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),          
    notes        TEXT                   
);

CREATE TABLE IF NOT EXISTS mobiles (
    id        SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    number    VARCHAR(50) NOT NULL,
    label     VARCHAR(50)               
);

CREATE TABLE IF NOT EXISTS emails (
    id        SERIAL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    address   VARCHAR(255) NOT NULL,
    label     VARCHAR(50)              
);
