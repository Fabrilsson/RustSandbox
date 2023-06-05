CREATE TABLE tasks (
    task_id varchar(255) PRIMARY KEY,
    task_type varchar(255) not null,
    task_state varchar(255) not null,
    task_description varchar not null,
    created_on TIMESTAMP NOT NULL,
    completed_on TIMESTAMP NULL
);

create unique index task_id_idx on tasks (task_id);