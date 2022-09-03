-- Your SQL goes here
CREATE TABLE posts (
    id serial primary key,
    title varchar(1024) not null,
    content text not null,
    url varchar(1024) not null unique,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);