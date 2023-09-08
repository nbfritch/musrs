create table filesystem_artifacts (
    id integer primary key autoincrement,
    relative_path varchar(256) not null,
    file_name varchar(200) not null,
    file_extension varchar(12) not null,
    is_present bit not null,
    first_path_segment varchar(150) null,
    second_path_segment varchar(150) null,
    created_at integer not null,
    updated_at integer
);

create table track_metadata (
    filesystem_artifact_id integer not null primary key,
    artist varchar(200),
    album varchar(200),
    track_name varchar(200),
    genre varchar(40),
    composer varchar(40),
    release_year integer,
    track_number integer,
    duration integer,
    foreign key (filesystem_artifact_id) references filesystem_artifacts(id)
);

create table playlists (
    id integer primary key autoincrement,
    playlist_name varchar(64) not null unique,
    playlist_desc varchar(256) null,
    created_at integer not null,
    updated_at integer
);

create table playlist_items (
    id integer primary key autoincrement,
    artifact_id integer not null,
    rank integer not null,
    foreign key (artifact_id)
        references filesystem_artifacts(id)
);