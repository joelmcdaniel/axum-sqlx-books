-- Create Books Table
CREATE TABLE IF NOT EXISTS books (
    id        SERIAL         PRIMARY KEY,
    title     VARCHAR(255)   NOT NULL,
    author    VARCHAR(255)   NOT NULL,
    year      SMALLINT       NOT NULL
);

-- Insert fake data
INSERT INTO books (title, author, year) VALUES
    ('Embedded Software with Rust', 'David Cabanis', 2027),
    ('The Rust Programming Language, 3rd Edition', 'Steve Klabnik and Carol Nichols', 2026),
    ('Rust Web Programming, 3rd Edition', 'Maxwell Flitton', 2026),
    ('The Rust Programming Handbook', 'Francesco Ciulla', 2025),
    ('Write Powerful Rust Macros', 'Sam Van Overmeire', 2024),
    ('Rust Servers, Services, and Apps', 'Prabhu Eshwarla', 2023),
    ('Rust in Action', 'Tim McNamara', 2021),
    ('Rust for Rustaceans', 'Jon Gjengset', 2021),
    ('Programming Rust', 'Jim Blandy and Jason Orendorff', 2021),
    ('Rust for the IoT', 'Joseph Faisal Nusairat ', 2020);
