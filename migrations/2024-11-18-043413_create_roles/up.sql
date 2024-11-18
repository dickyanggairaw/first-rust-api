-- Your SQL goes here
CREATE TABLE role (
    id SERIAL PRIMARY KEY,         -- Auto Increment dan Primary Key
    role_name VARCHAR(50) NOT NULL,     -- Nama peran
    description VARCHAR(255),           -- Deskripsi
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Waktu pembuatan
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  -- Waktu pembaruan
);