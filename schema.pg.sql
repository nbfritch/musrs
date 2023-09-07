create schema library;

create table library.filesystem_artifacts (
    id serial primary key,
    relative_path varchar(256) not null,
    file_name varchar(100) not null,
    file_extension not null,
    is_present bit not null,
    first_path_segment varchar(48) null,
    second_path_segment varchar(48) null,
    created_at timestamp not null default current_timestamp
);

create table library.tag_types (
    id serial primary key,
    tag_type_name varchar(20) not null unique
);

insert into library.tag_types (tag_type_name)
values
('user:track_name'),
('user:genre'),
('user:artist'),
('user:album'),
('id3:track_name'),
('id3:artist'),
('id3:album'),
('id3:genre'),
('id3:comment'),
('net:track_name'),
('net:artist'),
('net:album'),
('net:genre'),
('filesystem:album'),
('filesystem:artist'),
('filesystem:track_name'),
('filesystem:extension');

create table library.artifact_tags (
    id serial primary key,
    artifact_id integer not null,
    tag_type_id integer not null,
    tag_value varchar(256) not null,
    constraint fk_artifact_id_filesystem_artifacts
        foreign key(artifact_id)
            references library.filesystem_artifacts(id),
    constraint fk_tag_type_id_tag_types
        foreign key(tag_type_id)
            references library.tag_types(id),
    unique (artifact_id, tag_type_id)
);

create table library.playlists (
    id serial primary key,
    playlist_name varchar(64) not null unique,
    playlist_desc varchar(256) null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp
);

create table library.playlist_items (
    id serial primary key,
    artifact_id integer not null,
    rank integer not null,
    constraint fk_playlist_item_artifact_id_filesystem_artifacts
        foreign key (artifact_id)
            references library.filesystem_artifacts(id)
);