-- Add up migration script here
create table if not exists users(
    id bigserial primary key,
    username text unique not null,
    email text unique not null,
    password text not null
);
