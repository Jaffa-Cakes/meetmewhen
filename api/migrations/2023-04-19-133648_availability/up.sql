-- Your SQL goes here

CREATE TABLE public.availability
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY,
    basic_event text NOT NULL,
    name text NOT NULL,
    availabilities bytea NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (basic_event)
        REFERENCES public.basic_event (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);