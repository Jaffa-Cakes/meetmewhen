-- Your SQL goes here

CREATE TABLE public.basic_event
(
    id text NOT NULL,
    name text NOT NULL,
    "when" bytea NOT NULL,
    no_ealier time(0) without time zone NOT NULL,
    no_later time(0) without time zone NOT NULL,
    timezone text NOT NULL,
    created timestamp(0) with time zone NOT NULL DEFAULT now()::timestamp,
    PRIMARY KEY (id)
);