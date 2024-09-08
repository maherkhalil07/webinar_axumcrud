CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title TEXT,
    author TEXT
);

INSERT INTO books (title, author) VALUES ('Hands-on Rust', 'Wolverson, Herbert');
INSERT INTO books (title, author) VALUES ('Rust Brain Teasers', 'Wolverson, Herbert');
