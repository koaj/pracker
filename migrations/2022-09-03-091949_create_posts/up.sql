-- Your SQL goes here
create table posts (
    id serial primary key,
    title varchar(1024) not null,
    content text not null,
    url varchar(1024) not null unique,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
);